use actix_web::web::{Data, Json, ServiceConfig};
use actix_web::{HttpResponse, Responder, post};
use log::{debug, info};

use search::SearchResult;
use types::filters::QueryDto;

use crate::endpoints::powerlifter::Powerlifter;
use crate::endpoints::powerlifters_query::PowerliftersQuery;
use crate::server::ServerData;

mod filters;
pub mod powerlifter;
mod powerlifters_query;

async fn search(form: QueryDto, data: Data<ServerData>) -> Vec<SearchResult> {
    data.search_engine.search(&form.into()).await
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(powerlifters);
}

#[utoipa::path(
    post,
    path = "/powerlifters",
    request_body(
        content = PowerliftersQuery,
        content_type = "application/json"
    ),
    responses(
        (
            status = 200,
            description = "Powerlifters matching the supplied filters",
            body = Vec<Powerlifter>,
            content_type = "application/json"
        ),
        (
            status = 400,
            description = "Invalid form data"
        )
    ),
    tag = "Powerlifters"
)]
#[post("/powerlifters")]
async fn powerlifters(form: Json<PowerliftersQuery>, data: Data<ServerData>) -> impl Responder {
    debug!("form: {form:?}");
    let powerlifter_data: Vec<Powerlifter> = search(form.0.into(), data)
        .await
        .into_iter()
        .map(Powerlifter::from)
        .collect();
    info!("result count: {}", powerlifter_data.len());

    HttpResponse::Ok().json(powerlifter_data)
}
