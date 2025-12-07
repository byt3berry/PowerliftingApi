use rust_decimal::Decimal;
use types::prelude::*;

use crate::models::types::{Weight, WeightClassKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeightClass {
    pub kind: WeightClassKind,
    pub weight: Weight,
}

impl From<WeightClassDto> for WeightClass {
    fn from(value: WeightClassDto) -> Self {
        match value {
            WeightClassDto::UnderOrEqual(weight) => Self {
                kind: WeightClassKind::UnderOrEqual,
                weight: weight.into(),
            },
            WeightClassDto::Over(weight) => Self {
                kind: WeightClassKind::Over,
                weight: weight.into(),
            },
        }
    }
}

impl From<WeightClass> for WeightClassDto {
    fn from(value: WeightClass) -> Self {
        match value.kind {
            WeightClassKind::UnderOrEqual => Self::UnderOrEqual(value.weight.into()),
            WeightClassKind::Over => Self::Over(value.weight.into()),
        }
    }
}

impl From<Decimal> for WeightClass {
    fn from(mut value: Decimal) -> Self {
        if value.is_sign_positive() {
            Self {
                kind: WeightClassKind::Over,
                weight: value.into(),
            }
        } else {
            value.set_sign_negative(true);
            Self {
                kind: WeightClassKind::UnderOrEqual,
                weight: value.into(),
            }
        }
    }
}

impl From<WeightClass> for Decimal {
    fn from(value: WeightClass) -> Self {
        value.weight.into()
    }
}
