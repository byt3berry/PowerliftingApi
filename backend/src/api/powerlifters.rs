use actix_web::web::{Data, Form};
use actix_web::{post, HttpResponse, Responder};
use frontend::api::powerlifters::build_table;
use log::debug;
use types::filters::QueryDto;
use types::prelude::*;

use crate::server::ServerData;

#[post("/powerlifters")]
pub async fn powerlifters(form: Form<QueryDto>, data: Data<ServerData>) -> impl Responder {
    debug!("form: {form:?}");
    let powerlifter_data: Vec<ExportRow> = vec![];
    HttpResponse::Ok().body(build_table(powerlifter_data))
}
