use actix_web::dev::Server;
use anyhow::Result;
use clap::Parser;
use log::info;

use cli::Args;
use crate::data_fetching::entries::lifter_database::LifterDatabase;
use crate::data_fetching::entries::meet_database::MeetDatabase;
use crate::server::{start_server, ServerData};

mod api;
mod cli;
mod data_fetching;
mod server;

#[actix_web::main]
async fn main() -> Result<()> {
    // Enables debug infos
    std::env::set_var("RUST_LOG", "info");
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "debug");
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
