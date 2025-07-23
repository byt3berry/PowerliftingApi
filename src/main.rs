use actix_web::dev::Server;
use clap::Parser;
use log::info;

use cli::Args;
use crate::data_fetching::entries::meet_database::MeetDatabase;
use crate::data_fetching::types::meet_entry::MeetEntry;
use crate::{server::start_server};

mod api;
mod cli;
mod data_fetching;
mod server;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Enables debug infos
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let args: Args = Args::parse();
    args.validate()?;
    let meet_entries: Vec<MeetEntry> = MeetDatabase::from_folder(&args.path)?;
    let server: Server = start_server(args.ip, args.port, meet_entries)?;

    server.await?;

    info!("Server exited cleanly");
    Ok(())
}
