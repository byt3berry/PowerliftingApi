use anyhow::{Context, Result};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use types::filters::Query;
use types::prelude::EntryDto;

pub struct ReadOnlyRepository {
    options: ConnectOptions,
    connection: Option<DatabaseConnection>,
}

impl ReadOnlyRepository {
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

    pub async fn search(&self, query: &Query) -> Result<EntryDto> {
        todo!();
    }
}
