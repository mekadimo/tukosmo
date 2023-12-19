use std::cell::RefCell;
use std::rc::Rc;
use tukosmo_domain::core::language::model::LanguageSearchCriteria;
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::shared::model::LocalI18n;
use tukosmo_domain::core::shared::model::TransactionExecutor;
use tukosmo_domain::core::shared::repository::DataRepository;

use crate::core::shared::dto::DtoGetInitialData;
use crate::core::shared::dto::DtoGetLocalI18n;
use crate::core::shared::service::run_transaction;
use super::super::dto::DtoInitialData;

pub struct GlobalUseCase {
    data_repository: Rc<RefCell<dyn DataRepository>>,
    transaction_executor: Rc<RefCell<dyn TransactionExecutor>>,
}

impl GlobalUseCase {
    pub fn get_initial_data(
        &self,
        dto: DtoGetInitialData
    ) -> Result<DtoInitialData, DomainError> {
        run_transaction(self.transaction_executor.borrow_mut(), |transaction| {
            let data_repository = self.data_repository.borrow_mut();
            let mut language_repository =
                transaction.language_repository.borrow_mut();

            let language_code = match dto.language_code {
                Some(language_code) => language_code,
                // TODO: Take this value from the default config in the database
                None => "en".to_string(),
            };
            let local_i18n = data_repository.get_local_i18n(&language_code)?;
            let languages = language_repository.find(
                LanguageSearchCriteria::all_ordered()
            )?;

            Ok(DtoInitialData {
                language_code,
                languages,
                local_i18n,
            })
        })
    }

    pub fn get_local_i18n(
        &self,
        dto: DtoGetLocalI18n
    ) -> Result<LocalI18n, DomainError> {
        let data_repository = self.data_repository.borrow_mut();
        let local_i18n = data_repository.get_local_i18n(&dto.language_code);
        local_i18n
    }

    pub fn init(
        data_repository: Rc<RefCell<dyn DataRepository>>,
        transaction_executor: Rc<RefCell<dyn TransactionExecutor>>
    ) -> Self {
        Self { data_repository, transaction_executor }
    }
}
