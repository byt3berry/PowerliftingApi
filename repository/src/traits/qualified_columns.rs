use migrations::SelectStatement;

use crate::traits::IntoQualifiedColumn;
use crate::traits::IntoQualifiedColumns;

pub trait QualifiedColumns<T>
where T: IntoQualifiedColumn
{
    fn qualified_columns(&mut self, columns: impl IntoQualifiedColumns<T>) -> &mut Self;
}

impl<T> QualifiedColumns<T> for SelectStatement
where T: IntoQualifiedColumn
{
    fn qualified_columns(&mut self, columns: impl IntoQualifiedColumns<T>) -> &mut Self {
        self.columns(columns.into_qualified())
    }
}
