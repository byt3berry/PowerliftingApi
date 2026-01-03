use types::filters::QueryDto;

use crate::filters::{DivisionFilter, EquipmentFilter, FederationFilter, PowerlifterFilter, SexFilter};

#[derive(Debug)]
pub struct QueryFilter {
    pub federation_choice: FederationFilter,
    pub equipment_choice: EquipmentFilter,
    pub sex_choice: SexFilter,
    pub division_choice: DivisionFilter,
    pub powerlifters: PowerlifterFilter,
}

impl From<QueryDto> for QueryFilter {
    fn from(value: QueryDto) -> Self {
        Self {
            federation_choice: value.federation_choice.into(),
            equipment_choice: value.equipment_choice.into(),
            sex_choice: value.sex_choice.into(),
            division_choice: value.division_choice.into(),
            powerlifters: value.powerlifters.into(),
        }
    }
}
