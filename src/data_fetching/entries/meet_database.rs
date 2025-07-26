use anyhow::{bail, Result};
use csv::Reader;
use log::warn;
use std::fs::File;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::data_fetching::types::meet_entry::MeetEntry;

#[derive(Clone, Debug)]
pub struct MeetDatabase(Vec<MeetEntry>);

impl MeetDatabase {
    fn from_csv(csv: &PathBuf) -> Result<Vec<MeetEntry>> {
        let mut entries: Vec<MeetEntry> = Vec::with_capacity(50_000);

        let mut reader: Reader<File> = csv::ReaderBuilder::new().quoting(false).from_path(csv)?;
        for entry in reader.deserialize() {
            let entry: MeetEntry = entry?;
            entries.push(entry);
        }

        entries.shrink_to_fit();
        Ok(entries)
    }

    pub fn from_folder(meet_folder: &PathBuf) -> Result<Self> {
        if !meet_folder.is_dir() {
            bail!("{meet_folder:?} should be a folder");
        }

        let filename: &Path = Path::new("entries.csv");
        let entries: Vec<MeetEntry> = WalkDir::new(meet_folder)
            .into_iter()
            .filter_map(Result::ok)
            .map(|element| element.path().to_owned())
            .filter(|path| path.is_file())
            .filter(|path| path.file_name().unwrap() == filename)
            .map(|path| {
                let result: Result<Vec<MeetEntry>> = Self::from_csv(&path);

                if result.is_err() {
                    warn!("file {} can't be used: {:?}", path.display(), result.as_ref().err().unwrap());
                }

                result
            })
            .filter_map(Result::ok)
            .flatten()
            .collect();

        Ok(Self(entries))
    }
}

impl Deref for MeetDatabase {
    type Target = Vec<MeetEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::path::{Path, PathBuf};

    use crate::data_fetching::types::division::Division;
    use crate::data_fetching::types::equipment::Equipment;
    use crate::data_fetching::types::meet_entry::MeetEntry;
    use crate::data_fetching::types::sex::Sex;
    use crate::data_fetching::types::weight::Weight;
    use crate::data_fetching::types::weight_class::WeightClass;

    use super::MeetDatabase;

    const TEST_PATH: &str = "tests/data_fetching/entries/meet_database";

    #[test]
    fn test_from_csv_no_error_simple() {
        let test_file: PathBuf = Path::new(TEST_PATH).join("test1.csv");

        MeetDatabase::from_csv(&test_file).unwrap();
    }

    #[test]
    fn test_from_csv_no_error_divisions() {
        let test_file: PathBuf = Path::new(TEST_PATH).join("test3.csv");

        MeetDatabase::from_csv(&test_file).unwrap();
    }

    #[test]
    fn test_from_csv_no_error_no_weight_class() {
        let test_file: PathBuf = Path::new(TEST_PATH).join("test4.csv");

        MeetDatabase::from_csv(&test_file).unwrap();
    }

    #[test]
    fn test_from_csv_integer() {
        let test_file: PathBuf = Path::new(TEST_PATH).join("test1.csv");
        let expected: Vec<MeetEntry> = vec![
            MeetEntry {
                name: String::from("Test Powerlifter"),
                division: Division::Masters3,
                equipment: Equipment::Raw,
                age: Some(48),
                sex: Sex::M,
                bodyweight: Weight(45.),
                weight_class: Some(WeightClass::UnderOrEqual(Weight(47.))),
                squat1: Some(Weight(55.)),
                squat2: Some(Weight(60.)),
                squat3: Some(Weight(65.)),
                best3squat: Some(Weight(65.)),
                bench1: Some(Weight(30.)),
                bench2: Some(Weight(32.)),
                bench3: Some(Weight(35.)),
                best3bench: Some(Weight(35.)),
                deadlift1: Some(Weight(60.)),
                deadlift2: Some(Weight(70.)),
                deadlift3: Some(Weight(75.)),
                best3deadlift: Some(Weight(75.)),
                total: Weight(175.),
            }
        ];

        let meets: Vec<MeetEntry> = MeetDatabase::from_csv(&test_file).unwrap();

        assert_eq!(expected, meets);
    }

    #[test]
    fn test_from_csv_float() {
        let test_file: PathBuf = Path::new(TEST_PATH).join("test2.csv");
        let expected: Vec<MeetEntry> = vec![
            MeetEntry {
                name: String::from("Test Powerlifter"),
                division: Division::Masters3,
                equipment: Equipment::Raw,
                age: Some(48),
                sex: Sex::M,
                bodyweight: Weight(45.3),
                weight_class: Some(WeightClass::UnderOrEqual(Weight(47.))),
                squat1: Some(Weight(55.1)),
                squat2: Some(Weight(60.2)),
                squat3: Some(Weight(65.3)),
                best3squat: Some(Weight(65.3)),
                bench1: Some(Weight(30.4)),
                bench2: Some(Weight(32.5)),
                bench3: Some(Weight(35.6)),
                best3bench: Some(Weight(35.6)),
                deadlift1: Some(Weight(60.7)),
                deadlift2: Some(Weight(70.8)),
                deadlift3: Some(Weight(75.9)),
                best3deadlift: Some(Weight(75.9)),
                total: Weight(176.8),
            }
        ];

        let meets: Vec<MeetEntry> = MeetDatabase::from_csv(&test_file).unwrap();

        assert_eq!(expected, meets);
    }
}

#[cfg(test)]
mod perf_tests {
    use std::path::Path;
    use std::time::{Duration, Instant};

    use super::MeetDatabase;

    const ENTRIES_ROOT: &str = "/tmp/opl-data/meet-data/ffforce";

    #[test]
    #[ignore = "benchmark test, run only in release mode with `cargo run perf --release -- --ignored`"]
    fn perf_load_ffforce_entries() {
        let path: &Path = Path::new(ENTRIES_ROOT);
        assert!(path.is_dir());

        let now: Instant = Instant::now();
        MeetDatabase::from_folder(&path.to_path_buf()).unwrap();
        let elapsed: Duration = now.elapsed();

        assert!(elapsed.as_millis() < 100, "parsing too long: {}ms", elapsed.as_millis());
    }
}
