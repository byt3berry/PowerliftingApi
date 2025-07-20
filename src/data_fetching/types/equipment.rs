use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq)]
pub enum Equipment {
    Raw,
    Wraps,
    Single,
    Multi,
    Straps,
    #[default]
    Unlimited,
}
