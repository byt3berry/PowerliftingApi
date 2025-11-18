use crate::models::SeaCountry;

pub enum Country {
    FRANCE,
    OTHER,
}

impl From<SeaCountry> for Country {
    fn from(value: SeaCountry) -> Self {
        match value {
            SeaCountry::France => Self::FRANCE,
            SeaCountry::Other => Self::OTHER,
        }
    }
}

impl Into<SeaCountry> for Country {
    fn into(self) -> SeaCountry {
        match self {
            Self::FRANCE => SeaCountry::France,
            Self::OTHER => SeaCountry::Other,
        }
    }
}

impl From<types::Country> for Country {
    fn from(value: types::Country) -> Self {
        match value {
            types::Country::FRANCE => Self::FRANCE,
            types::Country::OTHER => Self::OTHER,
        }
    }
}
