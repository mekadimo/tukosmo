use diesel::pg::PgConnection;
use std::cell::RefCell;
use std::rc::Rc;
use tukosmo_domain::core::language::model::I18nText;
use tukosmo_domain::core::language::model::I18nTextId;
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::user::error;
use tukosmo_domain::core::user::model::User;
use tukosmo_domain::core::user::model::UserEmail;
use tukosmo_domain::core::user::model::UserId;
use tukosmo_domain::core::user::model::UserPlaintextPassword;
use tukosmo_domain::core::user::model::UserSearchCriteria;
use tukosmo_domain::core::user::model::UserSearchFilterCriteria;
use tukosmo_domain::core::user::model::UserEncryptedPassword;

use crate::core::language::diesel_orm::service::I18nTextManager;
use crate::core::shared::diesel_orm::statement::ModelSqlExecutor;
use super::super::model::DbUser;
use super::super::statement::UserSqlExecutor;

pub struct UserManager {
    i18n_text_manager: I18nTextManager,
    user_sql: UserSqlExecutor,
}

impl UserManager {
    pub fn add(
        &mut self,
        user: User,
        plaintext_password: UserPlaintextPassword
    ) -> Result<(), DomainError> {
        self.i18n_text_manager.add(user.name.clone())?;

        let encrypted_password =
            UserEncryptedPassword::new(plaintext_password)?;
        let db_user = DbUser::from_domain(user.clone(), encrypted_password);
        self.user_sql.insert(db_user)?;

        Ok(())
    }

    pub fn change_password(
        &mut self,
        user_id: UserId,
        plaintext_old_password: UserPlaintextPassword,
        plaintext_password: UserPlaintextPassword
    ) -> Result<(), DomainError> {
        self.verify_password(user_id.clone(), plaintext_old_password)?;

        let user = self.get(user_id)?;
        let encrypted_password =
            UserEncryptedPassword::new(plaintext_password)?;
        let db_user = DbUser::from_domain(user.clone(), encrypted_password);
        self.user_sql.update(&db_user)?;

        Ok(())
    }

    pub fn count(
        &mut self,
        filter_criteria: UserSearchFilterCriteria
    ) -> Result<i64, DomainError> {
        let total = self.user_sql.select_count(filter_criteria)?;
        Ok(total)
    }

    pub fn exists(
        &mut self,
        filter_criteria: UserSearchFilterCriteria
    ) -> Result<bool, DomainError> {
        let exists = self.user_sql.select_exists(filter_criteria)?;
        Ok(exists)
    }

    pub fn find(
        &mut self,
        search_criteria: UserSearchCriteria
    ) -> Result<Vec<User>, DomainError> {
        let db_users = self.user_sql.select(search_criteria)?;
        let user_name_ids = db_users
            .iter()
            .map(|l| I18nTextId::from_unvalidated(l.i18n_text_id_name.clone()))
            .collect();

        let user_names = self.i18n_text_manager.get_in_bulk(user_name_ids)?;

        let mut users: Vec<User> = vec![];
        for db_user in db_users {
            let user_name_id_value = db_user.i18n_text_id_name;
            let user_name: I18nText = user_names
                .iter()
                .find(|n| n.id.value() == &user_name_id_value)
                .unwrap()
                .clone();

            let user = db_user.to_domain(user_name);
            users.push(user);
        }

        Ok(users)
    }

    pub fn get(&mut self, user_id: UserId) -> Result<User, DomainError> {
        let db_users = self.user_sql.select(
            UserSearchCriteria::has_id(user_id)
        )?;
        let db_user = db_users.first().cloned().ok_or(error::USER_NOT_FOUND)?;

        let user_name = self.i18n_text_manager.get(
            I18nTextId::from_unvalidated(db_user.i18n_text_id_name.clone())
        )?;

        let user = db_user.to_domain(user_name);

        Ok(user)
    }

    pub fn get_by_email(
        &mut self,
        user_email: UserEmail
    ) -> Result<User, DomainError> {
        let db_users = self.user_sql.select(
            UserSearchCriteria::has_email(user_email)
        )?;
        let db_user = db_users.first().cloned().ok_or(error::USER_NOT_FOUND)?;

        let user_name = self.i18n_text_manager.get(
            I18nTextId::from_unvalidated(db_user.i18n_text_id_name.clone())
        )?;

        let user = db_user.to_domain(user_name);

        Ok(user)
    }

    pub fn init(connection: Rc<RefCell<PgConnection>>) -> Self {
        Self {
            i18n_text_manager: I18nTextManager::init(Rc::clone(&connection)),
            user_sql: UserSqlExecutor::init(connection),
        }
    }

    // TODO: Check an admin cannot suspend its own user account or disable admin
    // (add UserSession to update arguments, and validate that if UserId ==)
    pub fn update(&mut self, user: User) -> Result<(), DomainError> {
        let db_previous_users = self.user_sql.select(
            UserSearchCriteria::has_id(user.id.clone())
        )?;
        let db_previous_user = db_previous_users
            .first()
            .cloned()
            .ok_or(error::USER_NOT_FOUND)?;

        let db_user = DbUser::from_domain(
            user.clone(),
            UserEncryptedPassword::from_unvalidated(
                db_previous_user.encrypted_password
            )
        );
        self.user_sql.update(&db_user)?;

        self.i18n_text_manager.update(user.name)?;

        Ok(())
    }

    pub fn verify_password(
        &mut self,
        user_id: UserId,
        plaintext_password: UserPlaintextPassword
    ) -> Result<(), DomainError> {
        let db_users = self.user_sql.select(
            UserSearchCriteria::has_id(user_id)
        )?;
        let db_user = db_users.first().cloned().ok_or(error::USER_NOT_FOUND)?;

        let encrypted_password = db_user.get_encrypted_password();
        if !encrypted_password.verify(plaintext_password) {
            return Err(error::USER_PASSWORD_NOT_CORRECT);
        }

        Ok(())
    }
}
