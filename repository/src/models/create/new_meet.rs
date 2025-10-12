use diesel::Insertable;
use types::MeetDto;

use crate::models::types::country::Country;
use crate::models::types::federation::Federation;
use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::meets)]
pub struct NewMeet {
    name: String,
    federation: Federation,
    country: Country,
    state: String,
    town: String,
}

impl NewMeet {
    pub fn from(meet: &MeetDto) -> Self {
        Self {
            name: meet.name.clone(),
            federation: meet.federation.into(),
            country: meet.country.into(),
            state: meet.state.clone(),
            town: meet.town.clone(),
        }
    }
}
