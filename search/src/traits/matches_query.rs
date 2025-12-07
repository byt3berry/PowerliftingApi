use types::filters::QueryDto;

#[deprecated]
pub trait MatchesQuery {
    fn matches_query(&self, query: &QueryDto) -> bool;
}
