use actix_web::{post, HttpResponse, Responder};
use actix_web::web::{Data, Form};
use log::debug;
use maud::{html, Markup};
use serde::Deserialize;

use crate::data_fetching::entries::export_row::ExportRow;
use crate::data_fetching::POWERLIFTER_TABLE_HEADERS;
use crate::data_fetching::types::division::Division;
use crate::data_fetching::types::equipment::Equipment;
use crate::data_fetching::types::sex::Sex;
use crate::server::ServerData;

#[derive(Debug, Deserialize)]
pub struct PowerlifterForm {
    pub equipment_choice: Equipment,
    pub sex_choice: Sex,
    pub division_choice: Division,
    pub powerlifters: String,
}

#[post("/powerlifters")]
pub async fn powerlifters(form: Form<PowerlifterForm>, data: Data<ServerData>) -> impl Responder {
    debug!("form: {form:?}");
    let powerlifter_data: Vec<ExportRow> = data.lifter_database.search_export(&form.0).into();
    HttpResponse::Ok().body(build_table(powerlifter_data))
}

fn build_table(data: Vec<ExportRow>) -> Markup {
    html! {
        table {

            tr {
                @for header in &POWERLIFTER_TABLE_HEADERS {
                    th { (header) }
                }
            }

            @for row in data {
                tr {
                    td { (row.rank) }
                    td { (row.name) }
                    td { "FFForce" }
                    td { (row.division) }
                    td { (row.sex) }
                    td { (row.equipment) }
                    td { (row.weight_class) }
                    td { (row.bodyweight) }
                    td { (row.best_squat) }
                    td { (row.best_bench) }
                    td { (row.best_deadlift) }
                    td { (row.total) }
                }
            }

        }
    }
}
