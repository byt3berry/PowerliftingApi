pub struct ExportRow {
    pub rank: String,
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

impl Default for ExportRow {
    fn default() -> Self {
        Self {
            rank: String::from("?"),
            name: String::from("?"),
            equipment: String::from("?"),
            sex: String::from("?"),
            division: String::from("?"),
            bodyweight: String::from("?"),
            weight_class: String::from("?"),
            best_squat: String::from("?"),
            best_bench: String::from("?"),
            best_deadlift: String::from("?"),
            total: String::from("?"),
        }
    }
}
