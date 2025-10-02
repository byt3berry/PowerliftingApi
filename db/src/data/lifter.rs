use types::{Query, Username};

use crate::data::meet_entry::MeetEntry;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lifter {
    pub name: Username,
    pub meets: Vec<MeetEntry>,
}

impl Lifter {
    #[must_use]
    pub const fn new(name: Username, meets: Vec<MeetEntry>) -> Self {
        Self { name, meets, }
    }

    #[must_use]
    pub fn best_total(&self, query: &Query) -> Option<&MeetEntry> {
        self
            .meets
            .iter()
            .filter(|meet| query.sex_choice == meet.sex)
            .filter(|meet| query.division_choice == meet.division)
            .filter(|meet| query.equipment_choice == meet.equipment)
            .max_by_key(|meet| meet.total)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use types::Federation;
    use std::str::FromStr;
    use types::{Division, Equipment, Sex, Username, Weight, WeightClass};
                        
    use super::Lifter;
    use super::MeetEntry;

    #[test]
    fn test_from_meet_data_1() -> Result<()> {
        let username: Username = Username::from_str("Powerlifter").unwrap();
        let data: Vec<MeetEntry> = vec![
            MeetEntry {
                name: Username::from_str("Powerlifter").unwrap(),
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
                total: Weight(18.),
                federation: Federation::FFForce,
            }
        ];
        let expected: Lifter = Lifter {
            name: Username::from_str("Powerlifter").unwrap(),
            meets: vec![
                MeetEntry {
                    name: Username::from_str("Powerlifter").unwrap(),
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
                    total: Weight(18.),
                    federation: Federation::FFForce,
                }
            ],
        };

        let lifter: Lifter = Lifter::new(username, data);

        assert_eq!(expected, lifter);
        Ok(())
    }

    #[test]
    fn test_from_meet_data_2() -> Result<()> {
        let username: Username = Username::from_str("Powerlifter").unwrap();
        let data: Vec<MeetEntry> = vec![
            MeetEntry {
                name: Username::from_str("Powerlifter").unwrap(),
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
                total: Weight(18.),
                federation: Federation::FFForce,
            },
            MeetEntry {
                name: Username::from_str("Powerlifter").unwrap(),
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
                total: Weight(21.),
                federation: Federation::FFForce,
            }
        ];
        let expected: Lifter = Lifter {
            name: Username::from_str("Powerlifter").unwrap(),
            meets: vec![
                MeetEntry {
                    name: Username::from_str("Powerlifter").unwrap(),
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
                    total: Weight(18.),
                    federation: Federation::FFForce,
                },
                MeetEntry {
                    name: Username::from_str("Powerlifter").unwrap(),
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
                    total: Weight(21.),
                    federation: Federation::FFForce,
                }
            ],
        };

        let lifter: Lifter = Lifter::new(username, data);

        assert_eq!(expected, lifter);
        Ok(())
    }
}
