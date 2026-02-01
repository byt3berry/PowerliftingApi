use sea_orm::prelude::{ColumnTrait, DeriveDisplay, EnumIter};
use sea_orm_migration::prelude::SimpleExpr;

use types::filters::DivisionFilterDto;

use crate::traits::MatchFilter;

#[derive(Clone, Copy, Debug, DeriveDisplay, EnumIter, Eq, PartialEq)]
pub enum DivisionFilter {
    Any,
    Open,
    G,
    Cadet,
    Elite,
    SubJuniors,
    Juniors,
    Masters,
    Seniors,
    Masters1,
    Masters2,
    Masters3,
    Masters4,
}

impl From<DivisionFilterDto> for DivisionFilter {
    fn from(value: DivisionFilterDto) -> Self {
        match value {
            DivisionFilterDto::Any => Self::Any,
            DivisionFilterDto::Open => Self::Open,
            DivisionFilterDto::G => Self::G,
            DivisionFilterDto::Cadet => Self::Cadet,
            DivisionFilterDto::Elite => Self::Elite,
            DivisionFilterDto::SubJuniors => Self::SubJuniors,
            DivisionFilterDto::Juniors => Self::Juniors,
            DivisionFilterDto::Masters => Self::Masters,
            DivisionFilterDto::Seniors => Self::Seniors,
            DivisionFilterDto::Masters1 => Self::Masters1,
            DivisionFilterDto::Masters2 => Self::Masters2,
            DivisionFilterDto::Masters3 => Self::Masters3,
            DivisionFilterDto::Masters4 => Self::Masters4,
        }
    }
}

impl MatchFilter for DivisionFilter {
    fn eq<T>(self, column: T) -> Option<SimpleExpr>
        where T: ColumnTrait
    {
        if self == Self::Any {
            return None;
        }

        Some(column.eq(self.to_string().to_lowercase()))
    }
}
