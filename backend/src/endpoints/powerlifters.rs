use actix_web::web::{Data, Form};
use actix_web::{post, HttpResponse, Responder};
use log::{debug, info};

use frontend::endpoints::powerlifters::build_table;
use search::SearchResult;
use types::filters::QueryDto;
use types::prelude::ExportRow;

use crate::server::ServerData;

#[post("/powerlifters")]
pub async fn powerlifters(form: Form<QueryDto>, data: Data<ServerData>) -> impl Responder {
    debug!("form: {form:?}");
    let powerlifter_data: Vec<SearchResult> = data.search_engine.search(&form.0).await;
    info!("result count: {}", powerlifter_data.len());

    let rows: Vec<ExportRow> = powerlifter_data
        .into_iter()
        .map(ExportRow::from)
        .collect();

    HttpResponse::Ok().body(build_table(rows))
}
