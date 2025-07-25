use serde::Deserialize;

use crate::data_fetching::types::sex::Sex;
use crate::data_fetching::types::weight::Weight;

#[derive(Debug, Default, Deserialize, Eq, PartialEq)]
pub struct Dots(i32);

impl From<f64> for Dots {
    fn from(f: f64) -> Self {
        if f.is_finite() {
            Self(f.round() as i32)
        } else {
            Self(0)
        }
    }
}

impl Dots {
    /// Multiply and add. On many CPUs, this is a single instruction.
    #[inline]
    fn madd(a: f64, b: f64, c: f64) -> f64 {
        a.mul_add(b, c)
    }

    #[inline]
    fn poly4(a: f64, b: f64, c: f64, d: f64, e: f64, x: f64) -> f64 {
        let x2: f64 = x * x;
        let mut even: f64 = Self::madd(a, x2, c); // Ax^2 + C.
        let odd: f64 = Self::madd(b, x2, d); // Bx^2 + D.
        even = Self::madd(even, x2, e); // Ax^4 + Cx^2 + E.
        Self::madd(odd, x, even) // Ax^4 + Bx^3 + Cx^2 + Dx + E.
    }

    fn dots_coefficient_men(bodyweightkg: f64) -> f64 {
        const A: f64 = -0.000_001_093_000;
        const B: f64 = 0.000_739_129_300;
        const C: f64 = -0.191_875_922_100;
        const D: f64 = 24.090_075_600;
        const E: f64 = -307.750_760;

        // Bodyweight bounds are defined; bodyweights out of range match the boundaries.
        let adjusted: f64 = bodyweightkg.clamp(40., 210.);
        500. / Self::poly4(A, B, C, D, E, adjusted)
    }

    fn dots_coefficient_women(bodyweightkg: f64) -> f64 {
        const A: f64 = -0.000_001_070_600;
        const B: f64 = 0.000_515_856_800;
        const C: f64 = -0.112_665_549_500;
        const D: f64 = 13.617_503_200;
        const E: f64 = -57.962_880;

        // Bodyweight bounds are defined; bodyweights out of range match the boundaries.
        let adjusted: f64 = bodyweightkg.clamp(40., 150.);
        500. / Self::poly4(A, B, C, D, E, adjusted)
    }

    pub fn new(sex: Sex, bodyweight: Weight, total: Weight) -> Self {
        if bodyweight.is_zero() || total.is_zero() {
            return Self(0);
        }

        let coefficient: f64 = match sex {
            Sex::M | Sex::All => Self::dots_coefficient_men(f64::from(bodyweight)),
            Sex::F => Self::dots_coefficient_women(f64::from(bodyweight)),
        };

        Self::from(coefficient * f64::from(total))
    }
}
