use crate::data_fetching::types::lifter::Lifter;

pub struct ExportRow {
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

impl From<(String, Option<&Lifter>)> for ExportRow {
    fn from(value: (String, Option<&Lifter>)) -> Self {
        if let Some(lifter) = value.1 {
            Self {
                name: lifter.name.to_string(),
                equipment: lifter.equipment.to_string(),
                sex: lifter.sex.to_string(),
                division: lifter.division.to_string(),
                bodyweight: lifter.best_meet.bodyweight.to_string(),
                weight_class: lifter.best_meet.weight_class.map_or_else(|| "?".to_string(), |inner| inner.to_string()),
                best_squat: lifter.best_meet.best3squat.map_or_else(|| "?".to_string(), |inner| inner.to_string()),
                best_bench: lifter.best_meet.best3bench.map_or_else(|| "?".to_string(), |inner| inner.to_string()),
                best_deadlift: lifter.best_meet.best3deadlift.map_or_else(|| "?".to_string(), |inner| inner.to_string()),
                total: lifter.best_meet.total.to_string(),
            }
        } else {
            Self {
                name: value.0,
                ..Default::default()
            }
        }
    }
}
