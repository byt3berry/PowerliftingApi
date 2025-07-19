use actix_htmx::HtmxMiddleware;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpResponse, HttpServer};

use api::powerlifters::powerlifters;
use api::root::root;
use api::styles::styles;

mod api;
mod data_fetching;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enables debug infos
    #[cfg(debug_assertions)] std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(HtmxMiddleware)
            .wrap(Logger::new("%a [%s] %U"))
            .service(styles)
            .service(root)
            .service(powerlifters)
            .default_service(
                web::route().to(HttpResponse::ImATeapot),
            )
    })
    .workers(1)
    .bind(("localhost", 8080))?
    .run()
    .await
}
