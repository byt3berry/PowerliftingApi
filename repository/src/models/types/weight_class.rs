use rust_decimal::Decimal;
use types::prelude::*;

use crate::models::types::{Weight, WeightClassKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeightClass {
    pub kind: WeightClassKind,
    pub weight: Weight,
}

impl WeightClass {
    pub fn zero() -> Self {
        Self {
            kind: WeightClassKind::None,
            weight: Weight::zero(),
        }
    }
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
            WeightClassDto::None => Self::zero(),
        }
    }
}

impl From<Option<WeightClassDto>> for WeightClass {
    fn from(value: Option<WeightClassDto>) -> Self {
        match value {
            Some(value) => Self::from(value),
            None => Self::zero(),
        }
    }
}

impl From<Decimal> for WeightClass {
    fn from(mut value: Decimal) -> Self {
        if value.is_zero() {
            return Self::zero();
        }

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

impl From<Option<Decimal>> for WeightClass {
    fn from(value: Option<Decimal>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Self::zero(),
        }
    }
}
