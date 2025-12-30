use sea_orm::TryGetable;
use types::prelude::UsernameDto;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Username {
    pub name: String,
    pub parts: Vec<String>,
}

impl TryGetable for Username {
    fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, index: I) -> Result<Self, sea_orm::TryGetError> {
        let output = String::try_get_by(res, index)?;
        Ok(output.into())
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

impl From<String> for Username {
    fn from(value: String) -> Self {
        let parts: Vec<String> = value
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .map(str::to_lowercase)
            .collect();

        Self::new(&value, parts)
    }
}
