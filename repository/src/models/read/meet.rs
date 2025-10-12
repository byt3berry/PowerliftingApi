use diesel::pg::Pg;
use diesel::{Queryable, Selectable};

use crate::models::types::country::Country;
use crate::models::types::federation::Federation;
use crate::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::meets)]
#[diesel(check_for_backend(Pg))]
pub struct Meet {
    pub id: i32,
    name: String,
    federation: Federation,
    country: Country,
    state: String,
    town: String,
}
