use std::ops::Deref;
use std::str::FromStr;

use crate::models::types::Username;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PowerlifterFilter(pub Vec<Username>);

impl From<String> for PowerlifterFilter {
    fn from(value: String) -> Self {
        let powerlifters: Vec<Username> = value
            .lines()
            .map(Username::from_str)
            .filter_map(Result::ok)
            .collect();

        Self(powerlifters)
    }
}

impl Deref for PowerlifterFilter {
    type Target = Vec<Username>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
