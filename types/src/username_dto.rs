#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UsernameDto {
    pub name: String,
    pub parts: Vec<String>,
}

impl UsernameDto {
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

impl From<String> for UsernameDto {
    fn from(value: String) -> Self {
        let parts: Vec<String> = value
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .map(str::to_lowercase)
            .collect();

        Self::new(&value, parts)
    }
}

impl From<UsernameDto> for String {
    fn from(value: UsernameDto) -> Self {
        value.name
    }
}
