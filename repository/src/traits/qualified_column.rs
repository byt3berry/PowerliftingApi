use migrations::{ExprTrait, IntoIden, SelectStatement};

use crate::traits::IntoQualifiedColumn;

pub trait QualifiedColumn {
    fn qualified_column<T>(&mut self, column: T) -> &mut Self
        where T: IntoQualifiedColumn;

    fn qualified_column_casted<T, C>(&mut self, column: T, cast_as: C) -> &mut Self
    where 
        T: IntoQualifiedColumn,
        C: IntoIden;
}

impl QualifiedColumn for SelectStatement {
    fn qualified_column<T>(&mut self, column: T) -> &mut Self
        where T: IntoQualifiedColumn
    {
        self.column(column.into_qualified())
    }

    fn qualified_column_casted<T, C>(&mut self, column: T, cast_as: C) -> &mut Self
    where 
        T: IntoQualifiedColumn,
        C: IntoIden
    {
        self.expr(column.into_qualified().cast_as(cast_as))
    }
}
