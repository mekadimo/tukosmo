use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::query_builder::BoxedSelectStatement;
use diesel::query_builder::FromClause;
use diesel;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use tukosmo_domain::core::shared::error;
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::user::model::UserSearchCriteria;
use tukosmo_domain::core::user::model::UserSearchCriteriaOrderBy;
use tukosmo_domain::core::user::model::UserSearchFilterCriteria;

use crate::core::shared::diesel_orm::schema::user_;
use crate::core::shared::diesel_orm::statement::ModelSqlExecutor;
use super::super::model::DbUser;

pub struct UserSqlExecutor {
    connection: Rc<RefCell<PgConnection>>,
}

impl ModelSqlExecutor<
    DbUser,
    (
        diesel::sql_types::Timestamptz,
        diesel::sql_types::Text,
        diesel::sql_types::Text,
        diesel::sql_types::Uuid,
        diesel::sql_types::Uuid,
        diesel::sql_types::Bool,
        diesel::sql_types::Bool,
        diesel::sql_types::Timestamptz,
    ),
    UserSearchCriteria,
    UserSearchFilterCriteria,
    user_::table
>
for UserSqlExecutor {
    const TABLE: user_::table = user_::table;

    fn delete(
        &mut self,
        filter_criteria: UserSearchFilterCriteria
    ) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let mut statement = diesel::delete(user_::table).into_boxed();

        if let Some(user_id) = filter_criteria.id {
            statement = statement.filter(user_::id.eq(user_id.value().clone()));
        }
        if let Some(is_admin) = filter_criteria.is_admin {
            statement = statement.filter(
                user_::is_admin.eq(is_admin.value().clone())
            );
        }
        if let Some(email) = filter_criteria.email {
            statement = statement.filter(
                user_::email.eq(email.value().to_string())
            );
        }
        if let Some(not_user_id) = filter_criteria.not_id {
            statement = statement.filter(
                user_::id.ne(not_user_id.value().clone())
            );
        }

        let result = statement.execute(connection);

        match result {
            Ok(_affected_rows) => Ok(()),
            Err(_e) => Err(error::CANNOT_EXECUTE_DELETE_ON_DATABASE),
        }
    }

    fn get_connection(&mut self) -> Rc<RefCell<PgConnection>> {
        Rc::clone(&self.connection)
    }

    fn init(connection: Rc<RefCell<PgConnection>>) -> Self {
        Self { connection }
    }

    fn select(
        &mut self,
        search_criteria: UserSearchCriteria
    ) -> Result<Vec<DbUser>, DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let mut query = Self::select_query(search_criteria.filter);
        if let Some(order_by) = search_criteria.order_by {
            match order_by {
                UserSearchCriteriaOrderBy::CreationDate => {
                    query = query.order(user_::creation_date.asc());
                }
                UserSearchCriteriaOrderBy::Email => {
                    query = query.order(user_::email.asc());
                }
            }
        }

        let select = query.select(DbUser::as_select());

        let results = (
            if let Some(pagination) = search_criteria.pagination {
                select
                    .offset((pagination.page - 1) * pagination.results_per_page)
                    .limit(pagination.results_per_page)
            } else {
                select
            }
        )
            .load(connection)
            .map_err(|_e| error::CANNOT_EXECUTE_SELECT_ON_DATABASE)?;

        Ok(results)
    }

    fn select_query<'a>(
        filter_criteria: UserSearchFilterCriteria
    ) -> BoxedSelectStatement<
        'a,
        (
            diesel::sql_types::Timestamptz,
            diesel::sql_types::Text,
            diesel::sql_types::Text,
            diesel::sql_types::Uuid,
            diesel::sql_types::Uuid,
            diesel::sql_types::Bool,
            diesel::sql_types::Bool,
            diesel::sql_types::Timestamptz,
        ),
        FromClause<user_::table>,
        Pg
    > {
        let mut query = user_::table.into_boxed();

        if let Some(user_id) = filter_criteria.id {
            query = query.filter(user_::id.eq(user_id.value().clone()));
        }
        if let Some(is_admin) = filter_criteria.is_admin {
            query = query.filter(user_::is_admin.eq(is_admin.value().clone()));
        }
        if let Some(email) = filter_criteria.email {
            query = query.filter(user_::email.eq(email.value().to_string()));
        }
        if let Some(not_user_id) = filter_criteria.not_id {
            query = query.filter(user_::id.ne(not_user_id.value().clone()));
        }

        query
    }

    fn update(&mut self, db_user: &DbUser) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let result = diesel
            ::update(user_::table.find(db_user.id))
            .set(db_user)
            .execute(connection);

        match result {
            Ok(0) => Err(error::NOTHING_TO_UPDATE_ON_DATABASE),
            Ok(1) => Ok(()),
            Ok(_) => Err(error::UNDESIRED_UPDATES_ON_DATABASE),
            Err(_e) => Err(error::CANNOT_EXECUTE_UPDATE_ON_DATABASE),
        }
    }

    fn upsert_in_bulk(
        &mut self,
        db_users: Vec<DbUser>
    ) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let result = diesel
            ::insert_into(user_::table)
            .values(db_users)
            .on_conflict(user_::id)
            .do_update()
            .set((
                user_::email.eq(diesel::pg::upsert::excluded(user_::email)),
                user_::encrypted_password.eq(
                    diesel::pg::upsert::excluded(user_::encrypted_password)
                ),
                user_::is_admin.eq(
                    diesel::pg::upsert::excluded(user_::is_admin)
                ),
                user_::is_suspended.eq(
                    diesel::pg::upsert::excluded(user_::is_suspended)
                ),
                user_::update_date.eq(
                    diesel::pg::upsert::excluded(user_::update_date)
                ),
            ))
            .execute(connection);

        match result {
            Ok(_inserted_rows) => Ok(()),
            Err(_e) => Err(error::CANNOT_EXECUTE_INSERT_ON_DATABASE),
        }
    }
}
