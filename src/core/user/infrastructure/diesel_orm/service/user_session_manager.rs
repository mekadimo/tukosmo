use diesel::pg::PgConnection;
use std::cell::RefCell;
use std::rc::Rc;
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::user::error;
use tukosmo_domain::core::user::model::UserSession;
use tukosmo_domain::core::user::model::UserSessionId;
use tukosmo_domain::core::user::model::UserSessionSearchCriteria;
use tukosmo_domain::core::user::model::UserSessionSearchFilterCriteria;

use crate::core::shared::diesel_orm::statement::ModelSqlExecutor;
use super::super::model::DbUserSession;
use super::super::statement::UserSessionSqlExecutor;

pub struct UserSessionManager {
    user_session_sql: UserSessionSqlExecutor,
}

impl UserSessionManager {
    pub fn add(
        &mut self,
        user_session: UserSession
    ) -> Result<(), DomainError> {
        let db_user_session = DbUserSession::from_domain(user_session.clone());
        self.user_session_sql.insert(db_user_session)?;

        Ok(())
    }

    pub fn count(
        &mut self,
        filter_criteria: UserSessionSearchFilterCriteria
    ) -> Result<i64, DomainError> {
        let total = self.user_session_sql.select_count(filter_criteria)?;
        Ok(total)
    }

    pub fn delete(
        &mut self,
        user_session_id: UserSessionId
    ) -> Result<(), DomainError> {
        let user_session = self.get(user_session_id)?;
        self.user_session_sql.delete(
            UserSessionSearchCriteria::has_id(user_session.id).filter
        )?;

        Ok(())
    }

    pub fn exists(
        &mut self,
        filter_criteria: UserSessionSearchFilterCriteria
    ) -> Result<bool, DomainError> {
        let exists = self.user_session_sql.select_exists(filter_criteria)?;
        Ok(exists)
    }

    pub fn find(
        &mut self,
        search_criteria: UserSessionSearchCriteria
    ) -> Result<Vec<UserSession>, DomainError> {
        let db_user_sessions = self.user_session_sql.select(search_criteria)?;
        let user_sessions = db_user_sessions
            .iter()
            .map(|s| s.to_domain())
            .collect();

        Ok(user_sessions)
    }

    pub fn get(
        &mut self,
        user_session_id: UserSessionId
    ) -> Result<UserSession, DomainError> {
        let db_user_sessions = self.user_session_sql.select(
            UserSessionSearchCriteria::has_id(user_session_id)
        )?;
        let db_user_session = db_user_sessions
            .first()
            .cloned()
            .ok_or(error::USER_SESSION_NOT_FOUND)?;
        let user_session = db_user_session.to_domain();

        Ok(user_session)
    }

    pub fn init(connection: Rc<RefCell<PgConnection>>) -> Self {
        Self {
            user_session_sql: UserSessionSqlExecutor::init(connection),
        }
    }

    pub fn update(
        &mut self,
        user_session: UserSession
    ) -> Result<(), DomainError> {
        let db_user_session = DbUserSession::from_domain(user_session.clone());
        self.user_session_sql.update(&db_user_session)?;

        Ok(())
    }
}
