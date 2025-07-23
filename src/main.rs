use actix_web::dev::Server;
use clap::Parser;
use log::info;

use cli::Args;
use crate::server::start_server;

mod api;
mod cli;
mod data_fetching;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enables debug infos
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let args: Args = Args::parse();
    let server: Server = start_server(&args.ip, args.port)?;

    server.await?;

    info!("Server exited cleanly");
    Ok(())
}
