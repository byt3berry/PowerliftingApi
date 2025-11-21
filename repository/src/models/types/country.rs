use types::CountryDto;

use crate::models::SeaCountry;

pub enum Country {
    France,
    Other,
}

impl From<SeaCountry> for Country {
    fn from(value: SeaCountry) -> Self {
        match value {
            SeaCountry::France => Self::France,
            SeaCountry::Other => Self::Other,
        }
    }
}

impl From<Country> for SeaCountry {
    fn from(value: Country) -> Self {
        match value {
            Country::France => Self::France,
            Country::Other => Self::Other,
        }
    }
}

impl From<CountryDto> for Country {
    fn from(value: CountryDto) -> Self {
        match value {
            CountryDto::FRANCE => Self::France,
            CountryDto::OTHER => Self::Other,
        }
    }
}
