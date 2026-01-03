pub const DEFAULT_OUTPUT: &str = "?";

pub struct ExportRow {
    pub rank: String,
    pub federation: String,
    pub name: String,
    pub equipment: String,
    pub sex: String,
    pub division: String,
    pub bodyweight: String,
    pub weight_class: String,
    pub best_squat: String,
    pub best_bench: String,
    pub best_deadlift: String,
    pub total: String,
}

impl ExportRow {
    pub const DEFAULT_OUTPUT: &str = "?";
}

impl Default for ExportRow {
    fn default() -> Self {
        Self {
            rank: String::from(DEFAULT_OUTPUT),
            federation: String::from(DEFAULT_OUTPUT),
            name: String::from(DEFAULT_OUTPUT),
            equipment: String::from(DEFAULT_OUTPUT),
            sex: String::from(DEFAULT_OUTPUT),
            division: String::from(DEFAULT_OUTPUT),
            bodyweight: String::from(DEFAULT_OUTPUT),
            weight_class: String::from(DEFAULT_OUTPUT),
            best_squat: String::from(DEFAULT_OUTPUT),
            best_bench: String::from(DEFAULT_OUTPUT),
            best_deadlift: String::from(DEFAULT_OUTPUT),
            total: String::from(DEFAULT_OUTPUT),
        }
    }
}
