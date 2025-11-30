use anyhow::{bail, Context, Error, Result};
use migrations::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait, TransactionTrait};
use tracing::info;
use types::prelude::*;

use crate::models::types::{Entry, Meet};
use crate::models::{SeaActiveEntry, SeaActiveMeet, SeaColumnEntry, SeaEntityEntry, SeaEntityMeet};

pub struct WriteOnlyRepository {
    options: ConnectOptions,
    connection: Option<DatabaseConnection>,
}

impl WriteOnlyRepository {
    pub(crate) fn new(options: ConnectOptions) -> Result<Self> {
        Ok(Self {
            options,
            connection: None,
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        match Database::connect(self.options.clone()).await {
            Ok(connection) => {
                self.connection = Some(connection);
                Ok(())
            },
            Err(e) => Err(e).context("failed to connect to database"),
        }
    }

    pub async fn disconnect(self) -> Result<()> {
        if let Some(connection) = self.connection {
            connection.close().await?;
        }

        Ok(())
    }

    pub async fn refresh_migrations(&self)-> Result<()> {
        match self.connection {
            Some(ref connection) => Migrator::refresh(connection).await.context("failed to refresh migrations"),
            None => bail!("Can't apply migrations without connecting to the database"),
        }
    }

    pub async fn insert_meet(&mut self, meet: MeetDto) -> Result<()> {
        let meet_name: String = meet.data.name.clone();
        info!("Inserting meet {}", meet_name);

        let Some(ref connection) = self.connection else {
            bail!("Can't insert meet without connecting to the database")
        };

        connection.transaction::<_, (), Error>(|connection| {
            Box::pin(async move {
                let new_meet: SeaActiveMeet = Meet::from(meet.data).into();
                let inserted_id: Option<i32> = SeaEntityMeet::insert(new_meet)
                    .exec_with_returning_keys(connection)
                    .await?
                    .first()
                    .cloned();

                if let Some(inserted_id) = inserted_id {
                    for entry in meet.entries {
                        let mut new_entry: SeaActiveEntry = Entry::from(entry).into();
                        new_entry.set(SeaColumnEntry::MeetId, inserted_id.into());
                        SeaEntityEntry::insert(new_entry)
                            .exec(connection)
                            .await?;
                    }
                }


                Ok(())
            })
        })
        .await
        .context(format!("failed to insert meet {}", meet_name))
    }
}
