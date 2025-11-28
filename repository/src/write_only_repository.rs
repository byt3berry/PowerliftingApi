use anyhow::{anyhow, bail, Context, Error, Result};
use dotenvy::dotenv;
use migrations::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, DbErr, EntityTrait, TransactionError, TransactionTrait};
use std::env;
use std::time::Duration;
use tracing::info;
use types::MeetDto;

use crate::models::types::entry::Entry;
use crate::models::types::meet::Meet;
use crate::models::{SeaActiveEntry, SeaActiveMeet, SeaColumnEntry, SeaEntityEntry, SeaEntityMeet};

const DEFAULT_SCHEMA: &str = "public";

pub struct WriteOnlyRepository {
    options: ConnectOptions,
    connection: Option<DatabaseConnection>,
}

impl WriteOnlyRepository {
    fn build_connection_options() -> Result<ConnectOptions> {
        dotenv()?;
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_schema = env::var("DATABASE_SCHEMA").unwrap_or_else(|_| DEFAULT_SCHEMA.to_string());
        let mut connection: ConnectOptions = ConnectOptions::new(database_url);
        connection.set_schema_search_path(database_schema)
                  .acquire_timeout(Duration::from_secs(3))
                  .test_before_acquire(true)
                  .connect_lazy(true)
                  .sqlx_logging(false);

        Ok(connection)
    }

    pub async fn new() -> Result<Self> {
        let options: ConnectOptions = Self::build_connection_options()?;

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

    pub async fn push_migrations(&self)-> Result<()> {
        match self.connection {
            Some(ref connection) => Migrator::up(connection, None).await.context("failed to apply migrations"),
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
                        let mut new_entry: SeaActiveEntry = Entry::from(entry)
                            .into();
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
