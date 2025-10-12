// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "country"))]
    pub struct Country;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "division"))]
    pub struct Division;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "equipment"))]
    pub struct Equipment;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "federation"))]
    pub struct Federation;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "sex"))]
    pub struct Sex;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "weight"))]
    pub struct Weight;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "weightclass"))]
    pub struct Weightclass;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Division;
    use super::sql_types::Equipment;
    use super::sql_types::Sex;
    use super::sql_types::Weight;
    use super::sql_types::Weightclass;

    entries (id) {
        id -> Int4,
        meet_id -> Int4,
        #[max_length = 256]
        name -> Varchar,
        division -> Division,
        equipment -> Equipment,
        sex -> Sex,
        bodyweight -> Weight,
        weight_class -> Weightclass,
        squat1 -> Weight,
        squat2 -> Weight,
        squat3 -> Weight,
        squat4 -> Weight,
        bench1 -> Weight,
        bench2 -> Weight,
        bench3 -> Weight,
        bench4 -> Weight,
        deadlift1 -> Weight,
        deadlift2 -> Weight,
        deadlift3 -> Weight,
        deadlift4 -> Weight,
        best_squat -> Weight,
        best_bench -> Weight,
        best_deadlift -> Weight,
        total -> Weight,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Federation;
    use super::sql_types::Country;

    meets (id) {
        id -> Int4,
        #[max_length = 256]
        name -> Varchar,
        federation -> Federation,
        country -> Country,
        #[max_length = 256]
        state -> Varchar,
        #[max_length = 256]
        town -> Varchar,
    }
}

diesel::joinable!(entries -> meets (meet_id));

diesel::allow_tables_to_appear_in_same_query!(entries, meets,);
