use anyhow::{bail, Context, Error, Result};
use dotenvy::dotenv;
use migrations::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait, TransactionTrait};
use std::env;
use std::time::Duration;
use tracing::info;
use types::prelude::*;

use crate::models::types::{Entry, Meet};
use crate::models::{SeaActiveEntry, SeaActiveMeet, SeaColumnEntry, SeaEntityEntry, SeaEntityMeet};
use crate::repository_trait::Repository;

const DEFAULT_SCHEMA: &str = "public";
const DEFAULT_TABLE: &str = "powerlifting_api_data";

pub struct WriteOnlyRepository {
    options: ConnectOptions,
    connection: Option<DatabaseConnection>,
}

impl WriteOnlyRepository {
    fn build_connection_string() -> Result<String> {
        let host: String = env::var("DATABASE_HOST").context("DATABASE_HOST must be set")?;
        let username: String = env::var("DATABASE_USERNAME").context("DATABASE_USERNAME must be set")?;
        let password: String = env::var("DATABASE_PASSWORD").context("DATABASE_PASSWORD must be set")?;

        Ok(format!("postgres://{}:{}@{}/{}", username, password, host, DEFAULT_TABLE))
    }

    fn build_connection_options() -> Result<ConnectOptions> {
        dotenv()?;
        let connection_string = Self::build_connection_string()?;
        let database_schema = env::var("DATABASE_SCHEMA").unwrap_or_else(|_| DEFAULT_SCHEMA.to_string());

        info!("WriteOnly Repository setting up for {}", connection_string);
        let mut connection: ConnectOptions = ConnectOptions::new(connection_string);
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

impl Repository for WriteOnlyRepository {
    async fn connect(&mut self) -> Result<()> {
        match Database::connect(self.options.clone()).await {
            Ok(connection) => {
                self.connection = Some(connection);
                Ok(())
            },
            Err(e) => Err(e).context("failed to connect to database"),
        }
    }

    async fn disconnect(self) -> Result<()> {
        if let Some(connection) = self.connection {
            connection.close().await?;
        }

        Ok(())
    }
}
