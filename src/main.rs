use actix_web::dev::{Server, ServerHandle};
use clap::Parser;
use log::info;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Notify;

use cli::Args;
use crate::server::{handle_sigint_signal, handle_sigusr1_signal, schedule_daily_reload, start_server};

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
        let handler: ServerHandle = server.handle();

        let scheduled_task = schedule_daily_reload(&handler, &restart_notifier);
        handle_sigint_signal(&handler, &exit_flag);
        handle_sigusr1_signal(&handler, &restart_notifier);

        server.await?;

        if exit_flag.load(Ordering::SeqCst) {
            break;
        } else {
            scheduled_task.await.abort();
            restart_notifier.notified().await;
            continue;
        }
    }

    info!("Server exited cleanly");
    Ok(())
}
