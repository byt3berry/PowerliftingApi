use crate::models::SeaSex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sex {
    M,
    F,
}

impl From<types::Sex> for Sex {
    fn from(value: types::Sex) -> Self {
        match value {
            types::Sex::M => Self::M,
            types::Sex::F => Self::F,
            types::Sex::Any => panic!("Sex::Any cannot be stored in the database"),
        }
    }
}

impl From<SeaSex> for Sex {
    fn from(value: SeaSex) -> Self {
        match value {
            SeaSex::M => Self::M,
            SeaSex::F => Self::F,
        }
    }
}

impl Into<SeaSex> for Sex {
    fn into(self) -> SeaSex {
        match self {
            Self::M => SeaSex::M,
            Self::F => SeaSex::F,
        }
    }
}
