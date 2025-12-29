use anyhow::{bail, Context, Result};
use migrations::{Asterisk, Expr, Query, SelectStatement};
use sea_orm::{ColumnTrait, Condition, ConnectOptions, ConnectionTrait, Database, DatabaseConnection, EntityTrait, JoinType, Order, Statement};
use tracing::debug;
use types::filters::{DivisionFilterDto, FederationFilterDto, QueryDto, SexFilterDto};
use types::prelude::EntryDto;

use crate::models::read::{meet, ranked_entry};
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
            .add(ranked_entry::Column::Total.is_not_null());

        if query.federation_choice != FederationFilterDto::Any {
            ranks_condition = ranks_condition.add(meet::Column::Federation.eq(query.federation_choice.to_string().to_lowercase()));
        };

        if query.sex_choice != SexFilterDto::Any {
            ranks_condition = ranks_condition.add(ranked_entry::Column::Sex.eq(query.sex_choice.to_string().to_lowercase()));
        };

        if query.division_choice != DivisionFilterDto::Any {
            ranks_condition = ranks_condition.add(ranked_entry::Column::Division.eq(query.division_choice.to_string().to_lowercase()));
        };

        ranks_condition = ranks_condition.add(ranked_entry::Column::Equipment.eq(query.equipment_choice.to_string().to_lowercase()));

        let ranks: SelectStatement = Query::select()
            .from(ranked_entry::Entity)
            .distinct_on([ranked_entry::Column::Name.into_qualified()])
            .qualified_column(ranked_entry::Column::Id)
            .qualified_column(ranked_entry::Column::Name)
            .qualified_column(ranked_entry::Column::Total)
            .join(
                JoinType::LeftJoin, 
                meet::Entity,
                Expr::col(ranked_entry::Column::MeetId.into_qualified())
                    .equals(meet::Column::Id.into_qualified())
            )
            .cond_where(ranks_condition)
            .order_by_columns([
                (ranked_entry::Column::Name.into_qualified(), Order::Asc),
                (ranked_entry::Column::Total.into_qualified(), Order::Desc),
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
                (ranked_entry::Column::Total, Order::Desc),
            ])
            .to_owned();

        let mut condition: Condition = Condition::any();

        for line in query.powerlifters.lines() {
            let mut part_condition = Condition::all();

            for part in line.split_whitespace() {
                part_condition = part_condition.add(ranked_entry::Column::Name.contains(part));
            }

            condition = condition.add(part_condition);
        }

        let result: SelectStatement = Query::select()
            .from(ranked_entry::Entity)
            .column(ranked_entry::Column::Rank)
            .qualified_column(ranked_entry::Column::Id)
            .qualified_column(ranked_entry::Column::MeetId)
            .qualified_column(ranked_entry::Column::Name)
            .qualified_column_casted(ranked_entry::Column::Division, "text")
            .qualified_column_casted(ranked_entry::Column::Equipment, "text")
            .qualified_column_casted(ranked_entry::Column::Sex, "text")
            .qualified_column(ranked_entry::Column::Bodyweight)
            .qualified_column(ranked_entry::Column::WeightClass)
            .qualified_column(ranked_entry::Column::Squat1)
            .qualified_column(ranked_entry::Column::Squat2)
            .qualified_column(ranked_entry::Column::Squat3)
            .qualified_column(ranked_entry::Column::Squat4)
            .qualified_column(ranked_entry::Column::Bench1)
            .qualified_column(ranked_entry::Column::Bench2)
            .qualified_column(ranked_entry::Column::Bench3)
            .qualified_column(ranked_entry::Column::Bench4)
            .qualified_column(ranked_entry::Column::Deadlift1)
            .qualified_column(ranked_entry::Column::Deadlift2)
            .qualified_column(ranked_entry::Column::Deadlift3)
            .qualified_column(ranked_entry::Column::Deadlift4)
            .qualified_column(ranked_entry::Column::BestSquat)
            .qualified_column(ranked_entry::Column::BestBench)
            .qualified_column(ranked_entry::Column::BestDeadlift)
            .qualified_column(ranked_entry::Column::Total)
            .join_subquery(
                JoinType::RightJoin, 
                ranks,
                "ranks", 
                Expr::col(("ranks", ranked_entry::Column::Id))
                .equals(ranked_entry::Column::Id.into_qualified())
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
            .map(ranked_entry::Model::into)
            .collect();

        Ok(sea_entries)
    }
}
