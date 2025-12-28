use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryDto {
    pub rank: Option<i64>,
    pub name: UsernameDto,
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

impl From<EntryDto> for ExportRow {
    fn from(value: EntryDto) -> ExportRow {
        Self {
            rank: value.rank.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string()),
            name: value.name.name.clone(),
            equipment: value.equipment.to_string(),
            sex: value.sex.to_string(),
            division: value.division.to_string(),
            bodyweight: value.bodyweight.0.to_string(),
            weight_class: value.weight_class.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string()),
            best_squat: value.best_squat.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string()),
            best_bench: value.best_bench.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string()),
            best_deadlift: value.best_deadlift.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string()),
            total: value.total.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string()),
            ..Default::default()
        }
    }
}
