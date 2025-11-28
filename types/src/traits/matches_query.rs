use crate::prelude::*;

pub trait MatchesQuery {
    fn matches_query(&self, query: &Query) -> bool;
}
