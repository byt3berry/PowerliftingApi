use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

use crate::schema::sql_types;

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Division)]
pub enum Division {
    Any,
    Open,
    G,
    Cadet,
    Elite,
    SubJuniors,
    Juniors,
    Seniors,
    Masters,
    Masters1,
    Masters2,
    Masters3,
    Masters4
}

impl ToSql<sql_types::Division, Pg> for Division {
    fn to_sql<'b>(&'b self, output: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Self::Any => output.write_all(b"Any")?,
            Self::Open => output.write_all(b"Open")?,
            Self::G => output.write_all(b"G")?,
            Self::Cadet => output.write_all(b"Cadet")?,
            Self::Elite => output.write_all(b"Elite")?,
            Self::SubJuniors => output.write_all(b"SubJuniors")?,
            Self::Juniors => output.write_all(b"Juniors")?,
            Self::Seniors => output.write_all(b"Seniors")?,
            Self::Masters => output.write_all(b"Masters")?,
            Self::Masters1 => output.write_all(b"Masters1")?,
            Self::Masters2 => output.write_all(b"Masters2")?,
            Self::Masters3 => output.write_all(b"Masters3")?,
            Self::Masters4 => output.write_all(b"Masters4")?,
        }

        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::Division, Pg> for Division {
    fn from_sql<'b>(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Any" => Ok(Self::Any),
            b"Open" => Ok(Self::Open),
            b"G" => Ok(Self::G),
            b"Cadet" => Ok(Self::Cadet),
            b"Elite" => Ok(Self::Elite),
            b"SubJuniors" => Ok(Self::SubJuniors),
            b"Juniors" => Ok(Self::Juniors),
            b"Seniors" => Ok(Self::Seniors),
            b"Masters" => Ok(Self::Masters),
            b"Masters1" => Ok(Self::Masters1),
            b"Masters2" => Ok(Self::Masters2),
            b"Masters3" => Ok(Self::Masters3),
            b"Masters4" => Ok(Self::Masters4),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl From<types::Division> for Division {
    fn from(value: types::Division) -> Self {
        match value {
            types::Division::Any => Self::Any,
            types::Division::Open => Self::Open,
            types::Division::G => Self::G,
            types::Division::Cadet => Self::Cadet,
            types::Division::Elite => Self::Elite,
            types::Division::SubJuniors => Self::SubJuniors,
            types::Division::Juniors => Self::Juniors,
            types::Division::Masters => Self::Masters,
            types::Division::Seniors => Self::Seniors,
            types::Division::Masters1 => Self::Masters1,
            types::Division::Masters2 => Self::Masters2,
            types::Division::Masters3 => Self::Masters3,
            types::Division::Masters4 => Self::Masters4,
        }
    }
}
