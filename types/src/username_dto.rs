#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UsernameDto {
    pub name: String,
    parts: Vec<String>,
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
