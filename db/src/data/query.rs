use serde::Deserialize;
use types::{Division, Equipment, Sex};

#[derive(Debug, Deserialize)]
pub struct Query {
    pub equipment_choice: Equipment,
    pub sex_choice: Sex,
    pub division_choice: Division,
    pub powerlifters: String,
}
