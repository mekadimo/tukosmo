use crate::core::shared::model::DomainError;
use super::super::model::Language;
use super::super::model::LanguageId;
use super::super::model::LanguageSearchCriteria;
use super::super::model::LanguageSearchFilterCriteria;

pub trait LanguageRepository {
    fn add(&mut self, language: Language) -> Result<(), DomainError>;

    fn count(
        &mut self,
        criteria: LanguageSearchFilterCriteria
    ) -> Result<i64, DomainError>;

    fn delete(&mut self, language_id: LanguageId) -> Result<(), DomainError>;

    fn exists(
        &mut self,
        criteria: LanguageSearchFilterCriteria
    ) -> Result<bool, DomainError>;

    fn find(
        &mut self,
        criteria: LanguageSearchCriteria
    ) -> Result<Vec<Language>, DomainError>;

    fn get(&mut self, language_id: LanguageId) -> Result<Language, DomainError>;

    fn update(&mut self, language: Language) -> Result<(), DomainError>;
}
