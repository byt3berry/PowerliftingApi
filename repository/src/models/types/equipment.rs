use types::prelude::*;

use crate::models::SeaEquipment;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Equipment {
    Any,
    Raw,
    Wraps,
    Single,
    Multi,
    Straps,
    Sleeves,
    Bare,
    Unlimited
}

impl From<EquipmentDto> for Equipment {
    fn from(value: EquipmentDto) -> Self {
        match value {
            EquipmentDto::Any => Self::Any,
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

impl From<SeaEquipment> for Equipment {
    fn from(value: SeaEquipment) -> Self {
        match value {
            SeaEquipment::Any => Self::Any,
            SeaEquipment::Raw => Self::Raw,
            SeaEquipment::Wraps => Self::Wraps,
            SeaEquipment::Single => Self::Single,
            SeaEquipment::Multi => Self::Multi,
            SeaEquipment::Straps => Self::Straps,
            SeaEquipment::Sleeves => Self::Sleeves,
            SeaEquipment::Bare => Self::Bare,
            SeaEquipment::Unlimited => Self::Unlimited,
        }
    }
}

impl From<Equipment> for SeaEquipment {
    fn from(value: Equipment) -> Self {
        match value {
            Equipment::Any => Self::Any,
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
