use actix_web::dev::Server;
use anyhow::Result;
use clap::Parser;
use cli::Args;
use db::{Database, Meet, MeetData, MeetEntry};
use log::info;
use migration::Migrator;
use repository::{write_only_repository::WriteOnlyRepository};

use crate::server::{start_server, ServerData};

mod api;
mod cli;
mod server;

pub const POWERLIFTER_TABLE_HEADERS: [&str; 12] = [
    "Rank", 
    "Lifter", 
    "Federation", 
    "Division", 
    "Sex", 
    "Equipment", 
    "Class", 
    "Weight", 
    "Squat", 
    "Bench", 
    "Deadlift", 
    "Total",
];

fn from_meet(meet: &MeetData) -> types::Meet {
    types::Meet { 
        name: meet.name.clone(),
        federation: meet.federation,
        country: meet.country,
        state: meet.state.clone(),
        town: meet.town.clone(),
    }
}

fn from_entry(entry: &MeetEntry) -> types::Entry {
    types::Entry {
        name: entry.name.clone(),
        division: entry.division.into(),
        equipment: entry.equipment.into(),
        sex: entry.sex.into(),
        bodyweight: types::Weight(15.),
        weight_class: None,
        squat1: None,
        squat2: None,
        squat3: None,
        squat4: None,
        bench1: None,
        bench2: None,
        bench3: None,
        bench4: None,
        deadlift1: None,
        deadlift2: None,
        deadlift3: None,
        deadlift4: None,
        best_squat: None,
        best_bench: None,
        best_deadlift: None,
        total: types::Weight(4.),
    }
}

fn from_entries(entries: Vec<MeetEntry>) -> Vec<types::Entry> {
    entries
        .iter()
        .map(from_entry)
        .collect()
}

#[actix_web::main]
async fn main() -> Result<()> {
    // Enables debug infos
    unsafe {
        std::env::set_var("RUST_LOG", "info");
        #[cfg(debug_assertions)]
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let args: Args = Args::parse();
    args.validate()?;

    let mut write_only_repository = WriteOnlyRepository::new().await?;
    let database: Database = Database::from_directory(&args.path)?;

    write_only_repository.refresh_migrations().await?;

    for meet in database.meets.iter() {
        let meet_dto: types::Meet = from_meet(&meet.data);
        let entries_dto: Vec<types::Entry> = meet
            .entries
            .iter()
            .map(from_entry)
            .collect();

        write_only_repository.insert_meet_with_posts(meet_dto, entries_dto).await?;
    }

    write_only_repository.close().await?;

    // let data: ServerData = ServerData {
    //     database,
    // };
    // let server: Server = start_server(args.ip, args.port, data)?;

    // server.await?;

    info!("Server exited cleanly");
    Ok(())
}
