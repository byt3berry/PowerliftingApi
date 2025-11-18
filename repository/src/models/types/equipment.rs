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

impl From<types::Equipment> for Equipment {
    fn from(value: types::Equipment) -> Self {
        match value {
            types::Equipment::Any => Self::Any,
            types::Equipment::Raw => Self::Raw,
            types::Equipment::Wraps => Self::Wraps,
            types::Equipment::Single => Self::Single,
            types::Equipment::Multi => Self::Multi,
            types::Equipment::Straps => Self::Straps,
            types::Equipment::Sleeves => Self::Sleeves,
            types::Equipment::Bare => Self::Bare,
            types::Equipment::Unlimited => Self::Unlimited,
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

impl Into<SeaEquipment> for Equipment {
    fn into(self) -> SeaEquipment {
        match self {
            Self::Any => SeaEquipment::Any,
            Self::Raw => SeaEquipment::Raw,
            Self::Wraps => SeaEquipment::Wraps,
            Self::Single => SeaEquipment::Single,
            Self::Multi => SeaEquipment::Multi,
            Self::Straps => SeaEquipment::Straps,
            Self::Sleeves => SeaEquipment::Sleeves,
            Self::Bare => SeaEquipment::Bare,
            Self::Unlimited => SeaEquipment::Unlimited,
        }
    }
}
