use serde::Deserialize;

use crate::filters::{DivisionFilterDto, EquipmentFilterDto, FederationFilterDto, QueryDto, SexFilterDto};

#[derive(Clone, Debug, Deserialize)]
pub struct TopPowerliftersQueryDto {
    pub federation_choice: FederationFilterDto,
    pub equipment_choice: EquipmentFilterDto,
    pub sex_choice: SexFilterDto,
    pub division_choice: DivisionFilterDto,
    pub limit: u8,
}

impl From<TopPowerliftersQueryDto> for QueryDto {
    fn from(value: TopPowerliftersQueryDto) -> Self {
        Self {
            federation_choice: value.federation_choice.into(),
            equipment_choice: value.equipment_choice.into(),
            sex_choice: value.sex_choice.into(),
            division_choice: value.division_choice.into(),
            powerlifters: String::new(),
            limit: value.limit,
        }
    }
}
