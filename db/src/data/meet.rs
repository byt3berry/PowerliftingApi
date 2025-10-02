use itertools::Itertools;
use types::{MatchesQuery, Query};

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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use pretty_assertions::assert_eq;
    use types::{Division, Equipment, Federation, Query, Sex, Username, Weight, WeightClass};

    use crate::data::meet_data::MeetData;
    use crate::data::meet_entry::MeetEntry;

    use super::Meet;

    #[test]
    fn test_filter_matching_federation_not_matching() {
        let meet: Meet = Meet {
            data: MeetData {
                federation: Federation::IPF,
                country: types::Country::FRANCE,
                state: String::new(),
                town: String::new(),
                name: String::new(),
            },
            entries: vec![
                MeetEntry {
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
                    federation: Federation::FFForce,
                }
            ],
        };
        let query: Query = Query {
            federation_choice: Federation::FFForce,
            equipment_choice: Equipment::Any,
            sex_choice: Sex::Any,
            division_choice: Division::Any,
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
                federation: Federation::FFForce,
                country: types::Country::FRANCE,
                state: String::new(),
                town: String::new(),
                name: String::new(),
            },
            entries: vec![
                MeetEntry {
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
                    federation: Federation::FFForce,
                }
            ],
        };
        let query: Query = Query {
            federation_choice: Federation::FFForce,
            equipment_choice: Equipment::Wraps,
            sex_choice: Sex::Any,
            division_choice: Division::Any,
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
                federation: Federation::FFForce,
                country: types::Country::FRANCE,
                state: String::new(),
                town: String::new(),
                name: String::new(),
            },
            entries: vec![
                MeetEntry {
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
                    federation: Federation::FFForce,
                }
            ],
        };
        let query: Query = Query {
            federation_choice: Federation::FFForce,
            equipment_choice: Equipment::Any,
            sex_choice: Sex::Any,
            division_choice: Division::Juniors,
            powerlifters: String::new(),
        };
        let expected: Vec<&MeetEntry> = Vec::new();

        let result: Vec<&MeetEntry> = meet.filter_matching(&query);

        assert_eq!(expected, result);
    }
}
