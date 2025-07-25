use actix_htmx::HtmxMiddleware;
use actix_web::dev::Server;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::Result;
use log::info;
use std::net::IpAddr;

use crate::api::powerlifters::powerlifters;
use crate::api::root::root;
use crate::data_fetching::entries::lifter_database::LifterDatabase;

#[derive(Debug)]
pub struct ServerData {
    pub database: LifterDatabase,
}

/// Start a server listening on `ip`:`port`
pub fn start_server(ip: IpAddr, port: u16, lifter_database: LifterDatabase) -> Result<Server>{
    info!("Starting server on {ip}:{port}");

    Ok(
        HttpServer::new(move || {
            App::new()
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .wrap(HtmxMiddleware)
                .wrap(Logger::new("[%s] %U"))
                .app_data(web::Data::new(ServerData { database: lifter_database.clone() }))
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
