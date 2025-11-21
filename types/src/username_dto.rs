use anyhow::{Error, Result};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use std::fmt::{self, Display};

use crate::{Matches, MatchesQuery, Query};

#[derive(Clone, Debug, Ord, PartialOrd)]
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

    pub fn matches_str(&self, input: &str) -> bool {
        input
            .lines()
            .map(UsernameDto::from_str)
            .filter_map(Result::ok)
            .any(|username| self.matches(&username))
    }
}

impl Matches for UsernameDto {
    fn matches(&self, other: &Self) -> bool {
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

impl MatchesQuery for UsernameDto {
    fn matches_query(&self, query: &Query) -> bool {
        query.powerlifters
            .lines()
            .map(UsernameDto::from_str)
            .filter_map(Result::ok)
            .any(|query| self.matches(&query))
    }
}

impl FromStr for UsernameDto {
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

impl Eq for UsernameDto { }

impl PartialEq for UsernameDto {
    /// `self` is the entry in the opl-data csv files
    /// `other` is the powerlifter name requested
    fn eq(&self, other: &Self) -> bool {
        self.parts.eq(&other.parts)
    }
}

impl Display for UsernameDto {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.name)
    }
}

struct UsernameVisitor;

impl Visitor<'_> for UsernameVisitor {
    type Value = UsernameDto;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid username")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        UsernameDto::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for UsernameDto {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(UsernameVisitor)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::{assert_eq, Comparison};
    use rstest::rstest;

    use crate::Matches;

    use super::UsernameDto;

    #[rstest]
    #[case("a b", UsernameDto::new("a b", vec!["a".to_string(), "b".to_string()]))]
    #[case("A B", UsernameDto::new("A B", vec!["a".to_string(), "b".to_string()]))]
    #[case("a  b", UsernameDto::new("a b", vec!["a".to_string(), "b".to_string()]))]
    #[case("a b c", UsernameDto::new("a b c", vec!["a".to_string(), "b".to_string(), "c".to_string()]))]
    fn test_deserialize(
        #[case] input: String,
        #[case] expected: UsernameDto,
    ) {
        let result: Result<UsernameDto> = input.parse();

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
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
    fn test_compare_eq(
        #[case] first: UsernameDto,
        #[case] second: UsernameDto,
    ) {
        let result: bool = first.matches(&second);

        assert!(result, "{}", Comparison::new(&first, &second));
    }

    #[rstest]
    #[case("", "a b")]
    #[case("a b", "c d")]
    fn test_compare_not_eq(
        #[case] first: UsernameDto,
        #[case] second: UsernameDto,
    ) {
        let result: bool = first.matches(&second);

        assert!(!result, "{}", Comparison::new(&first, &second));
    }

    #[test]
    fn test_empty() {
        let username: UsernameDto = UsernameDto::empty();

        assert_eq!(String::new(), username.name);
        assert_eq!(Vec::<String>::new(), username.parts);
    }
}
