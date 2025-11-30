use anyhow::{Result, bail};
use csv::{Reader, ReaderBuilder};
use itertools::Itertools;
use log::warn;
use repository::Repository;
use types::prelude::MeetDto;
use std::fs::File;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::types::Meet;
use crate::types::MeetData;
use crate::types::Entry;

const ENTRIES_FILE_NAME: &str = "entries";
const MEET_FILE_NAME: &str = "meet";
const CSV_EXTENSION: &str = "csv";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Database(Vec<Meet>);

impl Database {
    fn from_data_csv(path: &PathBuf) -> Result<MeetData> {
        if !path.exists() {
            bail!("path \"{}\" should exist", path.display());
        }

        if !path.is_file() {
            bail!("path \"{}\" should be a file", path.display());
        }

        if path.extension().unwrap() != CSV_EXTENSION {
            bail!("extension of \"{}\" should be \"{}\"", path.display(), CSV_EXTENSION);
        }

        let mut reader: Reader<File> = ReaderBuilder::new()
            .quoting(false)
            .from_path(path)
            .expect("verifications before should be enough");

        let data: MeetData = match reader.deserialize().next() {
            Some(Ok(data)) => data,
            Some(Err(e)) => bail!("error with file \"{}\" : {}", path.display(), e),
            None => bail!("error with file \"{}\" : no rows", path.display()),
        };

        Ok(data)
    }

    fn from_entries_csv(path: &PathBuf, meet_data: Option<&MeetData>) -> Result<Vec<Entry>> {
        if !path.exists() {
            bail!("path \"{}\" should exist", path.display());
        }

        if !path.is_file() {
            bail!("path \"{}\" should be a file", path.display());
        }

        if path.extension().unwrap() != CSV_EXTENSION {
            bail!("extension of \"{}\" should be \"{}\"", path.display(), CSV_EXTENSION);
        }

        let mut reader: Reader<File> = ReaderBuilder::new()
            .quoting(false)
            .from_path(path)
            .expect("verifications before should be enough");
        let mut entries: Vec<Entry> = Vec::with_capacity(50_000);
        for entry in reader.deserialize() {
            let entry: Entry = entry?;
            entries.push(entry);
        }

        if entries.is_empty() {
            bail!("file \"{}\" should contain at least one meet", path.display());
        }

        entries.shrink_to_fit();

        Ok(entries)
    }

    pub fn from_directory(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            bail!("path \"{}\" should exist", path.display());
        }

        if !path.is_dir() {
            bail!("path \"{}\" should be a directory", path.display());
        }

        let entries_filename: PathBuf = Path::new(ENTRIES_FILE_NAME).with_extension(CSV_EXTENSION);
        let data_filename: PathBuf = Path::new(MEET_FILE_NAME).with_extension(CSV_EXTENSION);

        let meets: Vec<Meet> = WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .map(|element| element.path().to_owned())
            .filter(|path| path.is_dir())
            .filter(|path| path.join(&entries_filename).exists())
            .filter(|path| path.join(&data_filename).exists())
            .sorted_by(Ord::cmp)
            .map(|path| {
                let entries_path: PathBuf = path.join(&entries_filename);
                let data_path: PathBuf = path.join(&data_filename);

                let data = match Self::from_data_csv(&data_path) {
                    Ok(data) => data,
                    Err(e) => {
                        warn!("file {} can't be used: {:?}", data_path.display(), e);
                        bail!(e);
                    }
                };

                let entries = match Self::from_entries_csv(&entries_path, Some(&data)) {
                    Ok(entries) => entries,
                    Err(e) => {
                        warn!("file {} can't be used: {:?}", entries_path.display(), e);
                        bail!(e);
                    }
                };

                Result::Ok(Meet::new(data, entries))
            })
        .filter_map(Result::ok)
            .collect();

        Ok(Self(meets))
    }

    pub async fn save(&self) -> Result<()> {
        let mut write_only_repository = Repository::write_only()?;
        write_only_repository.connect().await?;
        write_only_repository.refresh_migrations().await?;

        for meet in self.iter() {
            let meet_dto: MeetDto = meet.clone().into();
            write_only_repository.insert_meet(meet_dto).await?;
        }

        write_only_repository.disconnect().await?;

        Ok(())
    }
}

impl Deref for Database {
    type Target = Vec<Meet>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;

    use crate::types::{Country, Division, Equipment, Federation, Meet, Sex, Username, Weight, WeightClass};
    use crate::types::MeetData;
    use crate::types::Entry;


    use super::Database;

    const TEST_PATH: &str = "test_data/entries/meet_database";

    #[rstest]
    #[case("test1/entries.csv")]
    #[case("test2/entries.csv")]
    fn test_from_entries_csv_no_error(
        #[case] entries: &str
    ) {
        let test_file: PathBuf = Path::new(TEST_PATH).join(entries);

        let result: Result<Vec<Entry>> = Database::from_entries_csv(&test_file, None);

        assert!(result.is_ok(), "{}", result.unwrap_err());
    }

    #[rstest]
    #[case("unknownDirectory/entries.csv")]
    #[case("test1/")]
    #[case("invalid/entries.txt")]
    #[case("invalid/data.csv")]
    fn test_from_entries_csv_error(
        #[case] file: &str
    ) {
        let test_file: PathBuf = Path::new(TEST_PATH).join(file);

        let result: Result<Vec<Entry>> = Database::from_entries_csv(&test_file, None);

        assert!(result.is_err());
    }

    #[test]
    fn test_from_entries_1() {
        let test_file: PathBuf = Path::new(TEST_PATH).join("test1/entries.csv");
        let expected: Vec<Entry> = vec![
            Entry {
                name: Username::from_str("FirstName LastName").unwrap(),
                division: Division::Masters,
                equipment: Equipment::Raw,
                sex: Sex::M,
                bodyweight: Weight(104.),
                weight_class: Some(WeightClass::UnderOrEqual(Weight(105.))),
                squat1: Some(Weight(1.)),
                squat2: Some(Weight(2.)),
                squat3: Some(Weight(3.)),
                best3squat: Some(Weight(3.)),
                bench1: Some(Weight(4.)),
                bench2: Some(Weight(5.)),
                bench3: Some(Weight(6.)),
                best3bench: Some(Weight(6.)),
                deadlift1: Some(Weight(7.)),
                deadlift2: Some(Weight(8.)),
                deadlift3: Some(Weight(9.)),
                best3deadlift: Some(Weight(9.)),
                total: Some(Weight(18.)),
            },
            ];

        let result: Result<Vec<Entry>> = Database::from_entries_csv(&test_file, None);

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_from_entries_2() {
        let test_file: PathBuf = Path::new(TEST_PATH).join("test2/entries.csv");
        let expected: Vec<Entry> = vec![
            Entry {
                name: Username::from_str("Powerlifter 1").unwrap(),
                division: Division::Masters3,
                equipment: Equipment::Raw,
                sex: Sex::M,
                bodyweight: Weight(104.),
                weight_class: Some(WeightClass::UnderOrEqual(Weight(105.))),
                squat1: Some(Weight(1.)),
                squat2: Some(Weight(2.)),
                squat3: Some(Weight(3.)),
                best3squat: Some(Weight(3.)),
                bench1: Some(Weight(4.)),
                bench2: Some(Weight(5.)),
                bench3: Some(Weight(6.)),
                best3bench: Some(Weight(6.)),
                deadlift1: Some(Weight(7.)),
                deadlift2: Some(Weight(8.)),
                deadlift3: Some(Weight(9.)),
                best3deadlift: Some(Weight(9.)),
                total: Some(Weight(18.)),
            },
            Entry {
                name: Username::from_str("Powerlifter 2").unwrap(),
                division: Division::Juniors,
                equipment: Equipment::Raw,
                sex: Sex::F,
                bodyweight: Weight(80.1),
                weight_class: Some(WeightClass::UnderOrEqual(Weight(84.))),
                squat1: Some(Weight(10.)),
                squat2: Some(Weight(11.)),
                squat3: Some(Weight(12.)),
                best3squat: Some(Weight(12.)),
                bench1: Some(Weight(13.)),
                bench2: Some(Weight(14.)),
                bench3: Some(Weight(15.)),
                best3bench: Some(Weight(15.)),
                deadlift1: Some(Weight(16.)),
                deadlift2: Some(Weight(17.)),
                deadlift3: Some(Weight(18.)),
                best3deadlift: Some(Weight(18.)),
                total: Some(Weight(45.)),
            },
            ];

        let result: Result<Vec<Entry>> = Database::from_entries_csv(&test_file, None);

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case("test1/meet.csv")]
    #[case("test2/meet.csv")]
    fn test_from_meet_csv_no_error(#[case] path: &str) {
        let test_file: PathBuf = Path::new(TEST_PATH).join(path);

        let result: Result<MeetData> = Database::from_data_csv(&test_file);

        assert!(result.is_ok());
    }

    #[rstest]
    #[case("unknownDirectory/meet.csv")]
    #[case("test1")]
    #[case("test1/unknownFile.csv")]
    #[case("invalid/meet.txt")]
    #[case("invalid/meet.csv")]
    #[case("invalid/entries.txt")]
    #[case("invalid/data.csv")]
    fn test_from_data_csv_error(#[case] path: &str) {
        let test_file: PathBuf = Path::new(TEST_PATH).join(path);

        let result: Result<MeetData> = Database::from_data_csv(&test_file);

        assert!(result.is_err());
    }

    #[test]
    fn test_from_data_1() {
        let test_file: PathBuf = Path::new(TEST_PATH).join("test1/meet.csv");
        let expected: MeetData = MeetData {
            federation: Federation::FFForce,
            country: Country::FRANCE,
            state: "Ile de France".to_string(),
            town: "Paris".to_string(),
            name: "Meet Name".to_string(),
        };

        let result: Result<MeetData> = Database::from_data_csv(&test_file);

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_from_data_2() {
        let test_file: PathBuf = Path::new(TEST_PATH).join("test2/meet.csv");
        let expected: MeetData = MeetData {
            federation: Federation::IPF,
            country: Country::OTHER,
            state: String::new(),
            town: String::new(),
            name: "Other Meet".to_string(),
        };

        let result: Result<MeetData> = Database::from_data_csv(&test_file);

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case("unknownDirectory/")]
    #[case("invalid/meet.csv")]
    fn test_from_directory_error(#[case] path: &str) {
        let test_file: PathBuf = Path::new(TEST_PATH).join(path);

        let result: Result<Database> = Database::from_directory(&test_file);

        assert!(result.is_err());
    }

    #[rstest]
    #[case("invalid/meet1/")]
    #[case("invalid/meet2/")]
    #[case("invalid/meet3/")]
    #[case("invalid/meet4/")]
    fn test_from_directory_missing_file(#[case] path: &str) {
        let test_file: PathBuf = Path::new(TEST_PATH).join(path);
        let expected: Database = Database(Vec::new());

        let result: Result<Database> = Database::from_directory(&test_file);

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_from_directory_1() {
        let test_directory: PathBuf = Path::new(TEST_PATH).join("test1/");
        let expected: Database = Database(vec![
            Meet {
                data: MeetData {
                    federation: Federation::FFForce,
                    country: Country::FRANCE,
                    state: "Ile de France".to_string(),
                    town: "Paris".to_string(),
                    name: "Meet Name".to_string(),
                },
                entries: vec![
                    Entry {
                        name: Username::from_str("FirstName LastName").unwrap(),
                        division: Division::Masters,
                        equipment: Equipment::Raw,
                        sex: Sex::M,
                        bodyweight: Weight(104.),
                        weight_class: Some(WeightClass::UnderOrEqual(Weight(105.))),
                        squat1: Some(Weight(1.)),
                        squat2: Some(Weight(2.)),
                        squat3: Some(Weight(3.)),
                        best3squat: Some(Weight(3.)),
                        bench1: Some(Weight(4.)),
                        bench2: Some(Weight(5.)),
                        bench3: Some(Weight(6.)),
                        best3bench: Some(Weight(6.)),
                        deadlift1: Some(Weight(7.)),
                        deadlift2: Some(Weight(8.)),
                        deadlift3: Some(Weight(9.)),
                        best3deadlift: Some(Weight(9.)),
                        total: Some(Weight(18.)),
                    }
                ],
            }
        ]);

        let result: Result<Database> = Database::from_directory(&test_directory);

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_from_directory_2() {
        let test_directory: PathBuf = Path::new(TEST_PATH).join("test2/");
        let expected: Database = Database(vec![
            Meet {
                data: MeetData {
                    federation: Federation::IPF,
                    country: Country::OTHER,
                    state: String::new(),
                    town: String::new(),
                    name: "Other Meet".to_string(),
                },
                entries: vec![
                    Entry {
                        name: Username::from_str("Powerlifter 1").unwrap(),
                        division: Division::Masters3,
                        equipment: Equipment::Raw,
                        sex: Sex::M,
                        bodyweight: Weight(104.),
                        weight_class: Some(WeightClass::UnderOrEqual(Weight(105.))),
                        squat1: Some(Weight(1.)),
                        squat2: Some(Weight(2.)),
                        squat3: Some(Weight(3.)),
                        best3squat: Some(Weight(3.)),
                        bench1: Some(Weight(4.)),
                        bench2: Some(Weight(5.)),
                        bench3: Some(Weight(6.)),
                        best3bench: Some(Weight(6.)),
                        deadlift1: Some(Weight(7.)),
                        deadlift2: Some(Weight(8.)),
                        deadlift3: Some(Weight(9.)),
                        best3deadlift: Some(Weight(9.)),
                        total: Some(Weight(18.)),
                    },
                    Entry {
                        name: Username::from_str("Powerlifter 2").unwrap(),
                        division: Division::Juniors,
                        equipment: Equipment::Raw,
                        sex: Sex::F,
                        bodyweight: Weight(80.1),
                        weight_class: Some(WeightClass::UnderOrEqual(Weight(84.))),
                        squat1: Some(Weight(10.)),
                        squat2: Some(Weight(11.)),
                        squat3: Some(Weight(12.)),
                        best3squat: Some(Weight(12.)),
                        bench1: Some(Weight(13.)),
                        bench2: Some(Weight(14.)),
                        bench3: Some(Weight(15.)),
                        best3bench: Some(Weight(15.)),
                        deadlift1: Some(Weight(16.)),
                        deadlift2: Some(Weight(17.)),
                        deadlift3: Some(Weight(18.)),
                        best3deadlift: Some(Weight(18.)),
                        total: Some(Weight(45.)),
                    }
                ],
            }
        ]);

        let result: Result<Database> = Database::from_directory(&test_directory);

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());

    }

    #[test]
    fn test_from_directory_3() {
        let test_file: PathBuf = Path::new(TEST_PATH).join("test3/");
        let expected: Database = Database(vec![
            Meet {
                data: MeetData {
                    federation: Federation::FFForce,
                    country: Country::FRANCE,
                    state: "Ile de France".to_string(),
                    town: "Paris".to_string(),
                    name: "Meet Name".to_string(),
                },
                entries: vec![
                    Entry {
                        name: Username::from_str("FirstName LastName").unwrap(),
                        division: Division::Masters,
                        equipment: Equipment::Raw,
                        sex: Sex::M,
                        bodyweight: Weight(104.),
                        weight_class: Some(WeightClass::UnderOrEqual(Weight(105.))),
                        squat1: Some(Weight(1.)),
                        squat2: Some(Weight(2.)),
                        squat3: Some(Weight(3.)),
                        best3squat: Some(Weight(3.)),
                        bench1: Some(Weight(4.)),
                        bench2: Some(Weight(5.)),
                        bench3: Some(Weight(6.)),
                        best3bench: Some(Weight(6.)),
                        deadlift1: Some(Weight(7.)),
                        deadlift2: Some(Weight(8.)),
                        deadlift3: Some(Weight(9.)),
                        best3deadlift: Some(Weight(9.)),
                        total: Some(Weight(18.)),
                    }
                ],
            },
            Meet {
                data: MeetData {
                    federation: Federation::IPF,
                    country: Country::OTHER,
                    state: String::new(),
                    town: String::new(),
                    name: "Other Meet".to_string(),
                },
                entries: vec![
                    Entry {
                        name: Username::from_str("Powerlifter 1").unwrap(),
                        division: Division::Masters3,
                        equipment: Equipment::Raw,
                        sex: Sex::M,
                        bodyweight: Weight(104.),
                        weight_class: Some(WeightClass::UnderOrEqual(Weight(105.))),
                        squat1: Some(Weight(1.)),
                        squat2: Some(Weight(2.)),
                        squat3: Some(Weight(3.)),
                        best3squat: Some(Weight(3.)),
                        bench1: Some(Weight(4.)),
                        bench2: Some(Weight(5.)),
                        bench3: Some(Weight(6.)),
                        best3bench: Some(Weight(6.)),
                        deadlift1: Some(Weight(7.)),
                        deadlift2: Some(Weight(8.)),
                        deadlift3: Some(Weight(9.)),
                        best3deadlift: Some(Weight(9.)),
                        total: Some(Weight(18.)),
                    },
                    Entry {
                        name: Username::from_str("Powerlifter 2").unwrap(),
                        division: Division::Juniors,
                        equipment: Equipment::Raw,
                        sex: Sex::F,
                        bodyweight: Weight(80.1),
                        weight_class: Some(WeightClass::UnderOrEqual(Weight(84.))),
                        squat1: Some(Weight(10.)),
                        squat2: Some(Weight(11.)),
                        squat3: Some(Weight(12.)),
                        best3squat: Some(Weight(12.)),
                        bench1: Some(Weight(13.)),
                        bench2: Some(Weight(14.)),
                        bench3: Some(Weight(15.)),
                        best3bench: Some(Weight(15.)),
                        deadlift1: Some(Weight(16.)),
                        deadlift2: Some(Weight(17.)),
                        deadlift3: Some(Weight(18.)),
                        best3deadlift: Some(Weight(18.)),
                        total: Some(Weight(45.)),
                    }
                ],
            }
        ]);

        let result: Result<Database> = Database::from_directory(&test_file);

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }
}
