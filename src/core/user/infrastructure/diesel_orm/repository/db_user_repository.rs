use diesel::pg::PgConnection;
use std::cell::RefCell;
use std::rc::Rc;
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::user::model::User;
use tukosmo_domain::core::user::model::UserEmail;
use tukosmo_domain::core::user::model::UserId;
use tukosmo_domain::core::user::model::UserPlaintextPassword;
use tukosmo_domain::core::user::model::UserSearchCriteria;
use tukosmo_domain::core::user::model::UserSearchFilterCriteria;
use tukosmo_domain::core::user::model::UserSession;
use tukosmo_domain::core::user::model::UserSessionId;
use tukosmo_domain::core::user::model::UserSessionSearchCriteria;
use tukosmo_domain::core::user::repository::UserRepository;

use super::super::service::UserManager;
use super::super::service::UserSessionManager;

pub struct DbUserRepository {
    user_manager: UserManager,
    user_session_manager: UserSessionManager,
}

impl DbUserRepository {
    pub fn init(connection: Rc<RefCell<PgConnection>>) -> Self {
        Self {
            user_manager: UserManager::init(Rc::clone(&connection)),
            user_session_manager: UserSessionManager::init(connection),
        }
    }
}

impl UserRepository for DbUserRepository {
    fn add(
        &mut self,
        user: User,
        plaintext_password: UserPlaintextPassword
    ) -> Result<(), DomainError> {
        self.user_manager.add(user, plaintext_password)?;
        Ok(())
    }

    fn change_password(
        &mut self,
        user_id: UserId,
        plaintext_old_password: UserPlaintextPassword,
        plaintext_password: UserPlaintextPassword
    ) -> Result<(), DomainError> {
        self.user_manager.change_password(
            user_id,
            plaintext_old_password,
            plaintext_password
        )?;
        Ok(())
    }

    fn count(
        &mut self,
        filter_criteria: UserSearchFilterCriteria
    ) -> Result<i64, DomainError> {
        let total = self.user_manager.count(filter_criteria)?;
        Ok(total)
    }

    fn delete(&mut self, user_id: UserId) -> Result<(), DomainError> {
        self.user_manager.delete(user_id)?;
        Ok(())
    }

    fn exists(
        &mut self,
        filter_criteria: UserSearchFilterCriteria
    ) -> Result<bool, DomainError> {
        let exists = self.user_manager.exists(filter_criteria)?;
        Ok(exists)
    }

    fn find_user_sessions(
        &mut self,
        search_criteria: UserSessionSearchCriteria
    ) -> Result<Vec<UserSession>, DomainError> {
        let user_sessions = self.user_session_manager.find(search_criteria)?;
        Ok(user_sessions)
    }

    fn find_users(
        &mut self,
        search_criteria: UserSearchCriteria
    ) -> Result<Vec<User>, DomainError> {
        let users = self.user_manager.find(search_criteria)?;
        Ok(users)
    }

    fn get_updated_session(
        &mut self,
        user_session_id: UserSessionId
    ) -> Result<UserSession, DomainError> {
        let mut user_session = self.user_session_manager.get(user_session_id)?;
        user_session.update()?;
        self.user_session_manager.update(user_session.clone())?;
        Ok(user_session)
    }

    fn get_user(&mut self, user_id: UserId) -> Result<User, DomainError> {
        let user = self.user_manager.get(user_id)?;
        Ok(user)
    }

    fn login(
        &mut self,
        user_email: UserEmail,
        plaintext_password: UserPlaintextPassword,
        ip_value: String,
        user_agent_request_header: String
    ) -> Result<UserSession, DomainError> {
        let user = self.user_manager.get_by_email(user_email)?;
        self.user_manager.verify_password(user.id.clone(), plaintext_password)?;

        let user_session = UserSession::new(
            user.id,
            ip_value,
            user_agent_request_header
        )?;
        self.user_session_manager.add(user_session.clone())?;

        Ok(user_session)
    }

    fn logout(
        &mut self,
        user_session_id: UserSessionId
    ) -> Result<(), DomainError> {
        self.user_session_manager.delete(user_session_id)?;
        Ok(())
    }

    fn update(&mut self, user: User) -> Result<(), DomainError> {
        self.user_manager.update(user)?;
        Ok(())
    }
}
