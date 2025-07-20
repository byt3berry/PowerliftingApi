use serde::Deserialize;

use crate::data_fetching::types::division::Division;
use crate::data_fetching::types::equipment::Equipment;
use crate::data_fetching::types::sex::Sex;
use crate::data_fetching::types::weight::Weight;
use crate::data_fetching::types::weight_class::WeightClass;

#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct MeetEntry {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,

    #[serde(rename(deserialize = "Division"))]
    pub division: Division,

    #[serde(rename(deserialize = "Equipment"))]
    pub equipment: Equipment,

    #[serde(rename(deserialize = "Age"))]
    pub age: u8,

    #[serde(rename(deserialize = "Sex"))]
    pub sex: Sex,

    #[serde(rename(deserialize = "BodyweightKg"))]
    pub bodyweight: Weight,

    #[serde(rename(deserialize = "WeightClassKg"))]
    pub weight_class: Option<WeightClass>,

    #[serde(rename(deserialize = "Squat1Kg"))]
    pub squat1: Weight,

    #[serde(rename(deserialize = "Squat2Kg"))]
    pub squat2: Weight,

    #[serde(rename(deserialize = "Squat3Kg"))]
    pub squat3: Weight,

    #[serde(rename(deserialize = "Best3SquatKg"))]
    pub best3squat: Weight,

    #[serde(rename(deserialize = "Bench1Kg"))]
    pub bench1: Weight,

    #[serde(rename(deserialize = "Bench2Kg"))]
    pub bench2: Weight,

    #[serde(rename(deserialize = "Bench3Kg"))]
    pub bench3: Weight,

    #[serde(rename(deserialize = "Best3BenchKg"))]
    pub best3bench: Weight,

    #[serde(rename(deserialize = "Deadlift1Kg"))]
    pub deadlift1: Weight,

    #[serde(rename(deserialize = "Deadlift2Kg"))]
    pub deadlift2: Weight,

    #[serde(rename(deserialize = "Deadlift3Kg"))]
    pub deadlift3: Weight,

    #[serde(rename(deserialize = "Best3DeadliftKg"))]
    pub best3deadlift: Weight,

    #[serde(rename(deserialize = "TotalKg"))]
    pub total: Weight
}

impl MeetEntry {
    pub fn from_string(data: &String) -> Vec<Self> {
        data
            .lines()
            .map(|name| {
                Self::default().with_name(name.to_string())
            })
            .collect()
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}
