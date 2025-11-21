use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, Zero};
use types::WeightDto;

const SCALE: u32 = 4;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Weight(pub Decimal);

impl Weight {
    pub fn zero() -> Self {
        Self::from(Decimal::zero())
    }
}

impl From<WeightDto> for Weight {
    fn from(value: WeightDto) -> Self {
        Self::from(value.0)
    }
}

impl From<Option<types::WeightDto>> for Weight {
    fn from(value: Option<types::WeightDto>) -> Self {
        match value {
            Some(value) => Self::from(value),
            None => Self::zero(),
        }
    }
}

impl From<f32> for Weight {
    fn from(value: f32) -> Self {
        match Decimal::from_f32(value) {
            Some(value) => Self(value.trunc_with_scale(SCALE)),
            None => Self::zero(),
        }
    }
}

impl From<Decimal> for Weight {
    fn from(value: Decimal) -> Self {
        Self(value)
    }
}

impl From<Weight> for Decimal {
    fn from(value: Weight) -> Self {
        value.0
    }
}
