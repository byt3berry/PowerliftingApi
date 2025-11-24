use types::{EntryDto, MatchesQuery, MeetDto, Query};

use crate::data::meet_data::MeetData;
use crate::data::meet_entry::MeetEntry;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Meet {
    pub data: MeetData,
    pub entries: Vec<MeetEntry>,
}

impl Meet {
    #[must_use]
    pub const fn new(data: MeetData, entries: Vec<MeetEntry>) -> Self {
        Self { data, entries }
    }

    pub fn filter_matching<'a>(&'a self, query: &Query) -> Vec<&'a MeetEntry> {
        if !self.data.matches_query(query) {
            return Vec::new();
        }

        self.entries
            .iter()
            .filter(|entry| entry.matches_query(query))
            .collect()
    }
}

impl From<Meet> for MeetDto {
    fn from(value: Meet) -> Self {
        Self {
            data: value.data.into(),
            entries: value.entries.into_iter().map(EntryDto::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use pretty_assertions::assert_eq;
    use types::{DivisionDto, EquipmentDto, FederationDto, Query, SexDto, UsernameDto, WeightDto, WeightClassDto};

    use crate::data::meet_data::MeetData;
    use crate::data::meet_entry::MeetEntry;

    use super::Meet;

    #[test]
    fn test_filter_matching_federation_not_matching() {
        let meet: Meet = Meet {
            data: MeetData {
                federation: FederationDto::IPF,
                country: types::CountryDto::FRANCE,
                state: String::new(),
                town: String::new(),
                name: String::new(),
            },
            entries: vec![
                MeetEntry {
                    name: UsernameDto::from_str("FirstName LastName").unwrap(),
                    division: DivisionDto::Masters,
                    equipment: EquipmentDto::Raw,
                    sex: SexDto::M,
                    bodyweight: WeightDto(104.),
                    weight_class: WeightClassDto::UnderOrEqual(WeightDto(105.)).into(),
                    squat1: WeightDto(1.).into(),
                    squat2: WeightDto(2.).into(),
                    squat3: WeightDto(3.).into(),
                    best3squat: WeightDto(3.).into(),
                    bench1: WeightDto(4.).into(),
                    bench2: WeightDto(5.).into(),
                    bench3: WeightDto(6.).into(),
                    best3bench: WeightDto(6.).into(),
                    deadlift1: WeightDto(7.).into(),
                    deadlift2: WeightDto(8.).into(),
                    deadlift3: WeightDto(9.).into(),
                    best3deadlift: WeightDto(9.).into(),
                    total: WeightDto(18.),
                    federation: FederationDto::FFForce,
                }
            ],
        };
        let query: Query = Query {
            federation_choice: FederationDto::FFForce,
            equipment_choice: EquipmentDto::Any,
            sex_choice: SexDto::Any,
            division_choice: DivisionDto::Any,
            powerlifters: String::new(),
        };
        let expected: Vec<&MeetEntry> = Vec::new();

        let result: Vec<&MeetEntry> = meet.filter_matching(&query);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_filter_matching_equipment_not_matching() {
        let meet: Meet = Meet {
            data: MeetData {
                federation: FederationDto::FFForce,
                country: types::CountryDto::FRANCE,
                state: String::new(),
                town: String::new(),
                name: String::new(),
            },
            entries: vec![
                MeetEntry {
                    name: UsernameDto::from_str("FirstName LastName").unwrap(),
                    division: DivisionDto::Masters,
                    equipment: EquipmentDto::Raw,
                    sex: SexDto::M,
                    bodyweight: WeightDto(104.),
                    weight_class: WeightClassDto::UnderOrEqual(WeightDto(105.)).into(),
                    squat1: WeightDto(1.).into(),
                    squat2: WeightDto(2.).into(),
                    squat3: WeightDto(3.).into(),
                    best3squat: WeightDto(3.).into(),
                    bench1: WeightDto(4.).into(),
                    bench2: WeightDto(5.).into(),
                    bench3: WeightDto(6.).into(),
                    best3bench: WeightDto(6.).into(),
                    deadlift1: WeightDto(7.).into(),
                    deadlift2: WeightDto(8.).into(),
                    deadlift3: WeightDto(9.).into(),
                    best3deadlift: WeightDto(9.).into(),
                    total: WeightDto(18.),
                    federation: FederationDto::FFForce,
                }
            ],
        };
        let query: Query = Query {
            federation_choice: FederationDto::FFForce,
            equipment_choice: EquipmentDto::Wraps,
            sex_choice: SexDto::Any,
            division_choice: DivisionDto::Any,
            powerlifters: String::new(),
        };
        let expected: Vec<&MeetEntry> = Vec::new();

        let result: Vec<&MeetEntry> = meet.filter_matching(&query);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_filter_matching_division_not_matching() {
        let meet: Meet = Meet {
            data: MeetData {
                federation: FederationDto::FFForce,
                country: types::CountryDto::FRANCE,
                state: String::new(),
                town: String::new(),
                name: String::new(),
            },
            entries: vec![
                MeetEntry {
                    name: UsernameDto::from_str("FirstName LastName").unwrap(),
                    division: DivisionDto::Masters,
                    equipment: EquipmentDto::Raw,
                    sex: SexDto::M,
                    bodyweight: WeightDto(104.),
                    weight_class: WeightClassDto::UnderOrEqual(WeightDto(105.)).into(),
                    squat1: WeightDto(1.).into(),
                    squat2: WeightDto(2.).into(),
                    squat3: WeightDto(3.).into(),
                    best3squat: WeightDto(3.).into(),
                    bench1: WeightDto(4.).into(),
                    bench2: WeightDto(5.).into(),
                    bench3: WeightDto(6.).into(),
                    best3bench: WeightDto(6.).into(),
                    deadlift1: WeightDto(7.).into(),
                    deadlift2: WeightDto(8.).into(),
                    deadlift3: WeightDto(9.).into(),
                    best3deadlift: WeightDto(9.).into(),
                    total: WeightDto(18.),
                    federation: FederationDto::FFForce,
                }
            ],
        };
        let query: Query = Query {
            federation_choice: FederationDto::FFForce,
            equipment_choice: EquipmentDto::Any,
            sex_choice: SexDto::Any,
            division_choice: DivisionDto::Juniors,
            powerlifters: String::new(),
        };
        let expected: Vec<&MeetEntry> = Vec::new();

        let result: Vec<&MeetEntry> = meet.filter_matching(&query);

        assert_eq!(expected, result);
    }
}
