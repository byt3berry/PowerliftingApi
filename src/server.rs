use actix_htmx::HtmxMiddleware;
use actix_web::dev::{Server, ServerHandle};
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::rt::signal::ctrl_c;
use actix_web::rt::signal::unix::{signal, SignalKind};
use actix_web::{web, App, HttpResponse, HttpServer};
use log::{info, warn};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Notify;

use crate::api::powerlifters::powerlifters;
use crate::api::root::root;

pub fn start_server(ip: &str, port: u16) -> std::io::Result<Server>{
    info!("Starting server on {ip}:{port}");

    Ok(
        HttpServer::new(|| {
            App::new()
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .wrap(HtmxMiddleware)
                .wrap(Logger::new("%a [%s] %U"))
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

pub fn handle_sigint_signal(handle: &ServerHandle, exit_flag: &Arc<AtomicBool>) {
    let handler_clone: ServerHandle = handle.clone();
    let flag_clone: Arc<AtomicBool> = exit_flag.clone();

    tokio::spawn(async move {
        ctrl_c().await.expect("failed to listen for ctrl-c");
        warn!("SIGINT received: Shutting down server");
        flag_clone.store(true, Ordering::SeqCst);
        handler_clone.stop(true).await;
    });
}

pub fn handle_sighup_signal(handle: &ServerHandle, notifier: &Arc<Notify>) {
    let handler_clone: ServerHandle = handle.clone();
    let notifier_clone = notifier.clone();

    tokio::spawn(async move {
        let mut sig = signal(SignalKind::hangup()).unwrap();
        sig.recv().await;
        warn!("SIGHUP received: Restarting server");
        handler_clone.stop(true).await;
        notifier_clone.notify_one();
    });
}
