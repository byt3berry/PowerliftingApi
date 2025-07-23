use std::sync::Arc;

use actix_htmx::HtmxMiddleware;
use actix_web::dev::Server;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpResponse, HttpServer};
use log::info;

use crate::api::powerlifters::powerlifters;
use crate::api::root::root;
use crate::data_fetching::types::meet_entry::MeetEntry;

struct ServerData {
    meet_entries: Vec<MeetEntry>,
}

/// Start a server listening on `ip`:`port`
pub fn start_server(ip: &str, port: u16, meet_entries: Vec<MeetEntry>) -> std::io::Result<Server>{
    info!("Starting server on {ip}:{port}");

    Ok(
        HttpServer::new(move || {
            App::new()
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .wrap(HtmxMiddleware)
                .wrap(Logger::new("[%s] %U"))
                .app_data(web::Data::new(ServerData { meet_entries: meet_entries.clone() }))
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
