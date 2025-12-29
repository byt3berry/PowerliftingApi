use migrations::ColumnRef;

use crate::traits::IntoQualifiedColumn;

pub trait IntoQualifiedColumns<T>: Iterator<Item = T>
where T: IntoQualifiedColumn
{
    fn into_qualified(self) -> Vec<ColumnRef>;
}

impl<T, U> IntoQualifiedColumns<U> for T
where
    T: Iterator<Item = U>,
    U: IntoQualifiedColumn,
{
    fn into_qualified(self) -> Vec<ColumnRef> {
        self.map(IntoQualifiedColumn::into_qualified).collect()
    }
}
