use std::str::FromStr;

use anyhow::{Error, Result};
use sea_orm::{DbErr, TryGetError, TryGetable};
use types::prelude::UsernameDto;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Username {
    pub name: String,
    pub parts: Vec<String>,
}

impl TryGetable for Username {
    fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, index: I) -> Result<Self, TryGetError> {
        let output: String = String::try_get_by(res, index)?;
        Self::from_str(&output)
            .map_err(|err| {
                let error: String = format!("{err:#}");
                TryGetError::DbErr(DbErr::Type(error))
            })
    }
}

impl Username {
    pub const fn empty() -> Self {
        Self {
            name: String::new(),
            parts: Vec::new(),
        }
    }

    #[must_use]
    pub fn new(name: &str, parts: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            parts,
        }
    }
}

impl From<UsernameDto> for Username {
    fn from(value: UsernameDto) -> Self {
        Self::new(&value.name, value.parts)
    }
}

impl From<Username> for UsernameDto {
    fn from(value: Username) -> Self {
        Self::new(&value.name, value.parts)
    }
}

impl FromStr for Username {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<String> = s
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .map(str::to_lowercase)
            .collect();

        Ok(Self::new(s, parts))
    }
}
