use actix_web::{post, HttpResponse, Responder};
use actix_web::web::{Data, Form};
use log::debug;
use maud::{html, Markup};
use serde::Deserialize;

use crate::data_fetching::types::lifter::Lifter;
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
    let powerlifter_data: Vec<(String, Option<Lifter>)> = data.database.search_many(&form.0);
    HttpResponse::Ok().body(build_table(powerlifter_data))
}


fn build_table(data: Vec<(String, Option<Lifter>)>) -> Markup {
    html! {
        table {

            tr {
                @for header in &POWERLIFTER_TABLE_HEADERS {
                    th { (header) }
                }
            }

            @for (name, result) in data {
                tr {
                    td { }

                    @if let Some(lifter) = result {
                        td { (lifter.name) }
                        td { "FFForce" }
                        td { (lifter.sex) }

                        @if let Some(age) = lifter.best_meet.age {
                            td { (age) }
                        } @else {
                            td { "?" }
                        }

                        td { (lifter.equipment) }

                        @if let Some(weight_class) = lifter.best_meet.weight_class {
                            td { (weight_class) }
                        } @else {
                            td { "?" }
                        }

                        td { (lifter.best_meet.bodyweight) }

                        @if let Some(squat) = lifter.best_meet.best3squat {
                            td { (squat) }
                        } @else {
                            td { "?" }
                        }

                        @if let Some(bench) = lifter.best_meet.best3bench {
                            td { (bench) }
                        } @else {
                            td { "?" }
                        }

                        @if let Some(deadlift) = lifter.best_meet.best3deadlift {
                            td { (deadlift) }
                        } @else {
                            td { "?" }
                        }

                        td { (lifter.best_meet.total) }
                    } @else {
                        td { (name) }

                        @for _ in 0..&POWERLIFTER_TABLE_HEADERS.len()-2 {
                            td { "?" }
                        }
                    }
                }
            }

        }
    }
}
