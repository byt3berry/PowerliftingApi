use anyhow::{Result};
use dotenvy::dotenv;
use migrations::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, DbErr, EntityTrait, TransactionError, TransactionTrait};
use std::env;
use tracing::info;
use types::{EntryDto, MeetDto};

use crate::models::types::entry::Entry;
use crate::models::types::meet::Meet;
use crate::models::{SeaActiveEntry, SeaActiveMeet, SeaColumnEntry, SeaEntityEntry, SeaEntityMeet};

const DEFAULT_SCHEMA: &str = "public";

pub struct WriteOnlyRepository {
    connection: DatabaseConnection,
}

impl WriteOnlyRepository {
    fn build_connection_options() -> Result<ConnectOptions> {
        dotenv()?;
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_schema = env::var("DATABASE_SCHEMA").unwrap_or_else(|_| DEFAULT_SCHEMA.to_string());
        let mut connection: ConnectOptions = ConnectOptions::new(database_url);
        connection.set_schema_search_path(database_schema)
                  .sqlx_logging(false);

        Ok(connection)
    }

    pub async fn new() -> Result<Self> {
        let options: ConnectOptions = Self::build_connection_options()?;
        let connection: DatabaseConnection = Database::connect(options).await?;

        Ok(Self {
            connection,
        })
    }

    pub async fn close(self) -> Result<(), DbErr> {
        self.connection.close().await
    }

    pub async fn refresh_migrations(&self)-> Result<(), DbErr> {
        Migrator::fresh(&self.connection).await?;
        Ok(())
    }

    pub async fn insert_meet_with_posts(&mut self, meet: MeetDto, entries: Vec<EntryDto>) -> Result<(), TransactionError<DbErr>> {
        info!("Inserting meet {}", meet.name);
        let new_meet: SeaActiveMeet = Meet::from(meet).into();

        self.connection.transaction(|connection| {
            Box::pin(async move {
                let inserted_id: Option<i32> = SeaEntityMeet::insert(new_meet)
                    .exec_with_returning_keys(connection)
                    .await?
                    .first()
                    .cloned();

                if let Some(inserted_id) = inserted_id {
                    for entry in entries {
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
    }
}
