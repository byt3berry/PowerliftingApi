use types::EquipmentDto;

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
            types::EquipmentDto::Any => Self::Any,
            types::EquipmentDto::Raw => Self::Raw,
            types::EquipmentDto::Wraps => Self::Wraps,
            types::EquipmentDto::Single => Self::Single,
            types::EquipmentDto::Multi => Self::Multi,
            types::EquipmentDto::Straps => Self::Straps,
            types::EquipmentDto::Sleeves => Self::Sleeves,
            types::EquipmentDto::Bare => Self::Bare,
            types::EquipmentDto::Unlimited => Self::Unlimited,
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
