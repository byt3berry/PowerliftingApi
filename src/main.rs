use clap::Parser;

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

    start_server(args.ip, args.port)?
    .await
}
