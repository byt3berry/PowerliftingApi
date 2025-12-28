use anyhow::{bail, Context, Result};
use migrations::{Asterisk, ColumnRef, Expr, IntoIden, OrderExpr, Query, SelectStatement};
use sea_orm::{ColumnAsExpr, ColumnTrait, Condition, ConnectOptions, ConnectionTrait, Database, DatabaseConnection, EntityTrait, JoinType, Order, OrderedStatement, Statement};
use tracing::debug;
use types::filters::{DivisionFilterDto, EquipmentFilterDto, FederationFilterDto, QueryDto, SexFilterDto};
use types::prelude::EntryDto;

use crate::models::read::{ranked_entry};
use crate::models::types::Entry;
use crate::models::{SeaColumnEntry, SeaColumnMeet, SeaEntityEntry, SeaEntityMeet};
use crate::traits::{IntoQualifiedColumn, QualifiedColumn};

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

        let mut ranks_condition: Condition = Condition::all()
            .add(SeaColumnEntry::Total.is_not_null());

        if query.federation_choice != FederationFilterDto::Any {
            ranks_condition = ranks_condition.add(SeaColumnMeet::Federation.eq(query.federation_choice.to_string().to_lowercase()));
        };

        if query.sex_choice != SexFilterDto::Any {
            ranks_condition = ranks_condition.add(SeaColumnEntry::Sex.eq(query.sex_choice.to_string().to_lowercase()));
        };

        if query.division_choice != DivisionFilterDto::Any {
            ranks_condition = ranks_condition.add(SeaColumnEntry::Division.eq(query.division_choice.to_string().to_lowercase()));
        };

        ranks_condition = ranks_condition.add(SeaColumnEntry::Equipment.eq(query.equipment_choice.to_string().to_lowercase()));

        let ranks: SelectStatement = Query::select()
            .from(SeaEntityEntry)
            .distinct_on([SeaColumnEntry::Name.into_qualified()])
            .qualified_column(SeaColumnEntry::Id)
            .qualified_column(SeaColumnEntry::Name)
            .qualified_column(SeaColumnEntry::Total)
            // .expr_as(
            //     Expr::cust("DENSE_RANK() OVER (ORDER BY \"entries\".\"total\" DESC)"),
            //     ranked_entry::Column::Rank,
            // )
            .join(
                JoinType::LeftJoin, 
                SeaEntityMeet,
                Expr::col(SeaColumnEntry::MeetId.into_qualified())
                    .equals(SeaColumnMeet::Id.into_qualified())
            )
            .cond_where(ranks_condition)
            .order_by_columns([
                (SeaColumnEntry::Name.into_qualified(), Order::Asc),
                (SeaColumnEntry::Total.into_qualified(), Order::Desc),
                // (ColumnRef::Column(ranked_entry::Column::Rank.into_iden()), Order::Asc),
            ])
            .to_owned();

        let ranks: SelectStatement =Query::select()
            .from_subquery(ranks, "ranks")
            .column(Asterisk)
            .expr_as(
                Expr::cust("ROW_NUMBER() OVER (ORDER BY \"total\" DESC)"),
                ranked_entry::Column::Rank,
            )
            .order_by_columns([
                (SeaColumnEntry::Total, Order::Desc),
            ])
            .to_owned();

        let mut condition: Condition = Condition::any();

        for line in query.powerlifters.lines() {
            let mut part_condition = Condition::all();

            for part in line.split_whitespace() {
                part_condition = part_condition.add(SeaColumnEntry::Name.contains(part));
            }

            condition = condition.add(part_condition);
        }

        let result: SelectStatement = Query::select()
            .from(SeaEntityEntry)
            .column(ranked_entry::Column::Rank)
            .qualified_column(SeaColumnEntry::Id)
            .qualified_column(SeaColumnEntry::MeetId)
            .qualified_column(SeaColumnEntry::Name)
            .qualified_column_casted(SeaColumnEntry::Division, "text")
            .qualified_column_casted(SeaColumnEntry::Equipment, "text")
            .qualified_column_casted(SeaColumnEntry::Sex, "text")
            .qualified_column(SeaColumnEntry::Bodyweight)
            .qualified_column(SeaColumnEntry::WeightClass)
            .qualified_column(SeaColumnEntry::Squat1)
            .qualified_column(SeaColumnEntry::Squat2)
            .qualified_column(SeaColumnEntry::Squat3)
            .qualified_column(SeaColumnEntry::Squat4)
            .qualified_column(SeaColumnEntry::Bench1)
            .qualified_column(SeaColumnEntry::Bench2)
            .qualified_column(SeaColumnEntry::Bench3)
            .qualified_column(SeaColumnEntry::Bench4)
            .qualified_column(SeaColumnEntry::Deadlift1)
            .qualified_column(SeaColumnEntry::Deadlift2)
            .qualified_column(SeaColumnEntry::Deadlift3)
            .qualified_column(SeaColumnEntry::Deadlift4)
            .qualified_column(SeaColumnEntry::BestSquat)
            .qualified_column(SeaColumnEntry::BestBench)
            .qualified_column(SeaColumnEntry::BestDeadlift)
            .qualified_column(SeaColumnEntry::Total)
            .join_subquery(
                JoinType::RightJoin, 
                ranks,
                "ranks", 
                Expr::col(("ranks", ranked_entry::Column::Id))
                .equals(SeaColumnEntry::Id.into_qualified())
            )
            .order_by(ranked_entry::Column::Rank, sea_orm::Order::Asc)
            .cond_where(condition)
            .to_owned();

        let statement: Statement = connection.get_database_backend().build(&result);
        debug!("sql query:\n{:?}", statement.to_string());
        let result = ranked_entry::Entity::find().from_raw_sql(statement);
        let sea_entries: Vec<EntryDto> = result
            .all(connection)
            .await?
            .into_iter()
            .map(Entry::from)
            .map(EntryDto::from)
            .collect();

        Ok(sea_entries)
    }
}
