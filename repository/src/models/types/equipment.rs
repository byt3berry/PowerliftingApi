use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

use crate::schema::sql_types;

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Equipment)]
pub enum Equipment {
    Any,
    Raw,
    Wraps,
    Single,
    Multi,
    Straps,
    Sleeves,
    Bare,
    Unlimited
}

impl ToSql<sql_types::Equipment, Pg> for Equipment {
    fn to_sql<'b>(&'b self, output: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Self::Any => output.write_all(b"Any")?,
            Self::Raw => output.write_all(b"Raw")?,
            Self::Wraps => output.write_all(b"Wraps")?,
            Self::Single => output.write_all(b"Single")?,
            Self::Multi => output.write_all(b"Multi")?,
            Self::Straps => output.write_all(b"Straps")?,
            Self::Sleeves => output.write_all(b"Sleeves")?,
            Self::Bare => output.write_all(b"Bare")?,
            Self::Unlimited => output.write_all(b"Unlimited")?,
        }

        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::Equipment, Pg> for Equipment {
    fn from_sql<'b>(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Any" => Ok(Self::Any),
            b"Raw" => Ok(Self::Raw),
            b"Wraps" => Ok(Self::Wraps),
            b"Single" => Ok(Self::Single),
            b"Multi" => Ok(Self::Multi),
            b"Straps" => Ok(Self::Straps),
            b"Sleeves" => Ok(Self::Sleeves),
            b"Bare" => Ok(Self::Bare),
            b"Unlimited" => Ok(Self::Unlimited),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl From<types::Equipment> for Equipment {
    fn from(value: types::Equipment) -> Self {
        match value {
            types::Equipment::Any => Self::Any,
            types::Equipment::Raw => Self::Raw,
            types::Equipment::Wraps => Self::Wraps,
            types::Equipment::Single => Self::Single,
            types::Equipment::Multi => Self::Multi,
            types::Equipment::Straps => Self::Straps,
            types::Equipment::Sleeves => Self::Sleeves,
            types::Equipment::Bare => Self::Bare,
            types::Equipment::Unlimited => Self::Unlimited,
        }
    }
}
