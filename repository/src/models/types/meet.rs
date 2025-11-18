use sea_orm::ActiveValue::Set;

use crate::models::{SeaActiveMeet, SeaMeet};
use crate::models::types::country::Country;
use crate::models::types::federation::Federation;

pub struct Meet {
    pub name: String,
    pub federation: Federation,
    pub country: Country,
    pub state: String,
    pub town: String,
}

impl From<types::Meet> for Meet {
    fn from(meet: types::Meet) -> Self {
        Self {
            name: meet.name.clone(),
            federation: meet.federation.into(),
            country: meet.country.into(),
            state: meet.state.clone(),
            town: meet.town.clone(),
        }
    }
}

impl From<SeaMeet> for Meet {
    fn from(value: SeaMeet) -> Self {
        Self {
            federation: value.federation.into(),
            country: value.country.into(),
            state: value.state.into(),
            town: value.town.into(),
            name: value.name.into(),
        }
    }
}

impl Into<SeaActiveMeet> for Meet {
    fn into(self) -> SeaActiveMeet {
        SeaActiveMeet {
            federation: Set(self.federation.into()),
            country: Set(self.country.into()),
            state: Set(self.state.into()),
            town: Set(self.town.into()),
            name: Set(self.name.into()),
            ..Default::default()
        }
    }
}
