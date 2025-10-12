use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

use crate::schema::sql_types;

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Sex)]
pub enum Sex {
    M,
    F,
}

impl ToSql<sql_types::Sex, Pg> for Sex {
    fn to_sql<'b>(&'b self, output: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Self::M => output.write_all(b"M")?,
            Self::F => output.write_all(b"F")?,
        }

        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::Sex, Pg> for Sex {
    fn from_sql<'b>(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"M" => Ok(Self::M),
            b"F" => Ok(Self::F),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl From<types::Sex> for Sex {
    fn from(value: types::Sex) -> Self {
        match value {
            types::Sex::M => Self::M,
            types::Sex::F => Self::F,
            other => panic!("Variant {} can't be stored into database", other),
        }
    }
}
