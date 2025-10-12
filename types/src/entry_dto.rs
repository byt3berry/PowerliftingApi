use crate::{Division, Equipment, Sex, Username, Weight, WeightClass};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryDto {
    pub name: Username,
    pub division: Division,
    pub equipment: Equipment,
    pub sex: Sex,
    pub bodyweight: Weight,
    pub weight_class: Option<WeightClass>,
    pub squat1: Option<Weight>,
    pub squat2: Option<Weight>,
    pub squat3: Option<Weight>,
    pub squat4: Option<Weight>,
    pub bench1: Option<Weight>,
    pub bench2: Option<Weight>,
    pub bench3: Option<Weight>,
    pub bench4: Option<Weight>,
    pub deadlift1: Option<Weight>,
    pub deadlift2: Option<Weight>,
    pub deadlift3: Option<Weight>,
    pub deadlift4: Option<Weight>,
    pub best_squat: Option<Weight>,
    pub best_bench: Option<Weight>,
    pub best_deadlift: Option<Weight>,
    pub total: Weight,
}
