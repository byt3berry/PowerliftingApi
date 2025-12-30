#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UsernameDto {
    pub name: String,
    pub parts: Vec<String>,
}

impl UsernameDto {
    #[must_use]
    pub fn new(name: &str, parts: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            parts,
        }
    }
}

impl From<UsernameDto> for String {
    fn from(value: UsernameDto) -> Self {
        value.name
    }
}
