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
use tukosmo_domain::core::user::model::UserSessionSearchCriteria;
use tukosmo_domain::core::user::model::UserSessionSearchCriteriaOrderBy;
use tukosmo_domain::core::user::model::UserSessionSearchFilterCriteria;

use crate::core::shared::diesel_orm::schema::user_session;
use crate::core::shared::diesel_orm::statement::ModelSqlExecutor;
use super::super::model::DbUserSession;

pub struct UserSessionSqlExecutor {
    connection: Rc<RefCell<PgConnection>>,
}

impl ModelSqlExecutor<
    DbUserSession,
    (
        diesel::sql_types::Timestamptz,
        diesel::sql_types::Uuid,
        diesel::sql_types::Uuid,
        diesel::sql_types::Text,
        diesel::sql_types::Timestamptz,
        diesel::sql_types::Text,
        diesel::sql_types::Uuid,
    ),
    UserSessionSearchCriteria,
    UserSessionSearchFilterCriteria,
    user_session::table
>
for UserSessionSqlExecutor {
    const TABLE: user_session::table = user_session::table;

    fn delete(
        &mut self,
        filter_criteria: UserSessionSearchFilterCriteria
    ) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let mut statement = diesel::delete(user_session::table).into_boxed();

        if let Some(user_session_id) = filter_criteria.id {
            statement = statement.filter(
                user_session::id.eq(user_session_id.value().clone())
            );
        }
        if let Some(not_user_session_id) = filter_criteria.not_id {
            statement = statement.filter(
                user_session::id.ne(not_user_session_id.value().clone())
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
        search_criteria: UserSessionSearchCriteria
    ) -> Result<Vec<DbUserSession>, DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let mut query = Self::select_query(search_criteria.filter);
        if let Some(order_by) = search_criteria.order_by {
            match order_by {
                UserSessionSearchCriteriaOrderBy::CreationDateAsc => {
                    query = query.order(user_session::creation_date.asc());
                }
                UserSessionSearchCriteriaOrderBy::CreationDateDesc => {
                    query = query.order(user_session::creation_date.desc());
                }
                UserSessionSearchCriteriaOrderBy::LastRequestDateAsc => {
                    query = query.order(user_session::last_request_date.asc());
                }
                UserSessionSearchCriteriaOrderBy::LastRequestDateDesc => {
                    query = query.order(user_session::last_request_date.desc());
                }
            }
        }

        let select = query.select(DbUserSession::as_select());

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
        filter_criteria: UserSessionSearchFilterCriteria
    ) -> BoxedSelectStatement<
        'a,
        (
            diesel::sql_types::Timestamptz,
            diesel::sql_types::Uuid,
            diesel::sql_types::Uuid,
            diesel::sql_types::Text,
            diesel::sql_types::Timestamptz,
            diesel::sql_types::Text,
            diesel::sql_types::Uuid,
        ),
        FromClause<user_session::table>,
        Pg
    > {
        let mut query = user_session::table.into_boxed();

        if let Some(user_session_id) = filter_criteria.id {
            query = query.filter(
                user_session::id.eq(user_session_id.value().clone())
            );
        }
        if let Some(not_user_session_id) = filter_criteria.not_id {
            query = query.filter(
                user_session::id.ne(not_user_session_id.value().clone())
            );
        }

        query
    }

    fn update(
        &mut self,
        db_user_session: &DbUserSession
    ) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let result = diesel
            ::update(user_session::table.find(db_user_session.id))
            .set(db_user_session)
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
        db_user_sessions: Vec<DbUserSession>
    ) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let result = diesel
            ::insert_into(user_session::table)
            .values(db_user_sessions)
            .on_conflict(user_session::id)
            .do_update()
            .set((
                user_session::csrf_token.eq(
                    diesel::pg::upsert::excluded(user_session::csrf_token)
                ),
                user_session::last_request_date.eq(
                    diesel::pg::upsert::excluded(
                        user_session::last_request_date
                    )
                ),
            ))
            .execute(connection);

        match result {
            Ok(_inserted_rows) => Ok(()),
            Err(_e) => Err(error::CANNOT_EXECUTE_INSERT_ON_DATABASE),
        }
    }
}
