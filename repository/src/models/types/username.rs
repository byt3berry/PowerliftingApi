use types::prelude::UsernameDto;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Username {
    pub name: String,
    pub parts: Vec<String>,
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

impl From<Username> for String {
    fn from(value: Username) -> Self {
        value.name
    }
}
