use serde::Deserialize;
use types::prelude::*;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct MeetEntry {
    #[serde(rename(deserialize = "Name"))]
    pub name: UsernameDto,

    #[serde(rename(deserialize = "Division"))]
    pub division: DivisionDto,

    #[serde(rename(deserialize = "Equipment"))]
    pub equipment: EquipmentDto,

    #[serde(rename(deserialize = "Sex"))]
    pub sex: SexDto,

    #[serde(rename(deserialize = "BodyweightKg"))]
    pub bodyweight: WeightDto,

    #[serde(rename(deserialize = "WeightClassKg"))]
    pub weight_class: Option<WeightClassDto>,

    #[serde(rename(deserialize = "Squat1Kg"))]
    pub squat1: Option<WeightDto>,

    #[serde(rename(deserialize = "Squat2Kg"))]
    pub squat2: Option<WeightDto>,

    #[serde(rename(deserialize = "Squat3Kg"))]
    pub squat3: Option<WeightDto>,

    #[serde(rename(deserialize = "Best3SquatKg"))]
    pub best3squat: Option<WeightDto>,

    #[serde(rename(deserialize = "Bench1Kg"))]
    pub bench1: Option<WeightDto>,

    #[serde(rename(deserialize = "Bench2Kg"))]
    pub bench2: Option<WeightDto>,

    #[serde(rename(deserialize = "Bench3Kg"))]
    pub bench3: Option<WeightDto>,

    #[serde(rename(deserialize = "Best3BenchKg"))]
    pub best3bench: Option<WeightDto>,

    #[serde(rename(deserialize = "Deadlift1Kg"))]
    pub deadlift1: Option<WeightDto>,

    #[serde(rename(deserialize = "Deadlift2Kg"))]
    pub deadlift2: Option<WeightDto>,

    #[serde(rename(deserialize = "Deadlift3Kg"))]
    pub deadlift3: Option<WeightDto>,

    #[serde(rename(deserialize = "Best3DeadliftKg"))]
    pub best3deadlift: Option<WeightDto>,

    #[serde(rename(deserialize = "TotalKg"))]
    pub total: WeightDto,

    #[serde(skip)]
    pub federation: FederationDto,
}

impl MeetEntry {
    pub fn with_federation(&mut self, federation: FederationDto) -> &mut Self {
        self.federation = federation;
        self
    }

    pub fn dots(&self) -> DotsDto {
        DotsDto::new(self.sex, self.bodyweight, self.total)
    }
}

impl MatchesQuery for MeetEntry {
    fn matches_query(&self, query: &Query) -> bool {
        if !self.federation.matches(&query.federation_choice) { return false; }
        if !self.equipment.matches(&query.equipment_choice) { return false; }
        if !self.sex.matches(&query.sex_choice) { return false; }
        if !self.division.matches(&query.division_choice) { return false; }
        if !self.name.matches_str(&query.powerlifters) { return false; }

        true
    }
}

impl From<MeetEntry> for EntryDto {
    fn from(value: MeetEntry) -> Self {
        Self {
            name: value.name.clone(),
            division: value.division.into(),
            equipment: value.equipment.into(),
            sex: value.sex.into(),
            bodyweight: value.bodyweight.into(),
            weight_class: value.weight_class.into(),
            squat1: value.squat1.into(),
            squat2: value.squat2.into(),
            squat3: value.squat3.into(),
            squat4: None,
            bench1: value.bench1.into(),
            bench2: value.bench2.into(),
            bench3: value.bench3.into(),
            bench4: None,
            deadlift1: value.deadlift1.into(),
            deadlift2: value.deadlift2.into(),
            deadlift3: value.deadlift3.into(),
            deadlift4: None,
            best_squat: value.best3squat.into(),
            best_bench: value.best3bench.into(),
            best_deadlift: value.best3deadlift.into(),
            total: value.total.into(),
        }
    }
}
