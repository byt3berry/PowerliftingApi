use sea_orm_migration::prelude::{ColumnRef, IntoIden};
use sea_orm::prelude::{ColumnTrait};

pub trait IntoQualifiedColumn: ColumnTrait {
    fn into_qualified(self) -> ColumnRef;
}

impl<T> IntoQualifiedColumn for T
where T: ColumnTrait
{
    fn into_qualified(self) -> ColumnRef
    {
        ColumnRef::TableColumn(self.entity_name().into_iden(), self.into_iden())
    }
}
