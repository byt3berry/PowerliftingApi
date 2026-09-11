use rust_decimal::Decimal;
use search::SearchResult;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct Powerlifter {
    pub rank: i64,
    pub federation: String,
    pub name: String,
    pub equipment: String,
    pub sex: String,
    pub division: String,
    pub bodyweight: Decimal,
    pub weight_class: Option<Decimal>,
    pub best_squat: Option<Decimal>,
    pub best_bench: Option<Decimal>,
    pub best_deadlift: Option<Decimal>,
    pub total: Option<Decimal>,
}

impl From<SearchResult> for Powerlifter {
    fn from(value: SearchResult) -> Self {
        Self {
            rank: value.rank,
            federation: value.federation.to_string(),
            name: value.name.name,
            equipment: value.equipment.to_string(),
            sex: value.sex.to_string(),
            division: value.division.to_string(),
            bodyweight: value.bodyweight.into(),
            weight_class: value.weight_class.map(Decimal::from),
            best_squat: value.best_squat.map(Decimal::from),
            best_bench: value.best_bench.map(Decimal::from),
            best_deadlift: value.best_deadlift.map(Decimal::from),
            total: value.total.map(Decimal::from),
        }
    }
}
