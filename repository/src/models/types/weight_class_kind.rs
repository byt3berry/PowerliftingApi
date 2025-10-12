use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::SqlType;
use std::io::Write;

#[derive(SqlType)]
#[postgres(type_name = "WeightClassType")]
pub struct PgWeightClassKind;

#[derive(Clone, Copy, Debug, FromSqlRow)]
#[diesel(sql_type = String)]
pub enum WeightClassKind {
    UnderOrEqual,
    Over,
    None,
}

impl ToSql<PgWeightClassKind, Pg> for WeightClassKind {
    fn to_sql<'b>(&'b self, output: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Self::UnderOrEqual => output.write_all(b"UnderOrEqual")?,
            Self::Over => output.write_all(b"Over")?,
            Self::None => output.write_all(b"None")?,
        }

        Ok(IsNull::No)
    }
}

impl FromSql<PgWeightClassKind, Pg> for WeightClassKind {
    fn from_sql<'b>(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"UnderOrEqual" => Ok(Self::UnderOrEqual),
            b"Over" => Ok(Self::Over),
            b"None" => Ok(Self::None),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
