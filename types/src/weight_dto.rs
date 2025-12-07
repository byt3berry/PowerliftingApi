use std::fmt::Display;

use rust_decimal::prelude::{FromPrimitive, Zero};
use rust_decimal::Decimal;

const SCALE: u32 = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WeightDto(pub Decimal);

impl WeightDto {
    #[must_use]
    pub fn is_zero(self) -> bool {
        self == Self::from(0.)
    }

    pub fn zero() -> Self {
        Self::from(Decimal::zero())
    }
}

impl From<f32> for WeightDto {
    fn from(value: f32) -> Self {
        match Decimal::from_f32(value) {
            Some(value) => Self(value.trunc_with_scale(SCALE)),
            None => Self::zero(),
        }
    }
}

impl From<Decimal> for WeightDto {
    fn from(value: Decimal) -> Self {
        Self(value)
    }
}

impl From<WeightDto> for Decimal {
    fn from(value: WeightDto) -> Self {
        value.0
    }
}

impl ToString for WeightDto {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
