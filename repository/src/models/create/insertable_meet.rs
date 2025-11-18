use types::Meet;

use crate::models::types::country::Country;
use crate::models::types::federation::Federation;

pub struct InsertableMeet {
    name: String,
    federation: Federation,
    country: Country,
    state: String,
    town: String,
}

impl InsertableMeet {
    pub fn from(meet: &Meet) -> Self {
        Self {
            name: meet.name.clone(),
            federation: meet.federation.into(),
            country: meet.country.into(),
            state: meet.state.clone(),
            town: meet.town.clone(),
        }
    }
}
