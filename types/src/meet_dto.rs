use crate::{CountryDto, FederationDto};

pub struct MeetDto {
    pub name: String,
    pub federation: FederationDto,
    pub country: CountryDto,
    pub state: String,
    pub town: String,
}
