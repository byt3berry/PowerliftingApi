use actix_web::dev::Server;
use anyhow::{Result, bail};
use clap::Parser;
use dotenvy::dotenv;
use log::info;

use cli::Args;
use data_parsing::Database;
use search::SearchEngine;

use crate::server::{ServerData, start_server};

mod api_doc;
mod cli;
mod endpoints;
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
    dotenv()?;

    let args: Args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) => bail!(e),
    };
    args.validate()?;

    if args.migrate.is_some_and(|migrate| migrate) {
        Database::from_directory(args.path.as_ref().unwrap())?
            .save()
            .await?;
    }

    if args.start_server.is_some_and(|start_server| start_server) {
        let data: ServerData = ServerData {
            search_engine: SearchEngine,
        };
        let server: Server = start_server(args.ip.unwrap(), args.port.unwrap(), data)?;

        server.await?;

        info!("Server exited cleanly");
    }
    Ok(())
}
