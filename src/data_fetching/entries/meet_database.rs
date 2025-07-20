use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::data_fetching::types::meet_entry::MeetEntry;

struct MeetDatabase;

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

    pub fn from_folder(meet_folder: &Path) -> Vec<MeetEntry> {
        todo!()
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
    fn test_from_csv_no_error_body_weight_class() {
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
                age: 48,
                sex: Sex::M,
                bodyweight: Weight(45.),
                weight_class: Some(WeightClass::UnderOrEqual(Weight(47.))),
                squat1: Weight(55.),
                squat2: Weight(60.),
                squat3: Weight(65.),
                best3squat: Weight(65.),
                bench1: Weight(30.),
                bench2: Weight(32.5),
                bench3: Weight(35.),
                best3bench: Weight(35.),
                deadlift1: Weight(60.),
                deadlift2: Weight(70.),
                deadlift3: Weight(75.),
                best3deadlift: Weight(75.),
                total: Weight(175.),
            }
        ];

        let meets: Vec<MeetEntry> = MeetDatabase::from_csv(&test_file).unwrap();

        assert_eq!(expected, meets);
    }
}
