use crate::{EntryDto, MeetDataDto};

pub struct MeetDto {
    pub data: MeetDataDto,
    pub entries: Vec<EntryDto>,
}
