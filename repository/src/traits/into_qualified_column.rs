use migrations::{ColumnRef, ExprTrait, IntoIden, SimpleExpr};
use sea_orm::{ColumnTrait};

pub trait IntoQualifiedColumn: ColumnTrait {
    fn into_qualified(self) -> ColumnRef;

    fn into_qualified_casted<C>(self, cast_as: C) -> SimpleExpr
        where C: IntoIden;
}

impl<T> IntoQualifiedColumn for T
where T: ColumnTrait
{
    fn into_qualified(self) -> ColumnRef
    {
        ColumnRef::TableColumn(self.entity_name().into_iden(), self.into_iden())
    }

    fn into_qualified_casted<C>(self, cast_as: C) -> SimpleExpr
        where C: IntoIden
    {
        self.into_qualified().cast_as(cast_as)
    }
}
