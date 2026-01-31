use anyhow::{bail, Context, Result};
use sea_orm::prelude::{ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait};
use sea_orm::{Condition, ConnectOptions, Database, JoinType, Order, Statement};
use sea_orm_migration::prelude::extension::postgres::PgExpr;
use sea_orm_migration::prelude::{Asterisk, Expr, Query, SelectStatement};
use tracing::debug;

use crate::filters::{QueryFilter};
use crate::models::read::powerlifter_entry::PowerlifterEntry;
use crate::models::read::{meet, ranked_entry};
use crate::models::types::SearchResult;
use crate::traits::{MatchFilter, IntoQualifiedColumn, QualifiedColumn};

pub struct ReadOnlyRepository {
    options: ConnectOptions,
    connection: Option<DatabaseConnection>,
}

impl ReadOnlyRepository {
    pub(crate) const fn new(options: ConnectOptions) -> Self {
        Self {
            options,
            connection: None,
        }
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

    pub async fn search(&self, query: &QueryFilter) -> Result<Vec<PowerlifterEntry>> {
        let Some(ref connection) = self.connection else {
            bail!("Can't insert meet without connecting to the database")
        };

        let ranks_condition: Condition = Condition::all()
            .add(ranked_entry::Column::Total.is_not_null())
            .add_option(query.federation_choice.eq(meet::Column::Federation))
            .add_option(query.sex_choice.eq(ranked_entry::Column::Sex))
            .add_option(query.division_choice.eq(ranked_entry::Column::Division))
            .add_option(query.equipment_choice.eq(ranked_entry::Column::Equipment));

        let ranks: SelectStatement = Query::select()
            .from(ranked_entry::Entity)
            .distinct_on([ranked_entry::Column::Name.into_qualified()])
            .qualified_column(ranked_entry::Column::Id)
            .qualified_column(ranked_entry::Column::Name)
            .qualified_column(ranked_entry::Column::Total)
            .qualified_column_casted(meet::Column::Federation, "text")
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

        let ranks: SelectStatement = Query::select()
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

        for powerlifter in query.powerlifters.iter() {
            let mut part_condition = Condition::all();

            for part in &powerlifter.parts {
                let format: String = format!("%{part}%");
                part_condition = part_condition.add(Expr::column(("ranks", ranked_entry::Column::Name)).ilike(format));
            }

            condition = condition.add(part_condition);
        }

        let result: SelectStatement = Query::select()
            .from(ranked_entry::Entity)
            .column(ranked_entry::Column::Rank)
            .column(("ranks", meet::Column::Federation))
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
        let sea_entries: Vec<PowerlifterEntry> = result
            .into_model::<PowerlifterEntry>()
            .all(connection)
            .await?;

        let mut output: Vec<PowerlifterEntry> = Vec::new();

        for powerlifter in query.powerlifters.iter() {
            let entry: Option<PowerlifterEntry> = sea_entries
                .iter()
                .find(|x| {
                    if x.name.parts.len() < powerlifter.parts.len() {
                        return false;
                    }

                    for part in &powerlifter.parts {
                        if !x.name.parts.contains(part) {
                            return false;
                        }
                    }

                    true
                })
                .cloned();

            if let Some(entry) = entry {
                output.push(entry);
            }
        }

        Ok(output)
    }
}
