use actix_web::dev::Server;
use anyhow::Result;
use clap::Parser;
use log::info;

use cli::Args;
use crate::data_fetching::entries::lifter_database::LifterDatabase;
use crate::server::start_server;

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
    let lifter_database: LifterDatabase = LifterDatabase::from_folder(&args.path)?;
    let server: Server = start_server(args.ip, args.port, lifter_database)?;

    server.await?;

    info!("Server exited cleanly");
    Ok(())
}
