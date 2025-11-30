use crate::filters::{DivisionFilter, EquipmentFilter, FederationFilter, SexFilter};
use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub federation_choice: FederationFilter,
    pub equipment_choice: EquipmentFilter,
    pub sex_choice: SexFilter,
    pub division_choice: DivisionFilter,
    pub powerlifters: String,
}
