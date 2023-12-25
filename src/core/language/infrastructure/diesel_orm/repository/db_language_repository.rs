use diesel::pg::PgConnection;
use std::cell::RefCell;
use std::rc::Rc;
use tukosmo_domain::core::language::model::Language;
use tukosmo_domain::core::language::model::LanguageId;
use tukosmo_domain::core::language::model::LanguageSearchCriteria;
use tukosmo_domain::core::language::model::LanguageSearchFilterCriteria;
use tukosmo_domain::core::language::repository::LanguageRepository;
use tukosmo_domain::core::shared::model::DomainError;

use super::super::service::LanguageManager;

pub struct DbLanguageRepository {
    language_manager: LanguageManager,
}

impl DbLanguageRepository {
    pub fn init(connection: Rc<RefCell<PgConnection>>) -> Self {
        Self { language_manager: LanguageManager::init(connection) }
    }
}

impl LanguageRepository for DbLanguageRepository {
    fn add(&mut self, language: Language) -> Result<(), DomainError> {
        self.language_manager.add(language)?;
        Ok(())
    }

    fn count(
        &mut self,
        filter_criteria: LanguageSearchFilterCriteria
    ) -> Result<i64, DomainError> {
        let total = self.language_manager.count(filter_criteria)?;
        Ok(total)
    }

    fn delete(&mut self, language_id: LanguageId) -> Result<(), DomainError> {
        self.language_manager.delete(language_id)?;
        Ok(())
    }

    fn exists(
        &mut self,
        filter_criteria: LanguageSearchFilterCriteria
    ) -> Result<bool, DomainError> {
        let exists = self.language_manager.exists(filter_criteria)?;
        Ok(exists)
    }

    fn find(
        &mut self,
        search_criteria: LanguageSearchCriteria
    ) -> Result<Vec<Language>, DomainError> {
        let languages = self.language_manager.find(search_criteria)?;
        Ok(languages)
    }

    fn get(
        &mut self,
        language_id: LanguageId
    ) -> Result<Language, DomainError> {
        let language = self.language_manager.get(language_id)?;
        Ok(language)
    }

    fn update(&mut self, language: Language) -> Result<(), DomainError> {
        self.language_manager.update(language)?;
        Ok(())
    }
}
