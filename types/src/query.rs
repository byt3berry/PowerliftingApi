use serde::Deserialize;

use crate::{Division, Equipment, Federation, Sex};

#[derive(Debug, Deserialize)]
pub struct Query {
    pub federation_choice: Federation,
    pub equipment_choice: Equipment,
    pub sex_choice: Sex,
    pub division_choice: Division,
    pub powerlifters: String,
}
