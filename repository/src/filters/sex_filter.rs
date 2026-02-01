use sea_orm::prelude::{ColumnTrait, DeriveDisplay, EnumIter};
use sea_orm_migration::prelude::SimpleExpr;

use types::filters::SexFilterDto;

use crate::traits::MatchFilter;

#[derive(Clone, Copy, Debug, DeriveDisplay, EnumIter, Eq, PartialEq)]
pub enum SexFilter {
    Any,
    M,
    F,
}

impl From<SexFilterDto> for SexFilter {
    fn from(value: SexFilterDto) -> Self {
        match value {
            SexFilterDto::Any => Self::Any,
            SexFilterDto::M => Self::M,
            SexFilterDto::F => Self::F,
        }
    }
}

impl MatchFilter for SexFilter {
    fn eq<T>(self, column: T) -> Option<SimpleExpr>
    where T: ColumnTrait
    {
        if self == Self::Any {
            return None;
        }

        Some(column.eq(self.to_string().to_lowercase()))
    }
}
