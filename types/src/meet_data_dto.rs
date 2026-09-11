use chrono::NaiveDate;

use crate::prelude::{CountryDto, FederationDto};

pub struct MeetDataDto {
    pub name: String,
    pub date: NaiveDate,
    pub federation: FederationDto,
    pub country: CountryDto,
    pub state: String,
    pub town: String,
}
