use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, Zero};

const SCALE: u32 = 4;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Weight(pub Decimal);

impl Weight {
    pub fn zero() -> Self {
        Self::from(Decimal::zero())
    }
}

impl From<types::Weight> for Weight {
    fn from(value: types::Weight) -> Self {
        Self::from(value.0)
    }
}

impl From<Option<types::Weight>> for Weight {
    fn from(value: Option<types::Weight>) -> Self {
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
            None => panic!("invalid weight value {}", value),
        }
    }
}

impl From<Decimal> for Weight {
    fn from(value: Decimal) -> Self {
        Self(value)
    }
}

impl Into<Decimal> for Weight {
    fn into(self) -> Decimal {
        self.0
    }
}
