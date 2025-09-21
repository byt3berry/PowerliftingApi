use crate::entries::search_result::SearchResult;

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

impl<'db> From<SearchResult<'db>> for ExportRow {
    fn from(value: SearchResult<'db>) -> Self {
        if let (Some(rank), Some(meet)) = (value.rank, value.meet_entry) {
            Self {
                rank: rank.to_string(),
                name: value.name.to_string(),
                equipment: meet.equipment.to_string(),
                sex: meet.sex.to_string(),
                division: meet.division.to_string(),
                bodyweight: meet.bodyweight.to_string(),
                weight_class: meet.weight_class.map_or_else(|| "?".to_string(), |inner| inner.to_string()),
                best_squat: meet.best3squat.map_or_else(|| "?".to_string(), |inner| inner.to_string()),
                best_bench: meet.best3bench.map_or_else(|| "?".to_string(), |inner| inner.to_string()),
                best_deadlift: meet.best3deadlift.map_or_else(|| "?".to_string(), |inner| inner.to_string()),
                total: meet.total.to_string(),
            }
        } else {
            Self {
                name: value.name.to_string(),
                ..Default::default()
            }
        }
    }
}
