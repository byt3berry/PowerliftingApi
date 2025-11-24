use serde::Deserialize;
use types::CountryDto;
use types::FederationDto;
use types::MatchesQuery;
use types::MeetDataDto;
use types::Query;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct MeetData {
    #[serde(rename(deserialize = "Federation"))]
    #[serde(default)]
    pub federation: FederationDto,

    #[serde(rename(deserialize = "MeetCountry"))]
    #[serde(default)]
    pub country: CountryDto,

    #[serde(rename(deserialize = "MeetState"))]
    #[serde(default)]
    pub state: String,

    #[serde(rename(deserialize = "MeetTown"))]
    #[serde(default)]
    pub town: String,

    #[serde(rename(deserialize = "MeetName"))]
    #[serde(default)]
    pub name: String,
}

impl MatchesQuery for MeetData {
    fn matches_query(&self, query: &Query) -> bool {
        if self.federation.matches_query(query) {
            return false;
        }

        if self.country.matches_query(query) {
            return false;
        }

        return true;
    }
}

impl From<MeetData> for MeetDataDto {
    fn from(value: MeetData) -> Self {
        Self {
            name: value.name,
            federation: value.federation,
            country: value.country,
            state: value.state,
            town: value.town,
        }
    }
}
