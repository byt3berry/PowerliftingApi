use anyhow::Result;
use itertools::Itertools;
use types::Username;
use std::path::PathBuf;
use std::str::FromStr;

use crate::data::meet_entry::MeetEntry;
use crate::databases::lifter_database::LifterDatabase;
use crate::databases::meet_database::MeetDatabase;
use crate::entries::search_result::SearchResult;
use crate::{ExportRow, Query};

#[derive(Clone, Debug)]
pub struct Database {
    meets: MeetDatabase,
    lifters: LifterDatabase,
}

impl Database {
    pub fn from_directory(path: &PathBuf) -> Result<Self> {
        let meet_database: MeetDatabase = MeetDatabase::from_directory(path)?;
        let lifter_database: LifterDatabase = LifterDatabase::from(&meet_database);

        Ok(Self {
            meets: meet_database,
            lifters: lifter_database,
        })
    }

    #[must_use]
    pub const fn new(meets: MeetDatabase, lifters: LifterDatabase) -> Self {
        Self {
            meets,
            lifters,
        }
    }

    pub fn search_export(&self, form: &Query) -> Vec<ExportRow> {
        self
            .search(form)
            .into_iter()
            .map(ExportRow::from)
            .collect()
    }

    fn search(&self, form: &Query) -> Vec<SearchResult> {
        let lifters: Vec<&MeetEntry>  = self
            .lifters
            .iter()
            .filter_map(|lifter| lifter.best_total(form))
            .sorted_by_key(|lifter| lifter.total)
            .rev()
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

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;
    use types::{Division, Equipment, Sex, Username, Weight, WeightClass};
                    
    use crate::data::meet_entry::MeetEntry;
    use crate::data::query::Query;
    use crate::entries::search_result::SearchResult;

    use super::Database;

    const TEST_PATH: &str = "test_data/entries/database";

    #[test]
    fn test_search_1_not_found() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test1");
        let database: Database = Database::from_directory(&test_path).unwrap();
        let form: Query = Query {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Masters,
            powerlifters: "Powerlifter".to_string(),
        };
        let expected: SearchResult = SearchResult {
            name: Username::from_str("Powerlifter").unwrap(),
            rank: None,
            meet_entry: None,
        };

        let lifter: SearchResult = database.search(&form).first().unwrap().clone();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_1_meet_entry() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test1");
        let database: Database = Database::from_directory(&test_path).unwrap();
        let form: Query = Query {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Masters,
            powerlifters: "FirstName LastName".to_string(),
        };
        let expected_meet: MeetEntry = MeetEntry {
            name: Username::from_str("FirstName LastName").unwrap(),
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

        let lifter: Option<MeetEntry> = database.search(&form).first().unwrap().meet_entry.cloned();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_2_meet_entry() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test2");
        let database: Database = Database::from_directory(&test_path).unwrap();
        let form: Query = Query {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Masters,
            powerlifters: "LastName FirstName".to_string(),
        };
        let expected_meet: MeetEntry = MeetEntry {
            name: Username::from_str("FirstName LastName").unwrap(),
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

        let lifter: Option<MeetEntry> = database.search(&form).first().unwrap().meet_entry.cloned();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_3_meet_entry() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test3");
        let database: Database = Database::from_directory(&test_path).unwrap();
        let form: Query = Query {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::M,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 2".to_string(),
        };
        let expected_meet: MeetEntry = MeetEntry {
            name: Username::from_str("Powerlifter 2").unwrap(),
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

        let lifter: Option<MeetEntry> = database.search(&form).first().unwrap().meet_entry.cloned();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_4_meet_entry() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test4");
        let database: Database = Database::from_directory(&test_path).unwrap();
        let form: Query = Query {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 2".to_string(),
        };
        let expected_meet: MeetEntry = MeetEntry {
            name: Username::from_str("Powerlifter 2").unwrap(),
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

        let lifter: Option<MeetEntry> = database.search(&form).first().unwrap().meet_entry.cloned();

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_5_rank_1() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test5");
        let database: Database = Database::from_directory(&test_path).unwrap();
        let form: Query = Query {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 1".to_string(),
        };
        let expected: Option<usize> = Some(3);

        let lifter: Option<usize> = database.search(&form).first().unwrap().rank;

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_5_rank_2() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test5");
        let database: Database = Database::from_directory(&test_path).unwrap();
        let form: Query = Query {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 2".to_string(),
        };
        let expected: Option<usize> = Some(2);

        let lifter: Option<usize> = database.search(&form).first().unwrap().rank;

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_5_rank_3() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test5");
        let database: Database = Database::from_directory(&test_path).unwrap();
        let form: Query = Query {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 3".to_string(),
        };
        let expected: Option<usize> = Some(1);

        let lifter: Option<usize> = database.search(&form).first().unwrap().rank;

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_6_rank_1() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test6");
        let database: Database = Database::from_directory(&test_path).unwrap();
        let form: Query = Query {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 1".to_string(),
        };
        let expected: Option<usize> = Some(2);

        let lifter: Option<usize> = database.search(&form).first().unwrap().rank;

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_6_rank_2() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test6");
        let database: Database = Database::from_directory(&test_path).unwrap();
        let form: Query = Query {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Masters,
            powerlifters: "Powerlifter 2".to_string(),
        };
        let expected: Option<usize> = Some(1);

        let lifter: Option<usize> = database.search(&form).first().unwrap().rank;

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_search_6_rank_3() -> Result<()> {
        let test_path: PathBuf = Path::new(TEST_PATH).join("test6");
        let database: Database = Database::from_directory(&test_path).unwrap();
        let form: Query = Query {
            equipment_choice: Equipment::Raw,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: "Powerlifter 3".to_string(),
        };
        let expected: Option<usize> = Some(1);

        let lifter: Option<usize> = database.search(&form).first().unwrap().rank;

        assert_eq!(lifter, expected);
        Ok(())
    }
}
