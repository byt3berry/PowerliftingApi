use actix_htmx::HtmxMiddleware;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpResponse, HttpServer};
use clap::Parser;

use api::powerlifters::powerlifters;
use api::root::root;
use cli::Args;

mod api;
mod cli;
mod data_fetching;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enables debug infos
    #[cfg(debug_assertions)] std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let args: Args = Args::parse();

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
    .bind(("localhost", args.port))?
    .run()
    .await
}
