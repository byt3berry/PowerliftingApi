use actix_htmx::HtmxMiddleware;
use actix_web::dev::Server;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{App, HttpResponse, HttpServer, web};
use anyhow::Result;
use log::info;
use std::net::IpAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use search::SearchEngine;

use crate::api_doc::ApiDoc;
use crate::endpoints;

#[derive(Clone, Debug)]
pub struct ServerData {
    pub search_engine: SearchEngine,
}

/// Start a server listening on `ip`:`port`
pub fn start_server(ip: IpAddr, port: u16, data: ServerData) -> Result<Server> {
    info!("Starting server on {ip}:{port}");

    Ok(HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(HtmxMiddleware)
            .wrap(Logger::new("[%s] %U"))
            .app_data(web::Data::new(data.clone()))
            .configure(endpoints::config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .default_service(web::route().to(HttpResponse::ImATeapot))
    })
    .workers(1)
    .bind((ip, port))?
    .run())
}
