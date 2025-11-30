use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryDto {
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
