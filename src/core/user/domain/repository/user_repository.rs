use crate::core::shared::model::DomainError;
use super::super::model::User;
use super::super::model::UserEmail;
use super::super::model::UserId;
use super::super::model::UserPlaintextPassword;
use super::super::model::UserSearchCriteria;
use super::super::model::UserSearchFilterCriteria;
use super::super::model::UserSession;
use super::super::model::UserSessionId;
use super::super::model::UserSessionSearchCriteria;

pub trait UserRepository {
    fn add(
        &mut self,
        user: User,
        plaintext_password: UserPlaintextPassword
    ) -> Result<(), DomainError>;

    fn change_password(
        &mut self,
        user_id: UserId,
        plaintext_old_password: UserPlaintextPassword,
        plaintext_password: UserPlaintextPassword
    ) -> Result<(), DomainError>;

    fn count(
        &mut self,
        criteria: UserSearchFilterCriteria
    ) -> Result<i64, DomainError>;

    fn delete(&mut self, user_id: UserId) -> Result<(), DomainError>;

    fn exists(
        &mut self,
        criteria: UserSearchFilterCriteria
    ) -> Result<bool, DomainError>;

    fn find_user_sessions(
        &mut self,
        criteria: UserSessionSearchCriteria
    ) -> Result<Vec<UserSession>, DomainError>;

    fn find_users(
        &mut self,
        criteria: UserSearchCriteria
    ) -> Result<Vec<User>, DomainError>;

    fn get_updated_session(
        &mut self,
        session_id: UserSessionId
    ) -> Result<UserSession, DomainError>;

    fn get_user(&mut self, user_id: UserId) -> Result<User, DomainError>;

    fn login(
        &mut self,
        user_email: UserEmail,
        plaintext_password: UserPlaintextPassword,
        ip_value: String,
        user_agent_request_header: String
    ) -> Result<UserSession, DomainError>;

    fn logout(
        &mut self,
        user_session_id: UserSessionId
    ) -> Result<(), DomainError>;

    fn update(&mut self, user: User) -> Result<(), DomainError>;
}
