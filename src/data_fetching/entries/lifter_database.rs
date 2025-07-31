use itertools::Itertools;
use std::ops::Deref;
use std::str::FromStr;

use crate::api::powerlifters::PowerlifterForm;
use crate::data_fetching::entries::export_row::ExportRow;
use crate::data_fetching::entries::meet_database::MeetDatabase;
use crate::data_fetching::types::lifter::Lifter;
use crate::data_fetching::types::username::Username;

#[derive(Clone, Debug)]
pub struct LifterDatabase(Vec<Lifter>);

impl From<&MeetDatabase> for LifterDatabase {
    fn from(database: &MeetDatabase) -> Self {
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
    pub fn search_many(&self, form: &PowerlifterForm) -> Vec<ExportRow> {
        form
            .powerlifters
            .lines()
            .map(|powerlifter| (powerlifter.to_string(), self.search_one(powerlifter, form)))
            .map(ExportRow::from)
            .collect()
    }

    fn search_one(&self, name: &str, form: &PowerlifterForm) -> Option<&Lifter> {
        let name: Username = Username::from_str(name).ok()?;

        self.0
            .iter()
            .filter(|lifter| form.equipment_choice == lifter.equipment)
            .filter(|lifter| form.sex_choice == lifter.sex)
            .filter(|lifter| form.division_choice == lifter.division)
            .find(|lifter| name == lifter.name)
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
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;

    use crate::api::powerlifters::PowerlifterForm;
    use crate::data_fetching::entries::meet_database::MeetDatabase;
    use crate::data_fetching::types::division::Division;
    use crate::data_fetching::types::equipment::Equipment;
    use crate::data_fetching::types::lifter::Lifter;
    use crate::data_fetching::types::meet_entry::MeetEntry;
    use crate::data_fetching::types::sex::Sex;
    use crate::data_fetching::types::username::Username;
    use crate::data_fetching::types::weight::Weight;
    use crate::data_fetching::types::weight_class::WeightClass;

    use super::LifterDatabase;

    const TEST_PATH: &str = "tests/data_fetching/entries/lifter_database";

    #[test]
    fn test_search_one_1_found() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test1");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let name: &str = "FirstName LastName";
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Masters,
            powerlifters: "FirstName LastName".to_string(),
        };
        let best_meet: MeetEntry = MeetEntry {
            name: Username::from_str("FirstName LastName")?,
            division: Division::Masters,
            equipment: Equipment::Raw,
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
        };
        let expected: Option<Lifter> = Some(Lifter {
            name: Username::from_str("FirstName LastName")?,
            equipment: Equipment::Raw,
            sex: Sex::M,
            division: Division::Masters,
            best_meet,
        });

        let lifter: Option<Lifter> = lifter_database.search_one(&name, &form).cloned();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_one_1_not_found() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test1");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let name: &str = "Powerlifter";
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Masters,
            powerlifters: "Powerlifter".to_string(),
        };
        let expected: Option<Lifter> = None;

        let lifter: Option<Lifter> = lifter_database.search_one(name, &form).cloned();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_one_2() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test2");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let name: &str = "LastName FirstName";
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Masters,
            powerlifters: "LastName FirstName".to_string(),
        };
        let best_meet: MeetEntry = MeetEntry {
            name: Username::from_str("FirstName LastName")?,
            division: Division::Masters1,
            equipment: Equipment::Raw,
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
        };
        let expected: Option<Lifter> = Some(Lifter {
            name: Username::from_str("FirstName LastName")?,
            equipment: Equipment::Raw,
            sex: Sex::M,
            division: Division::Masters1,
            best_meet,
        });

        let lifter: Option<Lifter> = lifter_database.search_one(name, &form).cloned();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_one_3() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test3");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let name: &str = "Powerlifter 2";
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 2".to_string(),
        };
        let best_meet: MeetEntry = MeetEntry {
            name: Username::from_str("Powerlifter 2")?,
            division: Division::Juniors,
            equipment: Equipment::Raw,
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
        };
        let expected: Option<Lifter> = Some(Lifter {
            name: Username::from_str("Powerlifter 2")?,
            equipment: Equipment::Raw,
            sex: Sex::M,
            division: Division::Juniors,
            best_meet,
        });

        let lifter: Option<Lifter> = lifter_database.search_one(name, &form).cloned();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_one_4() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test4");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let name: &str = "Powerlifter 2";
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 2".to_string(),
        };
        let best_meet: MeetEntry = MeetEntry {
            name: Username::from_str("Powerlifter 2")?,
            division: Division::Juniors,
            equipment: Equipment::Raw,
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
        };
        let expected: Option<Lifter> = Some(Lifter {
            name: Username::from_str("Powerlifter 2")?,
            equipment: Equipment::Raw,
            sex: Sex::M,
            division: Division::Juniors,
            best_meet,
        });

        let lifter: Option<Lifter> = lifter_database.search_one(name, &form).cloned();

        assert_eq!(lifter, expected);
        Ok(())
    }
}

#[cfg(test)]
mod perf_tests {
    use anyhow::Result;
    use std::path::Path;
    use std::time::{Duration, Instant};

    use crate::api::powerlifters::PowerlifterForm;
    use crate::data_fetching::entries::meet_database::MeetDatabase;
    use crate::data_fetching::types::division::Division;
    use crate::data_fetching::types::equipment::Equipment;
    use crate::data_fetching::types::sex::Sex;

    use super::LifterDatabase;

    const ENTRIES_ROOT: &str = "/tmp/opl-data/meet-data/ffforce";

    #[test]
    #[ignore = "benchmark test, run only in release mode with `cargo run perf --release -- --ignored`"]
    fn perf_load_ffforce_entries() -> Result<()> {
        let path: &Path = Path::new(ENTRIES_ROOT);
        assert!(path.is_dir());

        let now: Instant = Instant::now();
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&path.to_path_buf())?.into();
        let _ = LifterDatabase::from(&meet_database);
        let elapsed: Duration = now.elapsed();

        assert!(elapsed.as_millis() < 1000, "parsing too long: {}ms", elapsed.as_millis());
        Ok(())
    }

    #[test]
    #[ignore = "benchmark test, run only in release mode with `cargo run perf --release -- --ignored`"]
    fn perf_search_ffforce_lifter() -> Result<()> {
        let path: &Path = Path::new(ENTRIES_ROOT);
        assert!(path.is_dir());
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&path.to_path_buf())?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let form: PowerlifterForm = PowerlifterForm { 
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Any,
            powerlifters: lifter_database[lifter_database.len()-1].name.to_string(),
        };

        let now: Instant = Instant::now();
        let _ = lifter_database.search_many(&form);
        let elapsed: Duration = now.elapsed();

        assert!(elapsed.as_millis() < 10, "searching too long: {}µs", elapsed.as_micros());
        Ok(())
    }
}
