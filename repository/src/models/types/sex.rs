use types::prelude::*;

use crate::models::SeaSex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sex {
    M,
    F,
}

impl From<SexDto> for Sex {
    fn from(value: SexDto) -> Self {
        match value {
            SexDto::M => Self::M,
            SexDto::F => Self::F,
        }
    }
}

impl From<SeaSex> for Sex {
    fn from(value: SeaSex) -> Self {
        match value {
            SeaSex::M => Self::M,
            SeaSex::F => Self::F,
        }
    }
}

impl From<Sex> for SeaSex {
    fn from(value: Sex) -> Self {
        match value {
            Sex::M => Self::M,
            Sex::F => Self::F,
        }
    }
}
