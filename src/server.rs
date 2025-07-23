use actix_htmx::HtmxMiddleware;
use actix_web::dev::{Server, ServerHandle};
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::rt::signal::ctrl_c;
use actix_web::rt::signal::unix::{signal, SignalKind};
use actix_web::{web, App, HttpResponse, HttpServer};
use chrono::{DateTime, Local, NaiveDateTime, NaiveTime};
use log::{info, warn};
use tokio::task::JoinHandle;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::signal::unix::Signal;
use tokio::sync::Notify;
use tokio::time::{sleep, Duration};

use crate::api::powerlifters::powerlifters;
use crate::api::root::root;

/// Start a server listening on `ip`:`port`
pub fn start_server(ip: &str, port: u16) -> std::io::Result<Server>{
    info!("Starting server on {ip}:{port}");

    Ok(
        HttpServer::new(|| {
            App::new()
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .wrap(HtmxMiddleware)
                .wrap(Logger::new("[%s] %U"))
                .service(root)
                .service(powerlifters)
                .default_service(
                    web::route().to(HttpResponse::ImATeapot),
                )
        })
        .workers(1)
        .bind((ip, port))?
        .run()
    )
}

/// Restart the server every day at 00:10
async fn restart_server(handler: &ServerHandle, restart_flag: &Arc<AtomicBool>) {
    let now: DateTime<Local>= Local::now();
    let next_run = NaiveTime::from_hms_nano_opt(10, 57, 0, 0).unwrap();
    let duration_until_next = next_run - now.time();
    info!("time to wait: {:?}", Duration::from_secs(duration_until_next.num_seconds() as u64));

    sleep(Duration::from_secs(duration_until_next.num_seconds() as u64)).await;
    handler.stop(true).await;
    restart_flag.store(true, Ordering::SeqCst);
    // handle_sigusr1_signal(handler, notifier);
}

/// Catch signal SIGINT to trigger the shut down of the server
pub fn handle_sigint_signal(handler: &ServerHandle, exit_flag: &Arc<AtomicBool>) {
    let handler_clone: ServerHandle = handler.clone();
    let flag_clone: Arc<AtomicBool> = exit_flag.clone();

    tokio::spawn(async move {
        ctrl_c().await.expect("failed to listen for ctrl-c");
        warn!("SIGINT received: Shutting down server");
        flag_clone.store(true, Ordering::SeqCst);
        handler_clone.stop(true).await;
    });
}

/// Create a task that restart the server
pub async fn schedule_daily_reload(handler: &ServerHandle, restart_flag: &Arc<AtomicBool>) -> JoinHandle<()> {
    // Wait 10 seconds before the first restart is enabled
    sleep(Duration::from_secs(10)).await;

    let handler_clone: ServerHandle = handler.clone();
    let flag_clone: Arc<AtomicBool> = restart_flag.clone();

    tokio::spawn(async move {
        restart_server(&handler_clone, &flag_clone).await;
    })
}
