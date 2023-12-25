use std::cell::RefCell;
use std::rc::Rc;
use tukosmo_domain::core::user::error;
use tukosmo_domain::core::user::model::User;
use tukosmo_domain::core::user::model::UserEmail;
use tukosmo_domain::core::user::model::UserId;
use tukosmo_domain::core::user::model::UserIsAdmin;
use tukosmo_domain::core::user::model::UserIsSuspended;
use tukosmo_domain::core::user::model::UserPlaintextPassword;
use tukosmo_domain::core::user::model::UserSearchCriteria;
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::shared::model::TransactionExecutor;

use crate::core::shared::service::run_transaction;
use super::super::dto::DtoAddUser;
use super::super::dto::DtoChangeUserPassword;
use super::super::dto::DtoEditUser;
use super::super::dto::DtoGetUser;
use super::super::dto::DtoGetUsersPaginated;
use super::super::dto::DtoUsersPaginated;

pub struct UserUseCase {
    transaction_executor: Rc<RefCell<dyn TransactionExecutor>>,
}

impl UserUseCase {
    pub fn add(&self, dto: DtoAddUser) -> Result<(), DomainError> {
        run_transaction(self.transaction_executor.borrow_mut(), |transaction| {
            let mut user_repository = transaction.user_repository.borrow_mut();

            let user_email = UserEmail::new(dto.form.email.clone())?;
            let user_email_already_exists = user_repository.exists(
                UserSearchCriteria::has_email(user_email.clone()).filter
            )?;
            if user_email_already_exists {
                return Err(error::USER_EMAIL_ALREADY_EXISTS);
            }

            if
                &dto.form.plaintext_password !=
                &dto.form.plaintext_password_repeated
            {
                return Err(error::USER_REPEATED_PASSWORD_DOES_NOT_MATCH);
            }

            let user = User::new(
                user_email,
                dto.form.name,
                UserIsAdmin::new(dto.form.is_admin),
                UserIsSuspended::new(dto.form.is_admin)
            )?;
            let plaintext_password = UserPlaintextPassword::new(
                dto.form.plaintext_password
            )?;

            user_repository.add(user, plaintext_password)?;
            Ok(())
        })
    }

    pub fn change_password(
        &self,
        dto: DtoChangeUserPassword
    ) -> Result<(), DomainError> {
        run_transaction(self.transaction_executor.borrow_mut(), |transaction| {
            let user_id = UserId::from_string(&dto.user_id)?;

            if
                &dto.form.plaintext_new_password !=
                &dto.form.plaintext_new_password_repeated
            {
                return Err(error::USER_REPEATED_PASSWORD_DOES_NOT_MATCH);
            }

            let plaintext_new_password = UserPlaintextPassword::new(
                dto.form.plaintext_new_password
            )?;
            let plaintext_old_password = UserPlaintextPassword::new(
                dto.form.plaintext_old_password
            )?;

            let mut user_repository = transaction.user_repository.borrow_mut();
            user_repository.change_password(
                user_id,
                plaintext_new_password,
                plaintext_old_password
            )?;

            Ok(())
        })
    }

    pub fn edit(&self, dto: DtoEditUser) -> Result<(), DomainError> {
        run_transaction(self.transaction_executor.borrow_mut(), |transaction| {
            let user_id = UserId::from_string(&dto.user_id)?;

            let mut user_repository = transaction.user_repository.borrow_mut();

            let user_email = UserEmail::new(dto.form.email.clone())?;
            let other_user_has_same_email = user_repository.exists(
                UserSearchCriteria::has_email_and_not_id(
                    user_email.clone(),
                    user_id.clone()
                ).filter
            )?;
            if other_user_has_same_email {
                return Err(error::USER_EMAIL_ALREADY_EXISTS);
            }

            let mut user = user_repository.get_user(user_id)?;
            user.modify(
                user_email,
                dto.form.name,
                UserIsAdmin::new(dto.form.is_admin),
                UserIsSuspended::new(dto.form.is_suspended)
            )?;

            user_repository.update(user)?;

            Ok(())
        })
    }

    pub fn get(&self, dto: DtoGetUser) -> Result<User, DomainError> {
        run_transaction(self.transaction_executor.borrow_mut(), |transaction| {
            let user_id = UserId::from_string(&dto.user_id)?;

            let mut user_repository = transaction.user_repository.borrow_mut();
            let user = user_repository.get_user(user_id)?;

            Ok(user)
        })
    }

    pub fn get_users_paginated(
        &self,
        dto: DtoGetUsersPaginated
    ) -> Result<DtoUsersPaginated, DomainError> {
        run_transaction(self.transaction_executor.borrow_mut(), |transaction| {
            let mut user_repository = transaction.user_repository.borrow_mut();

            let users = user_repository.find_users(
                UserSearchCriteria::paginated(
                    dto.current_page,
                    dto.results_per_page
                )
            )?;
            let total_results = user_repository.count(
                UserSearchCriteria::all().filter
            )?;

            Ok(DtoUsersPaginated { users, total_results })
        })
    }

    pub fn init(
        transaction_executor: Rc<RefCell<dyn TransactionExecutor>>
    ) -> Self {
        Self { transaction_executor }
    }
}
