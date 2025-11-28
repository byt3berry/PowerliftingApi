use maud::{html, Markup};
use types::prelude::*;

pub const POWERLIFTER_TABLE_HEADERS: [&str; 12] = [
    "Rank", 
    "Lifter", 
    "Federation", 
    "Division", 
    "Sex", 
    "Equipment", 
    "Class", 
    "Weight", 
    "Squat", 
    "Bench", 
    "Deadlift", 
    "Total",
];

pub fn build_table(data: Vec<ExportRow>) -> Markup {
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
