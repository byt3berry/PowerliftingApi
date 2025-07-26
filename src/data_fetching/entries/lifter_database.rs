use anyhow::Result;
use itertools::Itertools;
use std::ops::Deref;
use std::path::PathBuf;

use crate::api::powerlifters::PowerlifterForm;
use crate::data_fetching::entries::meet_database::MeetDatabase;
use crate::data_fetching::types::lifter::Lifter;

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

    pub fn search_many(&self, form: &PowerlifterForm) -> Vec<(String, Option<Lifter>)> {
        form
            .powerlifters
            .lines()
            .map(|powerlifter| (powerlifter.to_string(), self.search_one(powerlifter, form)))
            .collect()
    }

    fn search_one(&self, name: &str, form: &PowerlifterForm) -> Option<Lifter> {
        let name = name.to_lowercase();
        let name_backward: String = name
            .split_whitespace()
            .rev()
            .join(" ");

        for lifter in &self.0 {
            if form.equipment_choice != lifter.equipment { continue; }
            if form.sex_choice != lifter.sex { continue; }
            if form.division_choice != lifter.division { continue; }

            if lifter.name.to_lowercase() == name || lifter.name.to_lowercase() == name_backward {
                return Some(lifter.clone());
            }
        }

        None
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
    use pretty_assertions::assert_eq;
    use std::path::{Path, PathBuf};

    use crate::api::powerlifters::PowerlifterForm;
    use crate::data_fetching::types::division::Division;
    use crate::data_fetching::types::equipment::Equipment;
    use crate::data_fetching::types::lifter::Lifter;
    use crate::data_fetching::types::meet_entry::MeetEntry;
    use crate::data_fetching::types::sex::Sex;
    use crate::data_fetching::types::weight::Weight;
    use crate::data_fetching::types::weight_class::WeightClass;

    use super::LifterDatabase;

    const TEST_PATH: &str = "tests/data_fetching/entries/lifter_database";

    #[test]
    fn test_search_one_1_found() {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test1");
        let database: LifterDatabase = LifterDatabase::from_folder(&test_path).unwrap();
        let name: &str = "FirstName LastName";
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Masters,
            powerlifters: name.to_string(),
        };
        let expected: Option<Lifter> = Some(Lifter {
            name: "FirstName LastName".to_string(),
            equipment: Equipment::Raw,
            sex: Sex::M,
            division: Division::Masters,
            best_meet: MeetEntry {
                name: "FirstName LastName".to_string(),
                division: Division::Masters,
                equipment: Equipment::Raw,
                age: Some(48),
                sex: Sex::M,
                bodyweight: Weight(104.),
                weight_class: WeightClass::UnderOrEqual(Weight(105.)).into(),
                squat1: Weight(1.).into(),
                squat2: Weight(2.).into(),
                squat3: Weight(3.).into(),
                best3squat: Weight(3.).into(),
                bench1: Weight(4.).into(),
                bench2: Weight(5.).into(),
                bench3: Weight(6.).into(),
                best3bench: Weight(6.).into(),
                deadlift1: Weight(7.).into(),
                deadlift2: Weight(8.).into(),
                deadlift3: Weight(9.).into(),
                best3deadlift: Weight(9.).into(),
                total: Weight(18.),
            },
        });

        let lifter: Option<Lifter> = database.search_one(name, &form);

        assert_eq!(lifter, expected);
    }

    #[test]
    fn test_search_one_1_not_found() {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test1");
        let database: LifterDatabase = LifterDatabase::from_folder(&test_path).unwrap();
        let name: &str = "Powerlifter";
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Masters,
            powerlifters: name.to_string(),
        };
        let expected: Option<Lifter> = None;

        let lifter: Option<Lifter> = database.search_one(name, &form);

        assert_eq!(lifter, expected);
    }

    #[test]
    fn test_search_one_2() {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test2");
        let database: LifterDatabase = LifterDatabase::from_folder(&test_path).unwrap();
        let name: &str = "LastName FirstName";
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Masters,
            powerlifters: name.to_string(),
        };
        let expected: Option<Lifter> = Some(Lifter {
            name: "FirstName LastName".to_string(),
            equipment: Equipment::Raw,
            sex: Sex::M,
            division: Division::Masters1,
            best_meet: MeetEntry {
                name: "FirstName LastName".to_string(),
                division: Division::Masters1,
                equipment: Equipment::Raw,
                age: Some(48),
                sex: Sex::M,
                bodyweight: Weight(104.),
                weight_class: WeightClass::UnderOrEqual(Weight(105.)).into(),
                squat1: Weight(1.).into(),
                squat2: Weight(2.).into(),
                squat3: Weight(3.).into(),
                best3squat: Weight(3.).into(),
                bench1: Weight(4.).into(),
                bench2: Weight(5.).into(),
                bench3: Weight(6.).into(),
                best3bench: Weight(6.).into(),
                deadlift1: Weight(7.).into(),
                deadlift2: Weight(8.).into(),
                deadlift3: Weight(9.).into(),
                best3deadlift: Weight(9.).into(),
                total: Weight(18.),
            },
        });

        let lifter: Option<Lifter> = database.search_one(name, &form);

        assert_eq!(lifter, expected);
    }

    #[test]
    fn test_search_one_3() {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test3");
        let database: LifterDatabase = LifterDatabase::from_folder(&test_path).unwrap();
        let name: &str = "Powerlifter 2";
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Juniors,
            powerlifters: name.to_string(),
        };
        let expected: Option<Lifter> = Some(Lifter {
            name: "Powerlifter 2".to_string(),
            equipment: Equipment::Raw,
            sex: Sex::M,
            division: Division::Juniors,
            best_meet: MeetEntry {
                name: "Powerlifter 2".to_string(),
                division: Division::Juniors,
                equipment: Equipment::Raw,
                age: Some(22),
                sex: Sex::M,
                bodyweight: Weight(80.1),
                weight_class: WeightClass::UnderOrEqual(Weight(83.)).into(),
                squat1: Weight(28.).into(),
                squat2: Weight(29.).into(),
                squat3: Weight(30.).into(),
                best3squat: Weight(30.).into(),
                bench1: Weight(31.).into(),
                bench2: Weight(32.).into(),
                bench3: Weight(33.).into(),
                best3bench: Weight(33.).into(),
                deadlift1: Weight(34.).into(),
                deadlift2: Weight(35.).into(),
                deadlift3: Weight(36.).into(),
                best3deadlift: Weight(36.).into(),
                total: Weight(99.),
            },
        });

        let lifter: Option<Lifter> = database.search_one(name, &form);

        assert_eq!(lifter, expected);
    }

    #[test]
    fn test_search_one_4() {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test4");
        let database: LifterDatabase = LifterDatabase::from_folder(&test_path).unwrap();
        let name: &str = "Powerlifter 2";
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::All,
            division_choice: Division::Juniors,
            powerlifters: name.to_string(),
        };
        let expected: Option<Lifter> = Some(Lifter {
            name: "Powerlifter 2".to_string(),
            equipment: Equipment::Raw,
            sex: Sex::M,
            division: Division::Juniors,
            best_meet: MeetEntry {
                name: "Powerlifter 2".to_string(),
                division: Division::Juniors,
                equipment: Equipment::Raw,
                age: Some(22),
                sex: Sex::M,
                bodyweight: Weight(80.1),
                weight_class: WeightClass::UnderOrEqual(Weight(83.)).into(),
                squat1: Weight(28.).into(),
                squat2: Weight(29.).into(),
                squat3: Weight(30.).into(),
                best3squat: Weight(30.).into(),
                bench1: Weight(31.).into(),
                bench2: Weight(32.).into(),
                bench3: Weight(33.).into(),
                best3bench: Weight(33.).into(),
                deadlift1: Weight(34.).into(),
                deadlift2: Weight(35.).into(),
                deadlift3: Weight(36.).into(),
                best3deadlift: Weight(36.).into(),
                total: Weight(99.),
            },
        });

        let lifter: Option<Lifter> = database.search_one(name, &form);

        assert_eq!(lifter, expected);
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
