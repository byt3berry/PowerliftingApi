use rust_decimal::Decimal;
use sea_orm::{ColIdx, QueryResult, TryGetError, TryGetable};

use types::prelude::WeightClassDto;

use crate::models::types::Weight;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WeightClass {
    UnderOrEqual(Weight),
    Over(Weight),
}

impl TryGetable for WeightClass {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let output = Decimal::try_get_by(res, index)?;
        Ok(output.into())
    }
}

impl From<WeightClassDto> for WeightClass {
    fn from(value: WeightClassDto) -> Self {
        match value {
            WeightClassDto::UnderOrEqual(weight) => Self::UnderOrEqual(weight.into()),
            WeightClassDto::Over(weight) => Self::Over(weight.into()),
        }
    }
}

impl From<WeightClass> for WeightClassDto {
    fn from(value: WeightClass) -> Self {
        match value {
            WeightClass::UnderOrEqual(weight) => Self::UnderOrEqual(weight.into()),
            WeightClass::Over(weight) => Self::Over(weight.into()),
        }
    }
}

impl From<Decimal> for WeightClass {
    fn from(mut value: Decimal) -> Self {
        if value.is_sign_positive() {
            Self::Over(value.into())
        } else {
            value.set_sign_negative(true);
            Self::UnderOrEqual(value.into())
        }
    }
}
