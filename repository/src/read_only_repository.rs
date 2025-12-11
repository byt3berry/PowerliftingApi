use anyhow::{bail, Context, Result};
use sea_orm::{ColumnTrait, Condition, ConnectOptions, Database, DatabaseConnection, DbBackend, EntityTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Select};
use tracing::debug;
use types::filters::{DivisionFilterDto, EquipmentFilterDto, FederationFilterDto, QueryDto, SexFilterDto};
use types::prelude::EntryDto;

use crate::models::types::Entry;
use crate::models::{SeaColumnEntry, SeaColumnMeet, SeaEntityEntry, SeaEntityMeet, SeaEntry};

pub struct ReadOnlyRepository {
    options: ConnectOptions,
    connection: Option<DatabaseConnection>,
}

impl ReadOnlyRepository {
    pub(crate) fn new(options: ConnectOptions) -> Result<Self> {
        Ok(Self {
            options,
            connection: None,
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        match Database::connect(self.options.clone()).await {
            Ok(connection) => {
                self.connection = Some(connection);
                Ok(())
            },
            Err(e) => Err(e).context("failed to connect to database"),
        }
    }

    pub async fn disconnect(self) -> Result<()> {
        if let Some(connection) = self.connection {
            connection.close().await?;
        }

        Ok(())
    }

    pub async fn search(&self, query: &QueryDto) -> Result<Vec<EntryDto>> {
        let Some(ref connection) = self.connection else {
            bail!("Can't insert meet without connecting to the database")
        };

        let mut result: Select<SeaEntityEntry> = SeaEntityEntry::find()
            .left_join(SeaEntityMeet)
            .distinct_on([SeaColumnEntry::Name]);

        let mut condition: Condition = Condition::all();

        if query.federation_choice != FederationFilterDto::Any {
            condition = condition.add(SeaColumnMeet::Federation.eq(query.federation_choice.to_string().to_lowercase()));
        };

        if query.sex_choice != SexFilterDto::Any {
            condition = condition.add(SeaColumnEntry::Sex.eq(query.sex_choice.to_string().to_lowercase()));
        };

        if query.division_choice != DivisionFilterDto::Any {
            condition = condition.add(SeaColumnEntry::Division.eq(query.division_choice.to_string().to_lowercase()));
        };

        if query.equipment_choice != EquipmentFilterDto::Any {
            condition = condition.add(SeaColumnEntry::Equipment.eq(query.equipment_choice.to_string().to_lowercase()));
        };

        condition = condition.add(SeaColumnEntry::Name.is_in(query.powerlifters.lines()));

        result = result.filter(condition)
                       .order_by_desc(SeaColumnEntry::Name)
                       .order_by_desc(SeaColumnEntry::Total);
        let sea_entries: Vec<SeaEntry> = result.all(connection).await?;

        Ok(sea_entries
            .into_iter()
            .map(Entry::from)
            .map(EntryDto::from)
            .collect())
    }
}
