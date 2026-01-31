use repository::models::read::powerlifter_entry::PowerlifterEntry;
use repository::models::types::SearchResult;
use repository::{ReadOnlyRepository, Repository};
use types::filters::QueryDto;

#[derive(Debug, Clone)]
pub struct SearchEngine;

impl SearchEngine {
    pub async fn search(&self, query: &QueryDto) -> Vec<SearchResult> {
        let mut repository: ReadOnlyRepository = Repository::read_only().unwrap();
        repository.connect().await.unwrap();
        let result: Vec<PowerlifterEntry> = repository
            .search(&query.clone().into())
            .await
            .unwrap();
        repository.disconnect().await.unwrap();

        result
            .into_iter()
            .map(SearchResult::from)
            .collect()
    }
}
