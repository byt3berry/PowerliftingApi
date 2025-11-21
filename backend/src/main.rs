use actix_web::dev::Server;
use anyhow::Result;
use clap::Parser;
use cli::Args;
use data_parsing::Database;
use log::info;
use repository::write_only_repository::WriteOnlyRepository;
use types::{EntryDto, MeetDto};

use crate::server::{start_server, ServerData};

mod api;
mod cli;
mod server;

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

    for meet in database.iter() {
        let meet_dto: MeetDto = meet.data.clone().into();
        let entries_dto: Vec<EntryDto> = meet
        .entries
        .iter()
        .map(|entry| EntryDto::from(entry.clone()))
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
