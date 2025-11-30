use crate::filters::Query;

#[deprecated]
pub trait MatchesQuery {
    fn matches_query(&self, query: &Query) -> bool;
}
