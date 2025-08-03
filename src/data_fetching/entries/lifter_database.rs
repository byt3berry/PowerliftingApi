use itertools::Itertools;
use std::ops::Deref;
use std::str::FromStr;

use crate::api::powerlifters::PowerlifterForm;
use crate::data_fetching::entries::export_row::ExportRow;
use crate::data_fetching::entries::meet_database::MeetDatabase;
use crate::data_fetching::entries::search_result::SearchResult;
use crate::data_fetching::types::lifter::Lifter;
use crate::data_fetching::types::meet_entry::MeetEntry;
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
            .map(|chunk| Lifter::from(chunk.1))
            .collect();

        Self(lifters)
    }
}

impl LifterDatabase {
    fn from(database: &MeetDatabase) -> Self {
        let lifters: Vec<Lifter> = database
            .iter()
            .sorted_by_key(|entry| &entry.name)
            .chunk_by(|entry| &entry.name)
            .into_iter()
            .map(|chunk| Lifter::from(chunk.1))
            .collect();

        Self(lifters)
    }

    pub fn search_export(&self, form: &PowerlifterForm) -> Vec<ExportRow> {
        self
            .search(form)
            .into_iter()
            .map(ExportRow::from)
            .collect()
    }

    fn search(&self, form: &PowerlifterForm) -> Vec<SearchResult> {
        let lifters: Vec<&MeetEntry>  = self
            .iter()
            .filter_map(|lifter| lifter.best_total(form))
            .sorted_by_key(|lifter| lifter.total)
            .rev()
            // .inspect(|meet_entry| println!("{} {}", meet_entry.total, meet_entry.name.name))
            .collect();

        form
            .powerlifters
            .lines()
            .map(Username::from_str)
            .filter_map(Result::ok)
            .map(|powerlifter| {
                if let Some((rank, meet)) = lifters
                    .clone()
                        .into_iter()
                        .find_position(|lifter| lifter.name == powerlifter)
                {
                    (powerlifter, Some((rank+1, meet)))
                } else {
                    (powerlifter, None)
                }
            })
            .map(SearchResult::from)
            .collect()
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
    use crate::data_fetching::entries::search_result::SearchResult;
    use crate::data_fetching::types::division::Division;
    use crate::data_fetching::types::equipment::Equipment;
    use crate::data_fetching::types::meet_entry::MeetEntry;
    use crate::data_fetching::types::sex::Sex;
    use crate::data_fetching::types::username::Username;
    use crate::data_fetching::types::weight::Weight;
    use crate::data_fetching::types::weight_class::WeightClass;

    use super::LifterDatabase;

    const TEST_PATH: &str = "tests/data_fetching/entries/lifter_database";

    #[test]
    fn test_search_1_not_found() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test1");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Masters,
            powerlifters: "Powerlifter".to_string(),
        };
        let expected: SearchResult = SearchResult {
            name: Username::from_str("Powerlifter")?,
            rank: None,
            meet_entry: None,
        };

        let lifter: SearchResult = lifter_database.search(&form).first().unwrap().clone();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_1_meet_entry() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test1");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Masters,
            powerlifters: "FirstName LastName".to_string(),
        };
        let expected_meet: MeetEntry = MeetEntry {
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
        let expected: Option<MeetEntry> = Some(expected_meet);

        let lifter: Option<MeetEntry> = lifter_database.search(&form).first().unwrap().meet_entry.cloned();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_2_meet_entry() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test2");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Masters,
            powerlifters: "LastName FirstName".to_string(),
        };
        let expected_meet: MeetEntry = MeetEntry {
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
        let expected: Option<MeetEntry> = Some(expected_meet);

        let lifter: Option<MeetEntry> = lifter_database.search(&form).first().unwrap().meet_entry.cloned();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_3_meet_entry() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test3");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 2".to_string(),
        };
        let expected_meet: MeetEntry = MeetEntry {
            name: Username::from_str("Powerlifter 2")?,
            division: Division::Juniors,
            equipment: Equipment::Raw,
            sex: Sex::M,
            bodyweight: Weight(80.2),
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
        let expected: Option<MeetEntry> = Some(expected_meet);

        let lifter: Option<MeetEntry> = lifter_database.search(&form).first().unwrap().meet_entry.cloned();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_4_meet_entry() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test4");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 2".to_string(),
        };
        let expected_meet: MeetEntry = MeetEntry {
            name: Username::from_str("Powerlifter 2")?,
            division: Division::Juniors,
            equipment: Equipment::Raw,
            sex: Sex::M,
            bodyweight: Weight(80.2),
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
        let expected: Option<MeetEntry> = Some(expected_meet);

        let lifter: Option<MeetEntry> = lifter_database.search(&form).first().unwrap().meet_entry.cloned();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_5_rank_1() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test5");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 1".to_string(),
        };
        let expected: Option<usize> = Some(3);

        let lifter: Option<usize> = lifter_database.search(&form).first().unwrap().rank;

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_5_rank_2() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test5");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 2".to_string(),
        };
        let expected: Option<usize> = Some(2);

        let lifter: Option<usize> = lifter_database.search(&form).first().unwrap().rank;

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_5_rank_3() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test5");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 3".to_string(),
        };
        let expected: Option<usize> = Some(1);

        let lifter: Option<usize> = lifter_database.search(&form).first().unwrap().rank;

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_6_rank_1() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test6");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 1".to_string(),
        };
        let expected: Option<usize> = Some(2);

        let lifter: Option<usize> = lifter_database.search(&form).first().unwrap().rank;

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_6_rank_2() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test6");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Masters,
            powerlifters: "Powerlifter 2".to_string(),
        };
        let expected: Option<usize> = Some(1);

        let lifter: Option<usize> = lifter_database.search(&form).first().unwrap().rank;

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_6_rank_3() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test6");
        let meet_database: MeetDatabase = MeetDatabase::from_folder(&test_path)?.into();
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);
        let form: PowerlifterForm = PowerlifterForm {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 3".to_string(),
        };
        let expected: Option<usize> = Some(1);

        let lifter: Option<usize> = lifter_database.search(&form).first().unwrap().rank;

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

        let meet_database: MeetDatabase = MeetDatabase::from_folder(&path.to_path_buf())?.into();

        let now: Instant = Instant::now();
        let _ = LifterDatabase::from(&meet_database);
        let elapsed: Duration = now.elapsed();

        assert!(elapsed.as_millis() < 100, "parsing too long: {}ms", elapsed.as_millis());
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
        let _ = lifter_database.search(&form);
        let elapsed: Duration = now.elapsed();

        assert!(elapsed.as_millis() < 10, "searching too long: {}µs", elapsed.as_micros());
        Ok(())
    }
}
