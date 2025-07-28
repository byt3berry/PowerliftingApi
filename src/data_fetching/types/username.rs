use anyhow::{bail, Error, Result};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::ops::Deref;
use std::str::FromStr;
use std::fmt::{self, Display};

#[derive(Clone, Debug, Ord, PartialOrd)]
pub struct Username(pub Vec<String>);

impl FromStr for Username {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<String> = s
            .split_whitespace()
            .map(str::to_string)
            .filter(|w| !w.is_empty())
            .collect();

        if parts.is_empty() {
            bail!("Username must not be null");
        }

        Ok(Self(parts))
    }
}

impl Eq for Username { }

impl PartialEq for Username {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for part in self.iter() {
            if !other.contains(part) {
                return false;
            }
        }

        true
    }
}

impl Deref for Username {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.join(" "))
    }
}

struct UsernameVisitor;

impl Visitor<'_> for UsernameVisitor {
    type Value = Username;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid weight class")
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
    #[case("a b", Username(vec!["a".to_string(), "b".to_string()]))]
    #[case("a  b", Username(vec!["a".to_string(), "b".to_string()]))]
    #[case("a b c", Username(vec!["a".to_string(), "b".to_string(), "c".to_string()]))]
    fn test_deserialize(
        #[case] input: String,
        #[case] expected: Username,
    ) {
        let username: Username = input.parse().unwrap();

        assert_eq!(username, expected);
    }
}
