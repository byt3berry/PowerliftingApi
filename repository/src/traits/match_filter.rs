use sea_orm::prelude::ColumnTrait;
use sea_orm_migration::prelude::SimpleExpr;

pub trait MatchFilter {
    fn eq<T>(self, column: T) -> Option<SimpleExpr>
        where T: ColumnTrait;
}
