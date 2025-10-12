use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

use crate::schema::sql_types;

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Federation)]
pub enum Federation {
    FFForce,
    EPF,
    IPF,
    FFHMFAC,
    OTHER,
}

impl ToSql<sql_types::Federation, Pg> for Federation {
    fn to_sql<'b>(&'b self, output: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Self::FFForce => output.write_all(b"FFFORCE")?,
            Self::EPF => output.write_all(b"EPF")?,
            Self::IPF => output.write_all(b"IPF")?,
            Self::FFHMFAC => output.write_all(b"FFHMFAC")?,
            Self::OTHER => output.write_all(b"Other")?,
        }

        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::Federation, Pg> for Federation {
    fn from_sql<'b>(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"FFFORCE" => Ok(Self::FFForce),
            b"EPF" => Ok(Self::EPF),
            b"IPF" => Ok(Self::IPF),
            b"FFHMFAC" => Ok(Self::FFHMFAC),
            b"Other" => Ok(Self::OTHER),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl From<types::Federation> for Federation {
    fn from(value: types::Federation) -> Self {
        match value {
            types::Federation::FFForce => Self::FFForce,
            types::Federation::EPF => Self::EPF,
            types::Federation::IPF => Self::IPF,
            types::Federation::FFHMFAC => Self::FFHMFAC,
            types::Federation::OTHER => Self::OTHER,
        }
    }
}
