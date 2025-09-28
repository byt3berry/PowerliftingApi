use actix_web::dev::Server;
use anyhow::Result;
use clap::Parser;
use cli::Args;
use db::Database;
use log::info;

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

    let database: Database = Database::from_directory(&args.path)?;
    let data: ServerData = ServerData {
        database,
    };
    let server: Server = start_server(args.ip, args.port, data)?;

    server.await?;

    info!("Server exited cleanly");
    Ok(())
}
