use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, Output, ToSql, WriteTuple};
use diesel::sql_types::{Numeric, Record};

use crate::models::types::weight::Weight;
use crate::models::types::weight_class_kind::{PgWeightClassKind, WeightClassKind};
use crate::schema::sql_types;

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Weightclass)]
pub struct WeightClass {
    pub kind: WeightClassKind,
    pub weight: Weight,
}

impl ToSql<sql_types::Weightclass, Pg> for WeightClass {
    fn to_sql<'b>(&'b self, output: &mut Output<'b, '_, Pg>) -> serialize::Result {
        WriteTuple::<(PgWeightClassKind, sql_types::Weight)>::write_tuple(
            &(self.kind.clone(), self.weight.clone()),
            &mut output.reborrow(),
        )
    }
}

impl FromSql<sql_types::Weightclass, Pg> for WeightClass {
    fn from_sql<'b>(bytes: PgValue) -> deserialize::Result<Self> {
        let (kind, weight): (WeightClassKind, Weight) =
            <(WeightClassKind, Weight) as FromSql<Record<(
                PgWeightClassKind,
                sql_types::Weight
            )>, Pg>>::from_sql(bytes)?;

        Ok(WeightClass {
            kind,
            weight,
        })
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
            types::WeightClass::None => Self {
                kind: WeightClassKind::Over,
                weight: Weight(BigDecimal::from(0)),
            },
        }
    }
}

impl From<Option<types::WeightClass>> for WeightClass {
    fn from(value: Option<types::WeightClass>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Self {
                kind: WeightClassKind::None,
                weight: Weight(BigDecimal::from(0)),
            },
        }
    }
}
