use types::prelude::*;

use crate::data::meet_data::MeetData;
use crate::data::meet_entry::MeetEntry;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Meet {
    pub data: MeetData,
    pub entries: Vec<MeetEntry>,
}

impl Meet {
    #[must_use]
    pub const fn new(data: MeetData, entries: Vec<MeetEntry>) -> Self {
        Self { data, entries }
    }
}

impl From<Meet> for MeetDto {
    fn from(value: Meet) -> Self {
        Self {
            data: value.data.into(),
            entries: value.entries.into_iter().map(EntryDto::from).collect(),
        }
    }
}
