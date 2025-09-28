use itertools::Itertools;
use std::ops::Deref;

use crate::data::lifter::Lifter;
use crate::databases::meet_database::MeetDatabase;

#[derive(Clone, Debug)]
pub struct LifterDatabase(Vec<Lifter>);

impl From<&MeetDatabase> for LifterDatabase {
    fn from(database: &MeetDatabase) -> Self {
        let lifters: Vec<Lifter> = database
            .iter()
            .flat_map(|meet| meet.entries.clone())
            .sorted_by_key(|entry| entry.name.clone())
            .chunk_by(|entry| entry.name.clone())
            .into_iter()
            .map(|chunk| Lifter::new(chunk.0.clone(), chunk.1.collect()))
            .collect();

        Self(lifters)
    }
}

impl Deref for LifterDatabase {
    type Target = Vec<Lifter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
