use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

use crate::schema::sql_types;

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Country)]
pub enum Country {
    FRANCE,
    OTHER,
}

impl ToSql<sql_types::Country, Pg> for Country {
    fn to_sql<'b>(&'b self, output: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Self::FRANCE => output.write_all(b"FRANCE")?,
            Self::OTHER => output.write_all(b"OTHER")?,
        }

        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::Country, Pg> for Country {
    fn from_sql<'b>(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"FRANCE" => Ok(Self::FRANCE),
            b"OTHER" => Ok(Self::OTHER),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl From<types::Country> for Country {
    fn from(value: types::Country) -> Self {
        match value {
            types::Country::FRANCE => Self::FRANCE,
            types::Country::OTHER => Self::OTHER,
        }
    }
}
