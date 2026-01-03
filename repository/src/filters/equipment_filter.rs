use migrations::SimpleExpr;
use sea_orm::{ColumnTrait, DeriveDisplay, EnumIter};

use types::filters::EquipmentFilterDto;

use crate::traits::match_filter::MatchFilter;

#[derive(Clone, Copy, Debug, DeriveDisplay, EnumIter, Eq, PartialEq)]
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

impl From<EquipmentFilterDto> for EquipmentFilter {
    fn from(value: EquipmentFilterDto) -> Self {
        match value {
            EquipmentFilterDto::Raw => Self::Raw,
            EquipmentFilterDto::Wraps => Self::Wraps,
            EquipmentFilterDto::Single => Self::Single,
            EquipmentFilterDto::Multi => Self::Multi,
            EquipmentFilterDto::Straps => Self::Straps,
            EquipmentFilterDto::Sleeves => Self::Sleeves,
            EquipmentFilterDto::Bare => Self::Bare,
            EquipmentFilterDto::Unlimited => Self::Unlimited,
        }
    }
}

impl MatchFilter for EquipmentFilter {
    fn eq<T>(self, column: T) -> Option<SimpleExpr>
    where T: ColumnTrait
    {
        Some(column.eq(self.to_string().to_lowercase()))
    }
}
