use actix_web::dev::Server;
use anyhow::Result;
use clap::Parser;
use cli::Args;
use data_parsing::Database;
use log::info;

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

    Database::from_directory(&args.path)?.save().await?;

    // let data: ServerData = ServerData {
    //     database,
    // };
    // let server: Server = start_server(args.ip, args.port, data)?;

    // server.await?;

    info!("Server exited cleanly");
    Ok(())
}
