use serde::Deserialize;
use strum_macros::Display;
use types::filters::EquipmentFilterDto;
use utoipa::ToSchema;

#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, PartialEq, ToSchema)]
pub enum EquipmentFilter {
    Raw,
    Wraps,
    Single,
    Multi,
    Straps,
    Sleeves,
    Bare,
    Unlimited,
}

impl From<EquipmentFilter> for EquipmentFilterDto {
    fn from(value: EquipmentFilter) -> Self {
        match value {
            EquipmentFilter::Raw => Self::Raw,
            EquipmentFilter::Wraps => Self::Wraps,
            EquipmentFilter::Single => Self::Single,
            EquipmentFilter::Multi => Self::Multi,
            EquipmentFilter::Straps => Self::Straps,
            EquipmentFilter::Sleeves => Self::Sleeves,
            EquipmentFilter::Bare => Self::Bare,
            EquipmentFilter::Unlimited => Self::Unlimited,
        }
    }
}
