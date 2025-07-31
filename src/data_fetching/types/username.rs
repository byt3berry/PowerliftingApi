use anyhow::{bail, Error, Result};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use std::fmt::{self, Display};

#[derive(Clone, Debug, Ord, PartialOrd)]
pub struct Username {
    pub name: String,
    parts: Vec<String>,
}

impl Username {
    fn new(name: &str, parts: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            parts,
        }
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

        if parts.is_empty() {
            bail!("Username must not be null");
        }

        Ok(Self::new(s, parts))
    }
}

impl Eq for Username { }

impl PartialEq for Username {
    /// `self` is the entry in the opl-data csv files
    /// `other` is the powerlifter name requested
    fn eq(&self, other: &Self) -> bool {
        if self.parts.len() < other.parts.len() {
            return false;
        }

        for part in &other.parts {
            if !self.parts.contains(part) {
                return false;
            }
        }

        true
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

    #[test]
    #[should_panic]
    fn test_deserialize_failure() {
        let input: String = String::new();
        input.parse::<Username>().unwrap();
    }

    #[rstest]
    #[case("a b", Username::new("a b", vec!["a".to_string(), "b".to_string()]))]
    #[case("A B", Username::new("A B", vec!["a".to_string(), "b".to_string()]))]
    #[case("a  b", Username::new("a b", vec!["a".to_string(), "b".to_string()]))]
    #[case("a b c", Username::new("a b c", vec!["a".to_string(), "b".to_string(), "c".to_string()]))]
    fn test_deserialize(
        #[case] input: String,
        #[case] expected: Username,
    ) -> Result<()> {
        let username: Username = input.parse()?;

        assert_eq!(username, expected);
        Ok(())
    }

    #[rstest]
    #[case("a b", "a b")]
    #[case("a  b", "a b")]
    #[case("a b", "b a")]
    #[case("a b c", "c b a")]
    #[case("a b c", "b c a")]
    #[case("a b c", "c a b")]
    #[case("a b c", "a b")]
    #[case("a b c", "b a")]
    fn test_compare(
        #[case] first: String,
        #[case] second: String,
    ) -> Result<()> {
        let first_username: Username = first.parse()?;
        let second_username: Username = second.parse()?;

        assert_eq!(first_username, second_username);
        Ok(())
    }
}
