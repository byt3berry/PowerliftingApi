use diesel::result::Error;
use diesel::{Connection, PgConnection, RunQueryDsl, SelectableHelper};
use types::{EntryDto, MeetDto};

use crate::schema::entries;
use crate::schema::meets;

use crate::models::create::new_entry::NewEntry;
use crate::models::create::new_meet::NewMeet;
use crate::models::read::meet::Meet;

pub fn create_meet(connection: &mut PgConnection, meet: MeetDto, entries: Vec<EntryDto>) -> Result<(), Error> {
    let new_meet: NewMeet = NewMeet::from(&meet);

    connection.transaction::<_, Error, _>(|connection| {
        let result: Meet = diesel::insert_into(meets::table)
            .values(&new_meet)
            .returning(Meet::as_returning())
            .get_result(connection)
            .expect("Error saving new meet");

        create_entries(connection, result.id, entries)
    })
}

fn create_entries(connection: &mut PgConnection, meet_id: i32, entries: Vec<EntryDto>) -> Result<(), Error> {
    let new_entries: Vec<NewEntry> = entries
        .iter()
        .map(|entry| NewEntry::from(meet_id, entry))
        .collect();

    connection.transaction::<_, Error, _>(|connection| {
        for new_entry in new_entries {
            diesel::insert_into(entries::table)
                .values(&new_entry)
                .execute(connection)?;
        }
    
        Ok(())
    })
}
