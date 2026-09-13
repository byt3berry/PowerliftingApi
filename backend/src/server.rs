use actix_files::Files;
use actix_htmx::HtmxMiddleware;
use actix_web::dev::Server;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{App, HttpResponse, HttpServer, web};
use anyhow::Result;
use log::info;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use search::SearchEngine;

use crate::api_doc::ApiDoc;
use crate::endpoints;

#[derive(Clone, Debug)]
pub struct ServerData {
    pub ip: IpAddr,
    pub port: u16,
    pub search_engine: SearchEngine,
    pub static_data: PathBuf,
}

/// Start a server listening on `ip`:`port`
pub fn start_server(data: ServerData) -> Result<Server> {
    info!("Starting server on {}:{}", &data.ip, &data.port);
    let address: SocketAddr = SocketAddr::new(data.ip, data.port);

    Ok(HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(HtmxMiddleware)
            .wrap(Logger::new("[%s] %U"))
            .app_data(web::Data::new(data.clone()))
            .service(web::scope("/api").configure(endpoints::config))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(Files::new("/", data.static_data.clone()).index_file("index.html"))
            .default_service(web::route().to(HttpResponse::ImATeapot))
    })
    .workers(1)
    .bind(address)?
    .run())
}
