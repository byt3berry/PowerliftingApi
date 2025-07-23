use actix_htmx::HtmxMiddleware;
use actix_web::dev::Server;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpResponse, HttpServer};
use log::info;

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
