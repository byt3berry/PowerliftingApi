use migrations::SimpleExpr;
use sea_orm::ColumnTrait;

pub trait MatchFilter {
    fn eq<T>(self, column: T) -> Option<SimpleExpr>
        where T: ColumnTrait;
}
