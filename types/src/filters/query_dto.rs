use serde::Deserialize;

use crate::filters::{DivisionFilterDto, EquipmentFilterDto, FederationFilterDto, SexFilterDto};

#[derive(Debug, Deserialize)]
pub struct QueryDto {
    pub federation_choice: FederationFilterDto,
    pub equipment_choice: EquipmentFilterDto,
    pub sex_choice: SexFilterDto,
    pub division_choice: DivisionFilterDto,
    pub powerlifters: String,
}
