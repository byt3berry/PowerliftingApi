use actix_web::dev::Server;
use anyhow::Result;
use clap::Parser;
use db::{LifterDatabase, MeetDatabase};
use log::info;
use cli::Args;

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

    let meet_database: MeetDatabase = MeetDatabase::from_folder(&args.path)?;
    let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
    let data: ServerData = ServerData {
        lifter_database,
    };
    let server: Server = start_server(args.ip, args.port, data)?;

    server.await?;

    info!("Server exited cleanly");
    Ok(())
}
