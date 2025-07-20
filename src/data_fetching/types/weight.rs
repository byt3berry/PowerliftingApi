use std::{num, str::FromStr};

use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct Weight(pub f32);

// Simple format conversions.
impl From<Weight> for f32 {
    #[inline]
    fn from(w: Weight) -> f32 {
        (w.0 as f32) / 100.0
    }
}

impl FromStr for Weight {
    type Err = num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Weight(0.))
        } else {
            Ok(Weight::from_f32(s.parse::<f32>()?))
        }
    }
}

impl Weight {
    pub fn from_f32(f: f32) -> Weight {
        if f.is_finite() {
            Weight(f)
        } else {
            Weight(0.)
        }
    }
}
