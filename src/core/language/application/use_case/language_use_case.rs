use std::cell::RefCell;
use std::rc::Rc;
use tukosmo_domain::core::language::error;
use tukosmo_domain::core::language::model::Language;
use tukosmo_domain::core::language::model::LanguageCode;
use tukosmo_domain::core::language::model::LanguageId;
use tukosmo_domain::core::language::model::LanguageSearchCriteria;
use tukosmo_domain::core::shared::error as error_shared;
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::shared::model::TransactionExecutor;

use crate::core::shared::service::run_transaction;
use super::super::dto::DtoAddLanguage;
use super::super::dto::DtoDeleteLanguage;
use super::super::dto::DtoEditLanguage;
use super::super::dto::DtoGetLanguage;
use super::super::dto::DtoGetLanguagesPaginated;
use super::super::dto::DtoLanguagesPaginated;

pub struct LanguageUseCase {
    transaction_executor: Rc<RefCell<dyn TransactionExecutor>>,
}

impl LanguageUseCase {
    pub fn add(&self, dto: DtoAddLanguage) -> Result<(), DomainError> {
        run_transaction(self.transaction_executor.borrow_mut(), |transaction| {
            let mut language_repository =
                transaction.language_repository.borrow_mut();

            let language_code = LanguageCode::from(dto.form.code.clone());
            let language_code_already_exists = language_repository.exists(
                LanguageSearchCriteria::has_code(language_code).filter
            )?;
            if language_code_already_exists {
                return Err(error::LANGUAGE_CODE_ALREADY_EXISTS);
            }

            let language = Language::new(
                dto.form.code,
                dto.form.name,
                dto.form.original_name,
                dto.form.website_title,
                dto.form.website_subtitle
            )?;

            language_repository.add(language)?;
            Ok(())
        })
    }

    pub fn delete(&self, dto: DtoDeleteLanguage) -> Result<(), DomainError> {
        run_transaction(self.transaction_executor.borrow_mut(), |transaction| {
            let language_id = LanguageId::from_string(&dto.language_id)?;

            let mut language_repository =
                transaction.language_repository.borrow_mut();

            if !dto.form.requested {
                return Err(error_shared::FIELD_CANNOT_BE_EMPTY);
            }
            language_repository.delete(language_id)?;

            Ok(())
        })
    }

    pub fn edit(&self, dto: DtoEditLanguage) -> Result<(), DomainError> {
        run_transaction(self.transaction_executor.borrow_mut(), |transaction| {
            let language_id = LanguageId::from_string(&dto.language_id)?;

            let mut language_repository =
                transaction.language_repository.borrow_mut();

            let language_code = LanguageCode::from(dto.form.code.clone());
            let other_language_has_same_code = language_repository.exists(
                LanguageSearchCriteria::has_code_and_not_id(
                    language_code,
                    language_id.clone()
                ).filter
            )?;
            if other_language_has_same_code {
                return Err(error::LANGUAGE_CODE_ALREADY_EXISTS);
            }

            let mut language = language_repository.get(language_id)?;
            language.modify(
                dto.form.code,
                dto.form.name,
                dto.form.original_name,
                dto.form.website_title,
                dto.form.website_subtitle
            )?;

            language_repository.update(language)?;

            Ok(())
        })
    }

    pub fn get(&self, dto: DtoGetLanguage) -> Result<Language, DomainError> {
        run_transaction(self.transaction_executor.borrow_mut(), |transaction| {
            let language_id = LanguageId::from_string(&dto.language_id)?;

            let mut language_repository =
                transaction.language_repository.borrow_mut();
            let language = language_repository.get(language_id)?;

            Ok(language)
        })
    }

    pub fn get_all_languages(&self) -> Result<Vec<Language>, DomainError> {
        run_transaction(self.transaction_executor.borrow_mut(), |transaction| {
            let mut language_repository =
                transaction.language_repository.borrow_mut();

            let languages = language_repository.find(
                LanguageSearchCriteria::all_ordered()
            )?;

            Ok(languages)
        })
    }

    pub fn get_languages_paginated(
        &self,
        dto: DtoGetLanguagesPaginated
    ) -> Result<DtoLanguagesPaginated, DomainError> {
        run_transaction(self.transaction_executor.borrow_mut(), |transaction| {
            let mut language_repository =
                transaction.language_repository.borrow_mut();

            let languages = language_repository.find(
                LanguageSearchCriteria::paginated(
                    dto.current_page,
                    dto.results_per_page
                )
            )?;
            let total_results = language_repository.count(
                LanguageSearchCriteria::all().filter
            )?;

            Ok(DtoLanguagesPaginated { languages, total_results })
        })
    }

    pub fn init(
        transaction_executor: Rc<RefCell<dyn TransactionExecutor>>
    ) -> Self {
        Self { transaction_executor }
    }
}
