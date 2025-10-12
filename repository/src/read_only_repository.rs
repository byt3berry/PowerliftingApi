use anyhow::Result;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::env;

pub struct ReadOnlyRepository {
    connection: PgConnection,
}

impl ReadOnlyRepository {
    pub fn new() -> Result<Self> {
        dotenv()?;
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection: PgConnection = PgConnection::establish(&database_url)?;

        Ok(Self {
            connection,
        })
    }
}
