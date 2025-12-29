use anyhow::{Context, Result};
use dotenvy::dotenv;
use sea_orm::ConnectOptions;
use std::env;
use std::time::Duration;
use tracing::info;

use crate::{ReadOnlyRepository, WriteOnlyRepository};

const DEFAULT_SCHEMA: &str = "public";
const DEFAULT_TABLE: &str = "powerlifting_api_data";

pub struct Repository;

impl Repository {
    fn build_connection_string() -> Result<String> {
        let host: String = env::var("DATABASE_HOST").context("DATABASE_HOST must be set")?;
        let username: String = env::var("DATABASE_USERNAME").context("DATABASE_USERNAME must be set")?;
        let password: String = env::var("DATABASE_PASSWORD").context("DATABASE_PASSWORD must be set")?;

        Ok(format!("postgres://{username}:{password}@{host}/{DEFAULT_TABLE}"))
    }

    fn build_connection_options() -> Result<ConnectOptions> {
        dotenv()?;
        let connection_string = Self::build_connection_string()?;
        let database_schema = env::var("DATABASE_SCHEMA").unwrap_or_else(|_| DEFAULT_SCHEMA.to_string());

        info!("Repository setting up for {}", connection_string);
        let mut connection: ConnectOptions = ConnectOptions::new(connection_string);
        connection.set_schema_search_path(database_schema)
                  .acquire_timeout(Duration::from_secs(3))
                  .test_before_acquire(true)
                  .connect_lazy(true)
                  .sqlx_logging(false);

        Ok(connection)
    }

    pub fn write_only() -> Result<WriteOnlyRepository> {
        let options: ConnectOptions = Self::build_connection_options()?;

        Ok(WriteOnlyRepository::new(options))
    }

    pub fn read_only() -> Result<ReadOnlyRepository> {
        let options: ConnectOptions = Self::build_connection_options()?;

        Ok(ReadOnlyRepository::new(options))
    }
}
