use types::prelude::*;

use crate::types::MeetData;
use crate::types::Entry;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Meet {
    pub data: MeetData,
    pub entries: Vec<Entry>,
}

impl Meet {
    #[must_use]
    pub const fn new(data: MeetData, entries: Vec<Entry>) -> Self {
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
