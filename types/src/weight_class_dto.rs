use std::fmt::Display;

use rust_decimal::Decimal;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeightClassDto {
    UnderOrEqual(WeightDto),
    Over(WeightDto),
}

impl Display for WeightClassDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnderOrEqual(weight)
                | Self::Over(weight) => f.write_str(&weight.to_string()),
        }
    }
}

impl From<Decimal> for WeightClassDto {
    fn from(mut value: Decimal) -> Self {
        if value.is_sign_positive() {
            Self::Over(value.into())
        } else {
            value.set_sign_negative(true);
            Self::UnderOrEqual(value.into())
        }
    }
}

impl From<WeightClassDto> for Decimal {
    fn from(value: WeightClassDto) -> Self {
        match value {
            WeightClassDto::UnderOrEqual(weight)
                | WeightClassDto::Over(weight) => weight.into(),
        }
    }
}
