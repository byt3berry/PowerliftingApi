use types::{Query, UsernameDto};

use crate::data::meet_entry::MeetEntry;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lifter {
    pub name: UsernameDto,
    pub meets: Vec<MeetEntry>,
}

impl Lifter {
    #[must_use]
    pub const fn new(name: UsernameDto, meets: Vec<MeetEntry>) -> Self {
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
    use types::FederationDto;
    use std::str::FromStr;
    use types::{DivisionDto, EquipmentDto, SexDto, UsernameDto, WeightDto, WeightClassDto};
                        
    use super::Lifter;
    use super::MeetEntry;

    #[test]
    fn test_from_meet_data_1() -> Result<()> {
        let username: UsernameDto = UsernameDto::from_str("Powerlifter").unwrap();
        let data: Vec<MeetEntry> = vec![
            MeetEntry {
                name: UsernameDto::from_str("Powerlifter").unwrap(),
                division: DivisionDto::Juniors,
                equipment: EquipmentDto::Raw,
                sex: SexDto::M,
                bodyweight: WeightDto(81.),
                weight_class: WeightClassDto::UnderOrEqual(WeightDto(83.)).into(),
                squat1: Some(WeightDto(1.)),
                squat2: Some(WeightDto(2.)),
                squat3: Some(WeightDto(3.)),
                best3squat: Some(WeightDto(3.)),
                bench1: Some(WeightDto(4.)),
                bench2: Some(WeightDto(5.)),
                bench3: Some(WeightDto(6.)),
                best3bench: Some(WeightDto(6.)),
                deadlift1: Some(WeightDto(7.)),
                deadlift2: Some(WeightDto(8.)),
                deadlift3: Some(WeightDto(9.)),
                best3deadlift: Some(WeightDto(9.)),
                total: WeightDto(18.),
                federation: FederationDto::FFForce,
            }
        ];
        let expected: Lifter = Lifter {
            name: UsernameDto::from_str("Powerlifter").unwrap(),
            meets: vec![
                MeetEntry {
                    name: UsernameDto::from_str("Powerlifter").unwrap(),
                    division: DivisionDto::Juniors,
                    equipment: EquipmentDto::Raw,
                    sex: SexDto::M,
                    bodyweight: WeightDto(81.),
                    weight_class: WeightClassDto::UnderOrEqual(WeightDto(83.)).into(),
                    squat1: Some(WeightDto(1.)),
                    squat2: Some(WeightDto(2.)),
                    squat3: Some(WeightDto(3.)),
                    best3squat: Some(WeightDto(3.)),
                    bench1: Some(WeightDto(4.)),
                    bench2: Some(WeightDto(5.)),
                    bench3: Some(WeightDto(6.)),
                    best3bench: Some(WeightDto(6.)),
                    deadlift1: Some(WeightDto(7.)),
                    deadlift2: Some(WeightDto(8.)),
                    deadlift3: Some(WeightDto(9.)),
                    best3deadlift: Some(WeightDto(9.)),
                    total: WeightDto(18.),
                    federation: FederationDto::FFForce,
                }
            ],
        };

        let lifter: Lifter = Lifter::new(username, data);

        assert_eq!(expected, lifter);
        Ok(())
    }

    #[test]
    fn test_from_meet_data_2() -> Result<()> {
        let username: UsernameDto = UsernameDto::from_str("Powerlifter").unwrap();
        let data: Vec<MeetEntry> = vec![
            MeetEntry {
                name: UsernameDto::from_str("Powerlifter").unwrap(),
                division: DivisionDto::Juniors,
                equipment: EquipmentDto::Raw,
                sex: SexDto::M,
                bodyweight: WeightDto(81.),
                weight_class: WeightClassDto::UnderOrEqual(WeightDto(83.)).into(),
                squat1: Some(WeightDto(1.)),
                squat2: Some(WeightDto(2.)),
                squat3: Some(WeightDto(3.)),
                best3squat: Some(WeightDto(3.)),
                bench1: Some(WeightDto(4.)),
                bench2: Some(WeightDto(5.)),
                bench3: Some(WeightDto(6.)),
                best3bench: Some(WeightDto(6.)),
                deadlift1: Some(WeightDto(7.)),
                deadlift2: Some(WeightDto(8.)),
                deadlift3: Some(WeightDto(9.)),
                best3deadlift: Some(WeightDto(9.)),
                total: WeightDto(18.),
                federation: FederationDto::FFForce,
            },
            MeetEntry {
                name: UsernameDto::from_str("Powerlifter").unwrap(),
                division: DivisionDto::Juniors,
                equipment: EquipmentDto::Raw,
                sex: SexDto::M,
                bodyweight: WeightDto(82.),
                weight_class: WeightClassDto::UnderOrEqual(WeightDto(83.)).into(),
                squat1: Some(WeightDto(2.)),
                squat2: Some(WeightDto(3.)),
                squat3: Some(WeightDto(4.)),
                best3squat: Some(WeightDto(4.)),
                bench1: Some(WeightDto(5.)),
                bench2: Some(WeightDto(6.)),
                bench3: Some(WeightDto(7.)),
                best3bench: Some(WeightDto(7.)),
                deadlift1: Some(WeightDto(8.)),
                deadlift2: Some(WeightDto(9.)),
                deadlift3: Some(WeightDto(10.)),
                best3deadlift: Some(WeightDto(10.)),
                total: WeightDto(21.),
                federation: FederationDto::FFForce,
            }
        ];
        let expected: Lifter = Lifter {
            name: UsernameDto::from_str("Powerlifter").unwrap(),
            meets: vec![
                MeetEntry {
                    name: UsernameDto::from_str("Powerlifter").unwrap(),
                    division: DivisionDto::Juniors,
                    equipment: EquipmentDto::Raw,
                    sex: SexDto::M,
                    bodyweight: WeightDto(81.),
                    weight_class: WeightClassDto::UnderOrEqual(WeightDto(83.)).into(),
                    squat1: Some(WeightDto(1.)),
                    squat2: Some(WeightDto(2.)),
                    squat3: Some(WeightDto(3.)),
                    best3squat: Some(WeightDto(3.)),
                    bench1: Some(WeightDto(4.)),
                    bench2: Some(WeightDto(5.)),
                    bench3: Some(WeightDto(6.)),
                    best3bench: Some(WeightDto(6.)),
                    deadlift1: Some(WeightDto(7.)),
                    deadlift2: Some(WeightDto(8.)),
                    deadlift3: Some(WeightDto(9.)),
                    best3deadlift: Some(WeightDto(9.)),
                    total: WeightDto(18.),
                    federation: FederationDto::FFForce,
                },
                MeetEntry {
                    name: UsernameDto::from_str("Powerlifter").unwrap(),
                    division: DivisionDto::Juniors,
                    equipment: EquipmentDto::Raw,
                    sex: SexDto::M,
                    bodyweight: WeightDto(82.),
                    weight_class: WeightClassDto::UnderOrEqual(WeightDto(83.)).into(),
                    squat1: Some(WeightDto(2.)),
                    squat2: Some(WeightDto(3.)),
                    squat3: Some(WeightDto(4.)),
                    best3squat: Some(WeightDto(4.)),
                    bench1: Some(WeightDto(5.)),
                    bench2: Some(WeightDto(6.)),
                    bench3: Some(WeightDto(7.)),
                    best3bench: Some(WeightDto(7.)),
                    deadlift1: Some(WeightDto(8.)),
                    deadlift2: Some(WeightDto(9.)),
                    deadlift3: Some(WeightDto(10.)),
                    best3deadlift: Some(WeightDto(10.)),
                    total: WeightDto(21.),
                    federation: FederationDto::FFForce,
                }
            ],
        };

        let lifter: Lifter = Lifter::new(username, data);

        assert_eq!(expected, lifter);
        Ok(())
    }
}
