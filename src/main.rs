use actix_web::dev::{Server, ServerHandle};
use clap::Parser;
use log::info;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Notify;

use cli::Args;
use crate::server::{handle_sigint_signal, handle_sighup_signal, start_server};

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
    let exit_flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let restart_notifier: Arc<Notify> = Arc::new(Notify::new());

    loop {
        let server: Server = start_server(&args.ip, args.port)?;
        let handle: ServerHandle = server.handle();

        handle_sigint_signal(&handle, &exit_flag);
        handle_sighup_signal(&handle, &restart_notifier);

        server.await?;

        if exit_flag.load(Ordering::SeqCst) {
            break;
        } else {
            restart_notifier.notified().await;
            continue;
        }
    }

    info!("Server exited cleanly");
    Ok(())
}
