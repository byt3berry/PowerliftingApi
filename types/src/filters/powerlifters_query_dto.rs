use serde::Deserialize;

use crate::filters::{DivisionFilterDto, EquipmentFilterDto, FederationFilterDto, QueryDto, SexFilterDto};

#[derive(Clone, Debug, Deserialize)]
pub struct PowerliftersQueryDto {
    pub federation_choice: FederationFilterDto,
    pub equipment_choice: EquipmentFilterDto,
    pub sex_choice: SexFilterDto,
    pub division_choice: DivisionFilterDto,
    pub powerlifters: String,
}

impl From<PowerliftersQueryDto> for QueryDto {
    fn from(value: PowerliftersQueryDto) -> Self {
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

