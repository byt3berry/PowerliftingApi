use serde::Deserialize;
use utoipa::ToSchema;

use crate::endpoints::filters::{DivisionFilter, EquipmentFilter, FederationFilter, SexFilter};

use types::filters::QueryDto;

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct PowerliftersQuery {
    pub federation_choice: FederationFilter,
    pub equipment_choice: EquipmentFilter,
    pub sex_choice: SexFilter,
    pub division_choice: DivisionFilter,
    pub powerlifters: String,
}

impl From<PowerliftersQuery> for QueryDto {
    fn from(value: PowerliftersQuery) -> Self {
        Self {
            federation_choice: value.federation_choice.into(),
            equipment_choice: value.equipment_choice.into(),
            sex_choice: value.sex_choice.into(),
            division_choice: value.division_choice.into(),
            powerlifters: value.powerlifters.into(),
            limit: 0,
        }
    }
}
