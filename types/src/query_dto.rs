use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub federation_choice: FederationDto,
    pub equipment_choice: EquipmentDto,
    pub sex_choice: SexDto,
    pub division_choice: DivisionDto,
    pub powerlifters: String,
}
