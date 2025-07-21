use anyhow::{bail, Result};
use log::warn;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};

use crate::data_fetching::types::meet_entry::MeetEntry;

pub struct MeetDatabase;

impl MeetDatabase {
    fn from_csv(csv: &PathBuf) -> Result<Vec<MeetEntry>> {
        let mut entries = Vec::with_capacity(50_000);

        let mut rdr = csv::ReaderBuilder::new().quoting(false).from_path(csv)?;
        for entry in rdr.deserialize() {
            let entry: MeetEntry = entry?;
            entries.push(entry);
        }

        entries.shrink_to_fit();
        Ok(entries)
    }

    pub fn from_folder(meet_folder: &Path) -> Result<Vec<MeetEntry>> {
        if !meet_folder.is_dir() {
            bail!("{meet_folder:?} should be a folder");
        }

        let filename: &Path = Path::new("entries.csv");
        let output: Vec<MeetEntry> = WalkDir::new(meet_folder)
            .into_iter()
            .filter_map(|element| element.ok())
            .map(|element| element.path().to_owned())
            .filter(|path| path.is_file())
            .filter(|path| path.file_name().unwrap() == filename)
            .map(|path| {
                let result = Self::from_csv(&path);

                if result.is_err() {
                    warn!("file {path:?} can't be used: {:?}", result.as_ref().err().unwrap());
                }

                result
            })
            .filter_map(|element| element.ok())
            .flatten()
            .collect();

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::path::{Path, PathBuf};

    use crate::data_fetching::types::weight_class::WeightClass;
    use crate::data_fetching::types::weight::Weight;
    use crate::data_fetching::types::sex::Sex;
    use crate::data_fetching::types::meet_entry::MeetEntry;
    use crate::data_fetching::types::equipment::Equipment;
    use crate::data_fetching::types::division::Division;
    use crate::data_fetching::entries::meet_database::MeetDatabase;

    const TEST_PATH: &str = "tests/data_fetching/entries/meet_database";

    #[test]
    fn test_from_csv_no_error_simple() {
        let test_file: PathBuf = Path::new(TEST_PATH).join("test1.csv");

        MeetDatabase::from_csv(&test_file).unwrap();
    }

    #[test]
    fn test_from_csv_no_error_divisions() {
        let test_file: PathBuf = Path::new(TEST_PATH).join("test2.csv");

        MeetDatabase::from_csv(&test_file).unwrap();
    }

    #[test]
    fn test_from_csv_no_error_no_weight_class() {
        let test_file: PathBuf = Path::new(TEST_PATH).join("test3.csv");

        MeetDatabase::from_csv(&test_file).unwrap();
    }

    #[test]
    fn test_from_csv_simple() {
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
                bench2: Some(Weight(32.5)),
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
}

#[cfg(test)]
mod perf_tests {
    use std::path::Path;
    use std::time::{Duration, Instant};

    use crate::data_fetching::entries::meet_database::MeetDatabase;

    const ENTRIES_ROOT: &str = "/tmp/opl-data/meet-data/ffforce";
    
    #[test]
    #[ignore = "benchmark test, run only in release mode"]
    fn perf_load_ffforce_entries() {
        let path: &Path = Path::new(ENTRIES_ROOT);
        assert!(path.is_dir());

        let now = Instant::now();
        MeetDatabase::from_folder(path).unwrap();
        let elapsed: Duration = now.elapsed();

        assert!(elapsed.as_millis() < 100, "parsing too long: {}ms", elapsed.as_millis());
    }
}
