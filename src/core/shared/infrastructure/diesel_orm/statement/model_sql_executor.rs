use diesel::RunQueryDsl;
use diesel::dsl::count_star;
use diesel::dsl::exists;
use diesel::dsl::select;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::prelude::Insertable;
use diesel::query_builder::BoxedSelectStatement;
use diesel::query_builder::FromClause;
use diesel::query_builder::InsertStatement;
use diesel::query_dsl::methods::ExecuteDsl;
use diesel;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use tukosmo_domain::core::shared::error;
use tukosmo_domain::core::shared::model::DomainError;

pub trait ModelSqlExecutor<DbModel, Rows, SearchCriteria, SearchFilterCriteria, Table>
    where
        DbModel: Insertable<Table> +
            diesel::query_builder::UndecoratedInsertRecord<Table>,
        InsertStatement<Table, DbModel::Values>: ExecuteDsl<PgConnection>,
        Table: diesel::Table + diesel::query_builder::QueryId + 'static,
        <Table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<Pg>,
        diesel::query_builder::BatchInsert<
            Vec<<DbModel as Insertable<Table>>::Values>,
            Table,
            (),
            false
        >: diesel::query_builder::QueryFragment<
            Pg,
            diesel::backend::sql_dialect::batch_insert_support::PostgresLikeBatchInsertSupport
        >
{
    // TODO: Generalize implementation using generic types
    fn delete(
        &mut self,
        search_criteria: SearchFilterCriteria
    ) -> Result<(), DomainError>;

    fn get_connection(&mut self) -> Rc<RefCell<PgConnection>>;

    fn get_table() -> Table;

    fn init(connection: Rc<RefCell<PgConnection>>) -> Self;

    fn insert(&mut self, db_model: DbModel) -> Result<(), DomainError> {
        let connection = self.get_connection();
        let mut connection = connection.borrow_mut();
        let connection = connection.deref_mut();

        let table = Self::get_table();

        let result = diesel
            ::insert_into(table)
            .values(db_model)
            .execute(connection);

        match result {
            Ok(_inserted_rows) => Ok(()),
            Err(_e) => Err(error::CANNOT_EXECUTE_INSERT_ON_DATABASE),
        }
    }

    fn insert_in_bulk(
        &mut self,
        db_models: Vec<DbModel>
    ) -> Result<(), DomainError> {
        let connection = self.get_connection();
        let mut connection = connection.borrow_mut();
        let connection = connection.deref_mut();

        let table = Self::get_table();

        let result = diesel
            ::insert_into(table)
            .values(db_models)
            .execute(connection);

        match result {
            Ok(_inserted_rows) => Ok(()),
            Err(_e) => Err(error::CANNOT_EXECUTE_INSERT_ON_DATABASE),
        }
    }

    // TODO: Generalize implementation using generic types
    fn select(
        &mut self,
        search_criteria: SearchCriteria
    ) -> Result<Vec<DbModel>, DomainError>;

    fn select_count(
        &mut self,
        filter_criteria: SearchFilterCriteria
    ) -> Result<i64, DomainError> {
        let connection = self.get_connection();
        let mut connection = connection.borrow_mut();
        let connection = connection.deref_mut();

        use diesel::QueryDsl;
        let query = Self::select_query(filter_criteria);
        let total = query
            .select(count_star())
            .first(connection)
            .map_err(|_e| error::CANNOT_EXECUTE_SELECT_ON_DATABASE)?;
        Ok(total)
    }

    fn select_exists(
        &mut self,
        filter_criteria: SearchFilterCriteria
    ) -> Result<bool, DomainError> {
        let connection = self.get_connection();
        let mut connection = connection.borrow_mut();
        let connection = connection.deref_mut();

        let query = Self::select_query(filter_criteria);
        let exists = select(exists(query))
            .get_result(connection)
            .map_err(|_e| error::CANNOT_EXECUTE_SELECT_ON_DATABASE)?;
        Ok(exists)
    }

    fn select_query<'a>(
        filter_criteria: SearchFilterCriteria
    ) -> BoxedSelectStatement<'a, Rows, FromClause<Table>, Pg>;

    // TODO: Generalize implementation using generic types
    fn update(&mut self, db_model: &DbModel) -> Result<(), DomainError>;

    // TODO: Generalize implementation using generic types
    fn upsert_in_bulk(
        &mut self,
        db_models: Vec<DbModel>
    ) -> Result<(), DomainError>;
}
