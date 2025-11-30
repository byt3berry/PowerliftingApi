use anyhow::{Error, Result};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use types::prelude::UsernameDto;
use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Clone, Debug, Ord, PartialOrd)]
pub struct Username {
    pub name: String,
    parts: Vec<String>,
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

impl From<Username> for UsernameDto {
    fn from(value: Username) -> Self {
        UsernameDto::new(&value.name, value.parts)
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

impl Eq for Username { }

impl PartialEq for Username {
    /// `self` is the entry in the opl-data csv files
    /// `other` is the powerlifter name requested
    fn eq(&self, other: &Self) -> bool {
        self.parts.eq(&other.parts)
    }
}

impl Display for Username {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.name)
    }
}

struct UsernameVisitor;

impl Visitor<'_> for UsernameVisitor {
    type Value = Username;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid username")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Username::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Username {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(UsernameVisitor)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::Username;

    #[rstest]
    #[case("a b", Username::new("a b", vec!["a".to_string(), "b".to_string()]))]
    #[case("A B", Username::new("A B", vec!["a".to_string(), "b".to_string()]))]
    #[case("a  b", Username::new("a b", vec!["a".to_string(), "b".to_string()]))]
    #[case("a b c", Username::new("a b c", vec!["a".to_string(), "b".to_string(), "c".to_string()]))]
    fn test_deserialize(
        #[case] input: String,
        #[case] expected: Username,
    ) {
        let result: Result<Username> = input.parse();

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_empty() {
        let username: Username = Username::empty();

        assert_eq!(String::new(), username.name);
        assert_eq!(Vec::<String>::new(), username.parts);
    }
}
