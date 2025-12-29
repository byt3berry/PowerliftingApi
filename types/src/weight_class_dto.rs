use rust_decimal::Decimal;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeightClassDto {
    UnderOrEqual(WeightDto),
    Over(WeightDto),
}

impl ToString for WeightClassDto {
    fn to_string(&self) -> String {
        match self {
            WeightClassDto::UnderOrEqual(weight) => weight.to_string(),
            WeightClassDto::Over(weight) => weight.to_string(),
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

impl Into<Decimal> for WeightClassDto {
    fn into(self) -> Decimal {
        match self {
            WeightClassDto::UnderOrEqual(weight) => weight.into(),
            WeightClassDto::Over(weight) => weight.into(),
        }
    }
}
