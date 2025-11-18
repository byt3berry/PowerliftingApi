use rust_decimal::Decimal;

use crate::models::types::weight::Weight;
use crate::models::types::weight_class_kind::WeightClassKind;
use crate::models::SeaActiveEntry;

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

impl From<types::WeightClass> for WeightClass {
    fn from(value: types::WeightClass) -> Self {
        match value {
            types::WeightClass::UnderOrEqual(weight) => Self {
                kind: WeightClassKind::UnderOrEqual,
                weight: weight.into(),
            },
            types::WeightClass::Over(weight) => Self {
                kind: WeightClassKind::Over,
                weight: weight.into(),
            },
            types::WeightClass::None => Self::zero(),
        }
    }
}

impl From<Option<types::WeightClass>> for WeightClass {
    fn from(value: Option<types::WeightClass>) -> Self {
        match value {
            Some(value) => Self::from(value),
            None => Self::zero(),
        }
    }
}

impl From<Decimal> for WeightClass {
    fn from(value: Decimal) -> Self {
        if value.is_zero() {
            return Self::zero();
        }

        match value.is_sign_positive() {
            true => Self {
                kind: WeightClassKind::Over,
                weight: value.abs().into(),
            },
            false => Self {
                kind: WeightClassKind::UnderOrEqual,
                weight: value.abs().into(),
            },
        }
    }
}

impl Into<Decimal> for WeightClass {
    fn into(self) -> Decimal {
        self.weight.into()
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
