use itertools::Itertools;
use std::iter::Peekable;
use types::Sex;
use types::Username;

use crate::data::meet_entry::MeetEntry;
use crate::data::query::Query;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lifter {
    pub name: Username,
    pub sex: Sex,
    pub meets: Vec<MeetEntry>,
}

impl Lifter {
    pub fn best_total(&self, form: &Query) -> Option<&MeetEntry> {
        self
            .meets
            .iter()
            .filter(|meet| form.sex_choice == meet.sex)
            .filter(|meet| form.division_choice == meet.division)
            .filter(|meet| form.equipment_choice == meet.equipment)
            .max_by_key(|meet| meet.total)
    }
}

impl<'a, I> From<I> for Lifter where I: Iterator<Item = &'a MeetEntry> {
    fn from(data: I) -> Self {
        let mut data: Peekable<I> = data.peekable();

        let best_meet: &MeetEntry = data
            .peek()
            .expect("each lifter should have at least one meet entry");

        let meets: Vec<MeetEntry> = data
            .map(|meet| meet.to_owned())
            .sorted_by_key(|meet| meet.total)
            .collect();

        Self {
            name: best_meet.name.clone(),
            sex: best_meet.sex,
            meets,
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;
    use types::{Division, Equipment, Sex, Username, Weight, WeightClass};
                        
    use super::Lifter;
    use super::MeetEntry;

    #[test]
    #[should_panic]
    fn test_from_meet_data_error() {
        let data: Vec<MeetEntry> = vec![];

        let _ = Lifter::from(data.iter());
    }

    #[test]
    fn test_from_meet_data_1() -> Result<()> {
        let data: Vec<MeetEntry> = vec![
            MeetEntry {
                name: Username::from_str("Powerlifter")?,
                division: Division::Juniors,
                equipment: Equipment::Raw,
                sex: Sex::M,
                bodyweight: Weight(81.),
                weight_class: WeightClass::UnderOrEqual(Weight(83.)).into(),
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
                total: Weight(18.)
            },
            ];
        let expected: Lifter = Lifter {
            name: Username::from_str("Powerlifter")?,
            sex: Sex::M,
            meets: vec![
                MeetEntry {
                    name: Username::from_str("Powerlifter")?,
                    division: Division::Juniors,
                    equipment: Equipment::Raw,
                    sex: Sex::M,
                    bodyweight: Weight(81.),
                    weight_class: WeightClass::UnderOrEqual(Weight(83.)).into(),
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
                    total: Weight(18.)
                },
                ],
        };

        let lifter: Lifter = Lifter::from(data.iter());

        assert_eq!(lifter, expected);
        Ok(())
    }

    #[test]
    fn test_from_meet_data_2() -> Result<()> {
        let data: Vec<MeetEntry> = vec![
            MeetEntry {
                name: Username::from_str("Powerlifter")?,
                division: Division::Juniors,
                equipment: Equipment::Raw,
                sex: Sex::M,
                bodyweight: Weight(81.),
                weight_class: WeightClass::UnderOrEqual(Weight(83.)).into(),
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
                total: Weight(18.)
            },
            MeetEntry {
                name: Username::from_str("Powerlifter")?,
                division: Division::Juniors,
                equipment: Equipment::Raw,
                sex: Sex::M,
                bodyweight: Weight(82.),
                weight_class: WeightClass::UnderOrEqual(Weight(83.)).into(),
                squat1: Some(Weight(2.)),
                squat2: Some(Weight(3.)),
                squat3: Some(Weight(4.)),
                best3squat: Some(Weight(4.)),
                bench1: Some(Weight(5.)),
                bench2: Some(Weight(6.)),
                bench3: Some(Weight(7.)),
                best3bench: Some(Weight(7.)),
                deadlift1: Some(Weight(8.)),
                deadlift2: Some(Weight(9.)),
                deadlift3: Some(Weight(10.)),
                best3deadlift: Some(Weight(10.)),
                total: Weight(21.)
            },
            ];
        let expected: Lifter = Lifter {
            name: Username::from_str("Powerlifter")?,
            sex: Sex::M,
            meets: vec![
                MeetEntry {
                    name: Username::from_str("Powerlifter")?,
                    division: Division::Juniors,
                    equipment: Equipment::Raw,
                    sex: Sex::M,
                    bodyweight: Weight(81.),
                    weight_class: WeightClass::UnderOrEqual(Weight(83.)).into(),
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
                    total: Weight(18.)
                },
                MeetEntry {
                    name: Username::from_str("Powerlifter")?,
                    division: Division::Juniors,
                    equipment: Equipment::Raw,
                    sex: Sex::M,
                    bodyweight: Weight(82.),
                    weight_class: WeightClass::UnderOrEqual(Weight(83.)).into(),
                    squat1: Some(Weight(2.)),
                    squat2: Some(Weight(3.)),
                    squat3: Some(Weight(4.)),
                    best3squat: Some(Weight(4.)),
                    bench1: Some(Weight(5.)),
                    bench2: Some(Weight(6.)),
                    bench3: Some(Weight(7.)),
                    best3bench: Some(Weight(7.)),
                    deadlift1: Some(Weight(8.)),
                    deadlift2: Some(Weight(9.)),
                    deadlift3: Some(Weight(10.)),
                    best3deadlift: Some(Weight(10.)),
                    total: Weight(21.)
                },
                ],
        };

        let lifter: Lifter = Lifter::from(data.iter());

        assert_eq!(lifter, expected);
        Ok(())
    }
}
