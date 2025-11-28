use types::prelude::*;

use crate::models::SeaDivision;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Division {
    Any,
    Open,
    G,
    Cadet,
    Elite,
    SubJuniors,
    Juniors,
    Seniors,
    Masters,
    Masters1,
    Masters2,
    Masters3,
    Masters4
}

impl From<DivisionDto> for Division {
    fn from(value: DivisionDto) -> Self {
        match value {
            DivisionDto::Any => Self::Any,
            DivisionDto::Open => Self::Open,
            DivisionDto::G => Self::G,
            DivisionDto::Cadet => Self::Cadet,
            DivisionDto::Elite => Self::Elite,
            DivisionDto::SubJuniors => Self::SubJuniors,
            DivisionDto::Juniors => Self::Juniors,
            DivisionDto::Masters => Self::Masters,
            DivisionDto::Seniors => Self::Seniors,
            DivisionDto::Masters1 => Self::Masters1,
            DivisionDto::Masters2 => Self::Masters2,
            DivisionDto::Masters3 => Self::Masters3,
            DivisionDto::Masters4 => Self::Masters4,
        }
    }
}

impl From<SeaDivision> for Division {
    fn from(value: SeaDivision) -> Self {
        match value {
            SeaDivision::Any => Self::Any,
            SeaDivision::Open => Self::Open,
            SeaDivision::G => Self::G,
            SeaDivision::Cadet => Self::Cadet,
            SeaDivision::Elite => Self::Elite,
            SeaDivision::Subjuniors => Self::SubJuniors,
            SeaDivision::Juniors => Self::Juniors,
            SeaDivision::Masters => Self::Masters,
            SeaDivision::Seniors => Self::Seniors,
            SeaDivision::Masters1 => Self::Masters1,
            SeaDivision::Masters2 => Self::Masters2,
            SeaDivision::Masters3 => Self::Masters3,
            SeaDivision::Masters4 => Self::Masters4,
        }
    }
}

impl From<Division> for SeaDivision {
    fn from(value: Division) -> Self {
        match value {
            Division::Any => Self::Any,
            Division::Open => Self::Open,
            Division::G => Self::G,
            Division::Cadet => Self::Cadet,
            Division::Elite => Self::Elite,
            Division::SubJuniors => Self::Subjuniors,
            Division::Juniors => Self::Juniors,
            Division::Masters => Self::Masters,
            Division::Seniors => Self::Seniors,
            Division::Masters1 => Self::Masters1,
            Division::Masters2 => Self::Masters2,
            Division::Masters3 => Self::Masters3,
            Division::Masters4 => Self::Masters4,
        }
    }
}
