use types::Username;

use crate::data::meet_entry::MeetEntry;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchResult<'db> {
    pub name: Username,
    pub rank: Option<usize>,
    pub meet_entry: Option<&'db MeetEntry>,
}

impl<'db> From<(Username, Option<(usize, &'db MeetEntry)>)> for SearchResult<'db> {
    fn from((name, result): (Username, Option<(usize, &'db MeetEntry)>)) -> Self {
        if let Some((rank, meet_entry)) = result {
            Self {
                name,
                rank: Some(rank),
                meet_entry: Some(meet_entry),
            }
        } else {
            Self {
                name,
                rank: None,
                meet_entry: None,
            }
        }
    }
}
