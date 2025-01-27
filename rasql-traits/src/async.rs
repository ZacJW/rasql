use crate::DbTraits;

pub trait Client {
    type Traits: DbTraits;


    type PrepareError;
    type PreparedStatement;

    type Rows;
    type Row;

    type RowReadColumnError;

    type QueryError;
    type InsertOutcome;
    type InsertError;
    type UpdateOutcome;
    type UpdateError;
    type DeleteOutcome;
    type DeleteError;
}

#[cfg(feature = "tokio-postgres")]
impl Client for tokio_postgres::Client {
    type Traits = crate::PostgresTypesTraits;

    type PrepareError = tokio_postgres::Error;

    type PreparedStatement = tokio_postgres::Statement;

    type Rows = Vec<tokio_postgres::Row>;

    type Row = tokio_postgres::Row;

    type RowReadColumnError = tokio_postgres::Error;

    type QueryError = tokio_postgres::Error;

    type InsertOutcome = u64;

    type InsertError = tokio_postgres::Error;

    type UpdateOutcome = u64;

    type UpdateError = tokio_postgres::Error;

    type DeleteOutcome = u64;

    type DeleteError = tokio_postgres::Error;
}

/// Base trait that all table types implement. Allows for inserting rows and selecting all rows.
#[allow(async_fn_in_trait)]
pub trait Table<C: Client>: Sized {
    const SCHEMA: Option<&str>;
    const NAME: &str;

    type SelectAllStatement;

    async fn prepare_select_all(client: &C) -> Result<Self::SelectAllStatement, C::PrepareError>;

    /// Query all rows in the database table and load them into `Container`.
    async fn select_all<Container: FromIterator<Self>>(
        client: &C,
        select_all_statement: &Self::SelectAllStatement,
    ) -> Result<Container, C::QueryError>;

    type InsertStatement;

    async fn prepare_insert(client: &C) -> Result<Self::InsertStatement, C::PrepareError>;

    /// Insert this value into the database table.
    async fn insert(
        &self,
        client: &C,
        insert_statement: &Self::InsertStatement,
    ) -> Result<C::InsertOutcome, C::InsertError>;
}

/// Trait that all table types with a primary key implement.
/// 
/// In addition to what's allowed by [Table], also allows
/// updating, single row select, and deletion.
#[allow(async_fn_in_trait)]
pub trait TableWithPK<C: Client>: Table<C> {
    type PrimaryKeyRef<'a>: From<&'a Self>
    where
        Self: 'a;

    type UpdateStatement;

    async fn prepare_update(client: &C) -> Result<Self::UpdateStatement, C::PrepareError>;

    /// Update the row in the database table whose primary key is equal to this value'sm making the rest of the row equal too.
    async fn update(
        &self,
        client: &C,
        update_statement: &Self::UpdateStatement,
    ) -> Result<C::UpdateOutcome, C::UpdateError>;

    type SelectByPKStatement;

    async fn prepare_select_by_pk(client: &C)
        -> Result<Self::SelectByPKStatement, C::PrepareError>;

    async fn select_by_pk(
        client: &C,
        select_by_pk_statement: &Self::SelectByPKStatement,
        pk: Self::PrimaryKeyRef<'_>,
    ) -> Result<Option<Self>, C::QueryError>;

    type DeleteByPKStatememt;

    async fn prepare_delete_by_pk(client: &C)
        -> Result<Self::DeleteByPKStatememt, C::PrepareError>;

    async fn delete_by_pk(
        client: &C,
        delete_by_pk_statement: &Self::DeleteByPKStatememt,
        pk: Self::PrimaryKeyRef<'_>,
    ) -> Result<C::DeleteOutcome, C::DeleteError>;

    async fn delete_using_pk(
        &self,
        client: &C,
        delete_by_pk_statement: &Self::DeleteByPKStatememt,
    ) -> Result<C::DeleteOutcome, C::DeleteError> {
        Self::delete_by_pk(
            client,
            delete_by_pk_statement,
            Self::PrimaryKeyRef::from(self),
        )
        .await
    }
}
