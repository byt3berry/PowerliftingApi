use anyhow::Result;
use itertools::Itertools;
use std::ops::Deref;
use std::path::PathBuf;

use crate::data_fetching::entries::meet_database::MeetDatabase;
use crate::data_fetching::types::lifter::Lifter;
use crate::data_fetching::types::meet_entry::MeetEntry;

#[derive(Clone, Debug)]
pub struct LifterDatabase(Vec<Lifter>);

impl From<MeetDatabase> for LifterDatabase {
    fn from(database: MeetDatabase) -> Self {
        let lifters: Vec<Lifter> = database
            .iter()
            .sorted_by_key(|entry| &entry.name)
            .chunk_by(|entry| &entry.name)
            .into_iter()
            .map(|chunk| Lifter::from_meet_data(chunk.1))
            .collect();

        Self(lifters)
    }
}

impl LifterDatabase {
    pub fn from_folder(meet_folder: &PathBuf) -> Result<Self> {
        Ok(MeetDatabase::from_folder(meet_folder)?.into())
    }

    pub fn search_many(&self, data: &str) -> Vec<MeetEntry> {
        todo!();
    }

    fn search_one(&self, data: &str) -> MeetEntry {
        todo!();
    }
}

impl Deref for LifterDatabase {
    type Target = Vec<Lifter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    fn test_search_one() {
    }
}

#[cfg(test)]
mod perf_tests {
    use std::path::Path;
    use std::time::{Duration, Instant};

    use super::LifterDatabase;

    const ENTRIES_ROOT: &str = "/tmp/opl-data/meet-data/ffforce";

    #[test]
    #[ignore = "benchmark test, run only in release mode with `cargo run perf --release -- --ignored`"]
    fn perf_load_ffforce_entries() {
        let path: &Path = Path::new(ENTRIES_ROOT);
        assert!(path.is_dir());

        let now: Instant = Instant::now();
        LifterDatabase::from_folder(&path.to_path_buf()).unwrap();
        let elapsed: Duration = now.elapsed();

        assert!(elapsed.as_millis() < 100, "parsing too long: {}ms", elapsed.as_millis());
    }
}
