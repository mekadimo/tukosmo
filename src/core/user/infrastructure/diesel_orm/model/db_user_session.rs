use chrono::DateTime;
use chrono::Utc;
use diesel::pg::Pg;
use diesel::prelude::AsChangeset;
use diesel::prelude::Associations;
use diesel::prelude::Identifiable;
use diesel::prelude::Insertable;
use diesel::prelude::Queryable;
use diesel::prelude::Selectable;
use tukosmo_domain::core::user::model::UserBrowser;
use tukosmo_domain::core::user::model::UserId;
use tukosmo_domain::core::user::model::UserPlatform;
use tukosmo_domain::core::user::model::UserSession;
use tukosmo_domain::core::user::model::UserSessionCreationDate;
use tukosmo_domain::core::user::model::UserSessionCsrfToken;
use tukosmo_domain::core::user::model::UserSessionId;
use tukosmo_domain::core::user::model::UserSessionIp;
use tukosmo_domain::core::user::model::UserSessionLastRequestDate;
use tukosmo_domain::core::user::model::UserSessionUserAgentRequestHeader;
use uuid::Uuid;

use crate::core::shared::diesel_orm::schema::user_session;
use crate::core::user::diesel_orm::model::DbUser;

#[derive(
    AsChangeset,
    Associations,
    Clone,
    Debug,
    Identifiable,
    Insertable,
    PartialEq,
    Queryable,
    Selectable
)]
#[diesel(belongs_to(DbUser, foreign_key = user_id))]
#[diesel(table_name = user_session)]
#[diesel(check_for_backend(Pg))]
pub struct DbUserSession {
    pub creation_date: DateTime<Utc>,
    pub csrf_token: Uuid,
    pub id: Uuid,
    pub ip: String,
    pub last_request_date: DateTime<Utc>,
    pub user_agent_request_header: String,
    pub user_id: Uuid,
}

impl DbUserSession {
    pub fn from_domain(user_session: UserSession) -> Self {
        Self {
            creation_date: user_session.creation_date.value().clone(),
            csrf_token: user_session.csrf_token.value().clone(),
            id: user_session.id.value().clone(),
            ip: user_session.ip.value().to_string(),
            last_request_date: user_session.last_request_date.value().clone(),
            user_agent_request_header: user_session.user_agent_request_header
                .value()
                .to_string(),
            user_id: user_session.user_id.value().clone(),
        }
    }

    pub fn to_domain(&self) -> UserSession {
        UserSession {
            browser: UserBrowser::from_user_agent(
                &self.user_agent_request_header
            ),
            creation_date: UserSessionCreationDate::from(
                self.creation_date.clone()
            ),
            csrf_token: UserSessionCsrfToken::from(self.csrf_token.clone()),
            id: UserSessionId::from(self.id.clone()),
            ip: UserSessionIp::from(self.ip.clone()),
            last_request_date: UserSessionLastRequestDate::from(
                self.last_request_date.clone()
            ),
            platform: UserPlatform::from_user_agent(
                &self.user_agent_request_header
            ),
            user_agent_request_header: UserSessionUserAgentRequestHeader::from(
                self.user_agent_request_header.clone()
            ),
            user_id: UserId::from(self.user_id.clone()),
        }
    }
}
