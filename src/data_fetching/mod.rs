pub struct PowerlifterData {
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

pub fn get_powerlifter_data(data: &String) -> Vec<PowerlifterData> {
    Vec::new()
}
