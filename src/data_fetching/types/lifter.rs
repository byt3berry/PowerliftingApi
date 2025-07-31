use crate::data_fetching::types::division::Division;
use crate::data_fetching::types::equipment::Equipment;
use crate::data_fetching::types::meet_entry::MeetEntry;
use crate::data_fetching::types::sex::Sex;
use crate::data_fetching::types::username::Username;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lifter {
    pub name: Username,
    pub equipment: Equipment,
    pub sex: Sex,
    pub division: Division,
    pub best_meet: MeetEntry,
}

impl Lifter {
    pub fn from_meet_data<'a, I>(data: I) -> Self
        where I: Iterator<Item = &'a MeetEntry>
    {
        let best_meet: MeetEntry = data
            .max_by_key(|entry| entry.total)
            .expect("each lifter should have at least one meet entry")
            .clone();

        Self {
            name: best_meet.name.clone(),
            equipment: best_meet.equipment,
            sex: best_meet.sex,
            division: best_meet.division,
            best_meet,
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    use crate::data_fetching::types::division::Division;
    use crate::data_fetching::types::equipment::Equipment;
    use crate::data_fetching::types::meet_entry::MeetEntry;
    use crate::data_fetching::types::sex::Sex;
    use crate::data_fetching::types::username::Username;
    use crate::data_fetching::types::weight::Weight;
    use crate::data_fetching::types::weight_class::WeightClass;

    use super::Lifter;

    #[test]
    #[should_panic]
    fn test_from_meet_data_error() {
        let data: Vec<MeetEntry> = vec![];

        Lifter::from_meet_data(data.iter());
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
            equipment: Equipment::Raw,
            sex: Sex::M,
            division: Division::Juniors,
            best_meet: MeetEntry {
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
        };

        let lifter: Lifter = Lifter::from_meet_data(data.iter());

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
            equipment: Equipment::Raw,
            sex: Sex::M,
            division: Division::Juniors,
            best_meet: MeetEntry {
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
        };

        let lifter: Lifter = Lifter::from_meet_data(data.iter());

        assert_eq!(lifter, expected);
        Ok(())
    }
}
