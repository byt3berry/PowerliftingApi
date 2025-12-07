use repository::{ReadOnlyRepository, Repository};
use types::filters::QueryDto;
use types::prelude::{EntryDto, ExportRow};

#[derive(Debug, Clone)]
pub struct SearchEngine;

impl SearchEngine {
    pub async fn search(&self, query: &QueryDto) -> Vec<ExportRow> {
        let mut repository: ReadOnlyRepository = Repository::read_only().unwrap();
        repository.connect().await.unwrap();
        let result: Vec<EntryDto> = repository.search(query).await.unwrap();
        repository.disconnect().await.unwrap();

        let result: Vec<ExportRow> = result
            .into_iter()
            .map(ExportRow::from)
            .collect();

        result
    }
}
