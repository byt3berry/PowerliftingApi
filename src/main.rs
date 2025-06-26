use actix_htmx::HtmxMiddleware;
use actix_web::{App, HttpServer};
use api::powerlifters::powerlifters;
use api::root::root;

mod api;
mod data_fetching;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(HtmxMiddleware)
            .service(root)
            .service(powerlifters)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
