use chrono::DateTime;
use chrono::Utc;
use diesel::pg::Pg;
use diesel::prelude::AsChangeset;
use diesel::prelude::Associations;
use diesel::prelude::Identifiable;
use diesel::prelude::Insertable;
use diesel::prelude::Queryable;
use diesel::prelude::Selectable;
use tukosmo_domain::core::language::model::I18nText;
use tukosmo_domain::core::user::model::User;
use tukosmo_domain::core::user::model::UserCreationDate;
use tukosmo_domain::core::user::model::UserEmail;
use tukosmo_domain::core::user::model::UserEncryptedPassword;
use tukosmo_domain::core::user::model::UserId;
use tukosmo_domain::core::user::model::UserIsAdmin;
use tukosmo_domain::core::user::model::UserIsSuspended;
use tukosmo_domain::core::user::model::UserUpdateDate;
use uuid::Uuid;

use crate::core::language::diesel_orm::model::DbI18nText;
use crate::core::shared::diesel_orm::schema::user_;

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
#[diesel(belongs_to(DbI18nText, foreign_key = i18n_text_id_name))]
#[diesel(table_name = user_)]
#[diesel(check_for_backend(Pg))]
pub struct DbUser {
    pub creation_date: DateTime<Utc>,
    pub email: String,
    pub encrypted_password: String,
    pub i18n_text_id_name: Uuid,
    pub id: Uuid,
    pub is_admin: bool,
    pub is_suspended: bool,
    pub update_date: DateTime<Utc>,
}

impl DbUser {
    pub fn from_domain(
        user: User,
        encrypted_password: UserEncryptedPassword
    ) -> Self {
        Self {
            creation_date: user.creation_date.value().clone(),
            email: user.email.value().to_string(),
            encrypted_password: encrypted_password
                .encrypted_value()
                .to_string(),
            i18n_text_id_name: user.name.id.value().clone(),
            id: user.id.value().clone(),
            is_admin: user.is_admin.value().clone(),
            is_suspended: user.is_suspended.value().clone(),
            update_date: user.update_date.value().clone(),
        }
    }

    pub fn get_encrypted_password(self) -> UserEncryptedPassword {
        UserEncryptedPassword::from_unvalidated(self.encrypted_password.clone())
    }

    pub fn to_domain(self, user_name: I18nText) -> User {
        User {
            creation_date: UserCreationDate::from_unvalidated(
                self.creation_date.clone()
            ),
            email: UserEmail::from_unvalidated(self.email.clone()),
            id: UserId::from_unvalidated(self.id.clone()),
            is_admin: UserIsAdmin::from_unvalidated(self.is_admin.clone()),
            is_suspended: UserIsSuspended::from_unvalidated(
                self.is_suspended.clone()
            ),
            name: user_name,
            update_date: UserUpdateDate::from_unvalidated(
                self.update_date.clone()
            ),
        }
    }
}
