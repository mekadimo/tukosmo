use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::core::shared::error;
use crate::core::shared::model::DomainError;
use crate::core::shared::model::PaginationCriteria;
use super::UserBrowser;
use super::UserId;
use super::UserPlatform;

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct UserSession {
    pub browser: UserBrowser,
    pub creation_date: UserSessionCreationDate,
    pub csrf_token: UserSessionCsrfToken,
    pub id: UserSessionId,
    pub ip: UserSessionIp,
    pub last_request_date: UserSessionLastRequestDate,
    pub platform: UserPlatform,
    pub user_agent_request_header: UserSessionUserAgentRequestHeader,
    pub user_id: UserId,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct UserSessionCreationDate(DateTime<Utc>);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UserSessionCsrfToken(Uuid);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UserSessionId(Uuid);

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct UserSessionIp(String);

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct UserSessionLastRequestDate(DateTime<Utc>);

#[derive(Clone)]
pub struct UserSessionSearchCriteria {
    pub filter: UserSessionSearchFilterCriteria,
    pub order_by: Option<UserSessionSearchCriteriaOrderBy>,
    pub pagination: Option<PaginationCriteria>,
}

#[derive(Clone)]
pub enum UserSessionSearchCriteriaOrderBy {
    CreationDateAsc,
    CreationDateDesc,
    LastRequestDateAsc,
    LastRequestDateDesc,
}

#[derive(Clone)]
pub struct UserSessionSearchFilterCriteria {
    pub id: Option<UserSessionId>,
    pub not_id: Option<UserSessionId>,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct UserSessionUserAgentRequestHeader(String);

impl UserSession {
    pub fn new(
        user_id: UserId,
        ip_value: String,
        user_agent_request_header: String
    ) -> Result<Self, DomainError> {
        let id = UserSessionId::new();
        let csrf_token = UserSessionCsrfToken::new();
        let ip = UserSessionIp::new(ip_value);
        let browser = UserBrowser::from_user_agent(&user_agent_request_header);
        let platform = UserPlatform::from_user_agent(
            &user_agent_request_header
        );
        let creation_date = UserSessionCreationDate::new();
        let last_request_date = UserSessionLastRequestDate::new();
        let user_agent_request_header = UserSessionUserAgentRequestHeader::new(
            user_agent_request_header
        );

        Ok(UserSession {
            browser,
            creation_date,
            csrf_token,
            id,
            ip,
            last_request_date,
            platform,
            user_agent_request_header,
            user_id,
        })
    }

    // NOTE: Not used.
    // TODO: Figure out when should be used; this is not trivial, since
    // auto-renewal could accidentally forbid valid requests (for example, the
    // user fills a form using a valid token, but before sending it, it is
    // replaced by another one, so the request will be rejected). This must be
    // addressed, since immutable CSRF tokens are risky.
    pub fn renew_csrf_token(&mut self) -> Result<(), DomainError> {
        self.csrf_token = UserSessionCsrfToken::new();
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), DomainError> {
        self.last_request_date = UserSessionLastRequestDate::new();
        Ok(())
    }
}

impl UserSessionCreationDate {
    pub fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    fn new() -> Self {
        let value = Utc::now();
        Self(value)
    }

    pub fn value(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl UserSessionCsrfToken {
    pub fn from(value: Uuid) -> Self {
        Self(value)
    }

    pub fn from_string(value: &str) -> Result<Self, DomainError> {
        match Uuid::parse_str(value) {
            Ok(value) => Ok(Self(value)),
            Err(_) => Err(error::INVALID_UUID),
        }
    }

    fn new() -> Self {
        let value = Uuid::new_v4();
        Self(value)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl UserSessionId {
    pub fn from(value: Uuid) -> Self {
        Self(value)
    }

    pub fn from_string(value: &str) -> Result<Self, DomainError> {
        match Uuid::parse_str(value) {
            Ok(value) => Ok(Self(value)),
            Err(_) => Err(error::INVALID_UUID),
        }
    }

    fn new() -> Self {
        let value = Uuid::new_v4();
        Self(value)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl UserSessionIp {
    pub fn from(value: String) -> Self {
        Self(value)
    }

    fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl UserSessionLastRequestDate {
    pub fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    fn new() -> Self {
        let value = Utc::now();
        Self(value)
    }

    pub fn value(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl UserSessionSearchCriteria {
    pub fn all() -> Self {
        let criteria = Self::default();
        criteria
    }

    pub fn all_ordered() -> Self {
        let mut criteria = Self::default();
        criteria.order_by = Some(
            UserSessionSearchCriteriaOrderBy::LastRequestDateDesc
        );
        criteria
    }

    fn default() -> Self {
        Self {
            filter: UserSessionSearchFilterCriteria {
                id: None,
                not_id: None,
            },
            order_by: None,
            pagination: None,
        }
    }

    pub fn has_id(user_session_id: UserSessionId) -> Self {
        let mut criteria = Self::default();
        criteria.filter.id = Some(user_session_id);
        criteria
    }
}

impl UserSessionUserAgentRequestHeader {
    pub fn from(value: String) -> Self {
        Self(value)
    }

    fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
