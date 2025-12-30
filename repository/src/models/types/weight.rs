use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;

use sea_orm::TryGetable;
use types::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Weight(pub Decimal);

impl TryGetable for Weight {
    fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, index: I) -> Result<Self, sea_orm::TryGetError> {
        let output = Decimal::try_get_by(res, index)?;
        Ok(output.into())
    }
}

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

impl From<Weight> for WeightDto {
    fn from(value: Weight) -> Self {
        Self(value.0)
    }
}

impl From<Decimal> for Weight {
    fn from(value: Decimal) -> Self {
        Self(value)
    }
}
