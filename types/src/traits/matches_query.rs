use crate::Query;

pub trait MatchesQuery {
    fn matches_query(&self, query: &Query) -> bool;
}
