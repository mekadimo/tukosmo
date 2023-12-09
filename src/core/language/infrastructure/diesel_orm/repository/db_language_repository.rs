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
    language: LanguageManager,
}

impl DbLanguageRepository {
    pub fn init(connection: Rc<RefCell<PgConnection>>) -> Self {
        Self { language: LanguageManager::init(connection) }
    }
}

impl LanguageRepository for DbLanguageRepository {
    fn add(&mut self, language: Language) -> Result<(), DomainError> {
        self.language.add(language)?;
        Ok(())
    }

    fn count(
        &mut self,
        filter_criteria: LanguageSearchFilterCriteria
    ) -> Result<i64, DomainError> {
        let total = self.language.count(filter_criteria)?;
        Ok(total)
    }

    fn delete(&mut self, language_id: LanguageId) -> Result<(), DomainError> {
        self.language.delete(language_id)?;
        Ok(())
    }

    fn exists(
        &mut self,
        filter_criteria: LanguageSearchFilterCriteria
    ) -> Result<bool, DomainError> {
        let exists = self.language.exists(filter_criteria)?;
        Ok(exists)
    }

    fn find(
        &mut self,
        search_criteria: LanguageSearchCriteria
    ) -> Result<Vec<Language>, DomainError> {
        let languages = self.language.find(search_criteria)?;
        Ok(languages)
    }

    fn get(
        &mut self,
        language_id: LanguageId
    ) -> Result<Language, DomainError> {
        let language = self.language.get(language_id)?;
        Ok(language)
    }

    fn update(&mut self, language: Language) -> Result<(), DomainError> {
        self.language.update(language)?;
        Ok(())
    }
}
