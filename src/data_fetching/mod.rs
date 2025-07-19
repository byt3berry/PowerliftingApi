use types::division::Division;
use types::equipment::Equipment;
use types::place::Place;
use types::sex::Sex;
use types::weight_class::WeightClass;
use types::weight::Weight;

mod types;

#[derive(Debug, Default)]
pub struct LifterEntry {
    pub place: Place,
    pub name: String,
    pub division: Option<Division>,
    pub equipment: Equipment,
    pub age: u8,
    pub sex: Sex,
    pub bodyweightkg: Weight,
    pub weightclasskg: WeightClass,
    pub squat1kg: Weight,
    pub squat2kg: Weight,
    pub squat3kg: Weight,
    pub best3squatkg: Weight,
    pub bench1kg: Weight,
    pub bench2kg: Weight,
    pub bench3kg: Weight,
    pub best3benchkg: Weight,
    pub deadlift1kg: Weight,
    pub deadlift2kg: Weight,
    pub deadlift3kg: Weight,
    pub best3deadliftkg: Weight,
    pub totalkg: Weight,
}

pub const POWERLIFTER_TABLE_HEADERS: [&str; 12] = [
    "Rank", 
    "Lifter", 
    "Federation", 
    "Sex", 
    "Age", 
    "Equipment", 
    "Class", 
    "Weight", 
    "Squat", 
    "Bench", 
    "Deadlift", 
    "Total"
];

impl LifterEntry {
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
