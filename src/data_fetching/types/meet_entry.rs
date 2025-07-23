use serde::Deserialize;

use crate::data_fetching::types::division::Division;
use crate::data_fetching::types::dots::Dots;
use crate::data_fetching::types::equipment::Equipment;
use crate::data_fetching::types::sex::Sex;
use crate::data_fetching::types::weight::Weight;
use crate::data_fetching::types::weight_class::WeightClass;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct MeetEntry {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,

    #[serde(rename(deserialize = "Division"))]
    pub division: Division,

    #[serde(rename(deserialize = "Equipment"))]
    pub equipment: Equipment,

    #[serde(rename(deserialize = "Age"))]
    pub age: Option<u8>,

    #[serde(rename(deserialize = "Sex"))]
    pub sex: Sex,

    #[serde(rename(deserialize = "BodyweightKg"))]
    pub bodyweight: Weight,

    #[serde(rename(deserialize = "WeightClassKg"))]
    pub weight_class: Option<WeightClass>,

    #[serde(rename(deserialize = "Squat1Kg"))]
    pub squat1: Option<Weight>,

    #[serde(rename(deserialize = "Squat2Kg"))]
    pub squat2: Option<Weight>,

    #[serde(rename(deserialize = "Squat3Kg"))]
    pub squat3: Option<Weight>,

    #[serde(rename(deserialize = "Best3SquatKg"))]
    pub best3squat: Option<Weight>,

    #[serde(rename(deserialize = "Bench1Kg"))]
    pub bench1: Option<Weight>,

    #[serde(rename(deserialize = "Bench2Kg"))]
    pub bench2: Option<Weight>,

    #[serde(rename(deserialize = "Bench3Kg"))]
    pub bench3: Option<Weight>,

    #[serde(rename(deserialize = "Best3BenchKg"))]
    pub best3bench: Option<Weight>,

    #[serde(rename(deserialize = "Deadlift1Kg"))]
    pub deadlift1: Option<Weight>,

    #[serde(rename(deserialize = "Deadlift2Kg"))]
    pub deadlift2: Option<Weight>,

    #[serde(rename(deserialize = "Deadlift3Kg"))]
    pub deadlift3: Option<Weight>,

    #[serde(rename(deserialize = "Best3DeadliftKg"))]
    pub best3deadlift: Option<Weight>,

    #[serde(rename(deserialize = "TotalKg"))]
    pub total: Weight,
}

impl MeetEntry {
    pub fn from_string(data: &str) -> Vec<Self> {
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

    pub fn dots(&self) ->Dots {
        Dots::new(self.sex, self.bodyweight, self.total)
    }
}
