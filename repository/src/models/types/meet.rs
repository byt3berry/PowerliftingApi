use sea_orm::ActiveValue::Set;
use types::prelude::*;

use crate::models::types::{Country, Federation};
use crate::models::{SeaActiveMeet, SeaMeet};

pub struct Meet {
    pub name: String,
    pub federation: Federation,
    pub country: Country,
    pub state: String,
    pub town: String,
}

impl From<MeetDataDto> for Meet {
    fn from(meet: MeetDataDto) -> Self {
        Self {
            name: meet.name,
            federation: meet.federation.into(),
            country: meet.country.into(),
            state: meet.state,
            town: meet.town,
        }
    }
}

impl From<SeaMeet> for Meet {
    fn from(value: SeaMeet) -> Self {
        Self {
            name: value.name,
            federation: value.federation.into(),
            country: value.country.into(),
            state: value.state,
            town: value.town,
        }
    }
}

impl From<Meet> for SeaActiveMeet {
    fn from(value: Meet) -> Self {
        Self {
            name: Set(value.name),
            federation: Set(value.federation.into()),
            country: Set(value.country.into()),
            state: Set(value.state),
            town: Set(value.town),
            ..Default::default()
        }
    }
}
