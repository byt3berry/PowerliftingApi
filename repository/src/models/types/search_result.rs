use types::prelude::{DivisionDto, EquipmentDto, ExportRow, FederationDto, SexDto, UsernameDto, WeightClassDto, WeightDto};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchResult {
    pub rank: i64,
    pub name: UsernameDto,
    pub federation: FederationDto,
    pub division: DivisionDto,
    pub equipment: EquipmentDto,
    pub sex: SexDto,
    pub bodyweight: WeightDto,
    pub weight_class: Option<WeightClassDto>,
    pub squat1: Option<WeightDto>,
    pub squat2: Option<WeightDto>,
    pub squat3: Option<WeightDto>,
    pub squat4: Option<WeightDto>,
    pub bench1: Option<WeightDto>,
    pub bench2: Option<WeightDto>,
    pub bench3: Option<WeightDto>,
    pub bench4: Option<WeightDto>,
    pub deadlift1: Option<WeightDto>,
    pub deadlift2: Option<WeightDto>,
    pub deadlift3: Option<WeightDto>,
    pub deadlift4: Option<WeightDto>,
    pub best_squat: Option<WeightDto>,
    pub best_bench: Option<WeightDto>,
    pub best_deadlift: Option<WeightDto>,
    pub total: Option<WeightDto>,
}

impl From<SearchResult> for ExportRow {
    fn from(value: SearchResult) -> Self {
        Self {
            rank: value.rank.to_string(),
            federation: value.federation.to_string(),
            name: value.name.name.clone(),
            equipment: value.equipment.to_string(),
            sex: value.sex.to_string(),
            division: value.division.to_string(),
            bodyweight: value.bodyweight.to_string(),
            weight_class: value.weight_class.map_or_else(|| Self::DEFAULT_OUTPUT.to_string(), |v| v.to_string()),
            best_squat: value.best_squat.map_or_else(|| Self::DEFAULT_OUTPUT.to_string(), |v| v.to_string()),
            best_bench: value.best_bench.map_or_else(|| Self::DEFAULT_OUTPUT.to_string(), |v| v.to_string()),
            best_deadlift: value.best_deadlift.map_or_else(|| Self::DEFAULT_OUTPUT.to_string(), |v| v.to_string()),
            total: value.total.map_or_else(|| Self::DEFAULT_OUTPUT.to_string(), |v| v.to_string()),
            ..Default::default()
        }
    }
}
