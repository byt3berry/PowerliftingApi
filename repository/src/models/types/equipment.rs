use sea_orm::prelude::{DeriveActiveEnum, EnumIter};
use types::prelude::EquipmentDto;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "equipment")]
pub enum Equipment {
    #[sea_orm(string_value = "raw")]
    Raw,

    #[sea_orm(string_value = "wraps")]
    Wraps,

    #[sea_orm(string_value = "single")]
    Single,

    #[sea_orm(string_value = "multi")]
    Multi,

    #[sea_orm(string_value = "straps")]
    Straps,

    #[sea_orm(string_value = "sleeves")]
    Sleeves,

    #[sea_orm(string_value = "bare")]
    Bare,

    #[sea_orm(string_value = "unlimited")]
    Unlimited,
}

impl From<EquipmentDto> for Equipment {
    fn from(value: EquipmentDto) -> Self {
        match value {
            EquipmentDto::Raw => Self::Raw,
            EquipmentDto::Wraps => Self::Wraps,
            EquipmentDto::Single => Self::Single,
            EquipmentDto::Multi => Self::Multi,
            EquipmentDto::Straps => Self::Straps,
            EquipmentDto::Sleeves => Self::Sleeves,
            EquipmentDto::Bare => Self::Bare,
            EquipmentDto::Unlimited => Self::Unlimited,
        }
    }
}

impl From<Equipment> for EquipmentDto {
    fn from(value: Equipment) -> Self {
        match value {
            Equipment::Raw => Self::Raw,
            Equipment::Wraps => Self::Wraps,
            Equipment::Single => Self::Single,
            Equipment::Multi => Self::Multi,
            Equipment::Straps => Self::Straps,
            Equipment::Sleeves => Self::Sleeves,
            Equipment::Bare => Self::Bare,
            Equipment::Unlimited => Self::Unlimited,
        }
    }
}
