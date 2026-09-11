use actix_web::web::{Data, Form};
use actix_web::{HttpResponse, Responder, get, post};
use log::{debug, info};

use frontend::endpoints::powerlifters::build_table;
use frontend::endpoints::root_page;
use search::SearchResult;
use types::filters::{PowerliftersQueryDto, QueryDto, TopPowerliftersQueryDto};
use types::prelude::ExportRow;

use crate::server::ServerData;

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok().body(root_page())
}

#[post("/v1/powerlifters")]
pub async fn powerlifters(
    form: Form<PowerliftersQueryDto>,
    data: Data<ServerData>,
) -> impl Responder {
    debug!("form: {form:?}");
    let powerlifter_data: Vec<ExportRow> = search(form.0.into(), data).await;
    info!("result count: {}", powerlifter_data.len());

    HttpResponse::Ok().body(build_table(powerlifter_data))
}

#[post("/v1/top_powerlifters")]
pub async fn top_powerlifters(
    form: Form<TopPowerliftersQueryDto>,
    data: Data<ServerData>,
) -> impl Responder {
    debug!("form: {form:?}");
    let powerlifter_data: Vec<ExportRow> = search(form.0.into(), data).await;
    info!("result count: {}", powerlifter_data.len());

    HttpResponse::Ok().body(build_table(powerlifter_data))
}

async fn search(form: QueryDto, data: Data<ServerData>) -> Vec<ExportRow> {
    let powerlifter_data: Vec<SearchResult> = data.search_engine.search(&form.into()).await;

    powerlifter_data.into_iter().map(ExportRow::from).collect()
}
