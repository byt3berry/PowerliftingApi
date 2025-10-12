use anyhow::Result;
use diesel::RunQueryDsl;
use diesel::result::Error;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::env;
use types::{EntryDto, MeetDto};

use crate::schema::entries;
use crate::schema::meets;

use crate::models::create::new_entry::NewEntry;
use crate::models::create::new_meet::NewMeet;
use crate::models::read::meet::Meet;

pub struct WriteOnlyRepository {
    connection: PgConnection,
}

impl WriteOnlyRepository {
    pub fn new() -> Result<Self> {
        dotenv()?;
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection: PgConnection = PgConnection::establish(&database_url)?;

        Ok(Self {
            connection,
        })
    }

    pub fn create_meet_with_posts(&mut self, meet: MeetDto, entries: Vec<EntryDto>) -> Result<(), Error> {
        let new_meet: NewMeet = NewMeet::from(&meet);

        self.connection.transaction(|connection| {
            let meet: Meet = diesel::insert_into(meets::table)
                .values(&new_meet)
                .get_result(connection)?;

            let new_entries: Vec<NewEntry> = entries
                .iter()
                .map(|entry| NewEntry::from(meet.id, entry))
                .collect();

            diesel::insert_into(entries::table)
                .values(&new_entries)
                .execute(connection)?;

            Ok(())
        })
    }

    pub fn clean(&mut self) -> Result<(), Error> {
        self.connection.transaction(|connection| {
            diesel::delete(meets::table).execute(connection)?;
            diesel::delete(entries::table).execute(connection)?;

            Ok(())
        })
    }
}
