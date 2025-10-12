use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Numeric;

use crate::schema::sql_types;

#[derive(Clone, Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Weight)]
pub struct Weight(pub BigDecimal);

impl ToSql<sql_types::Weight, Pg> for Weight {
    fn to_sql<'b>(&'b self, output: &mut Output<'b, '_, Pg>) -> serialize::Result {
        <BigDecimal as ToSql<Numeric, Pg>>::to_sql(&self.0, output)
    }
}

impl FromSql<sql_types::Weight, Pg> for Weight {
    fn from_sql<'b>(bytes: PgValue) -> deserialize::Result<Self> {
        let weight: BigDecimal = <BigDecimal as FromSql<Numeric, Pg>>::from_sql(bytes)?;

        Ok(Self(weight))
    }
}

impl From<types::Weight> for Weight {
    fn from(value: types::Weight) -> Self {
        match BigDecimal::from_f32(value.0) {
            Some(value) => Self(value),
            None => panic!("invalid weight value {}", value.0),
        }
    }
}

impl From<Option<types::Weight>> for Weight {
    fn from(value: Option<types::Weight>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Self(BigDecimal::from(0)),
        }
    }
}
