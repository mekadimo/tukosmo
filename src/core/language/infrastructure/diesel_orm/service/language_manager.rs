use diesel::pg::PgConnection;
use std::cell::RefCell;
use std::rc::Rc;
use tukosmo_domain::core::language::error;
use tukosmo_domain::core::language::model::I18nText;
use tukosmo_domain::core::language::model::I18nTextId;
use tukosmo_domain::core::language::model::I18nTranslation;
use tukosmo_domain::core::language::model::I18nTranslationText;
use tukosmo_domain::core::language::model::Language;
use tukosmo_domain::core::language::model::LanguageId;
use tukosmo_domain::core::language::model::LanguageSearchCriteria;
use tukosmo_domain::core::language::model::LanguageSearchFilterCriteria;
use tukosmo_domain::core::shared::model::DomainError;

use crate::core::shared::diesel_orm::statement::ModelSqlExecutor;
use super::I18nTextManager;
use super::super::model::DbI18nTranslation;
use super::super::model::DbLanguage;
use super::super::statement::I18nTranslationSqlExecutor;
use super::super::statement::LanguageSqlExecutor;

pub struct LanguageManager {
    i18n_text_manager: I18nTextManager,
    i18n_translation_sql: I18nTranslationSqlExecutor,
    language_sql: LanguageSqlExecutor,
}

impl LanguageManager {
    pub fn add(&mut self, language: Language) -> Result<(), DomainError> {
        self.i18n_text_manager.add(language.name.clone())?;

        let db_language = DbLanguage::from_domain(language.clone());
        self.language_sql.insert(db_language)?;

        let original_name_translation = I18nTranslation::new(
            language.id,
            I18nTranslationText::new(language.original_name.value().to_string())
        );
        let db_original_name_translation = DbI18nTranslation::from_domain(
            original_name_translation,
            language.name.id
        );
        self.i18n_translation_sql.insert(db_original_name_translation)?;

        Ok(())
    }

    pub fn count(
        &mut self,
        filter_criteria: LanguageSearchFilterCriteria
    ) -> Result<i64, DomainError> {
        let total = self.language_sql.select_count(filter_criteria)?;
        Ok(total)
    }

    pub fn delete(
        &mut self,
        language_id: LanguageId
    ) -> Result<(), DomainError> {
        let total_languages = self.language_sql.select_count(
            LanguageSearchCriteria::all().filter
        )?;
        if total_languages == 1 {
            return Err(error::CANNOT_DELETE_LAST_LANGUAGE_LEFT);
        }

        let language = self.get(language_id)?;
        self.language_sql.delete(
            LanguageSearchCriteria::has_id(language.id).filter
        )?;
        self.i18n_text_manager.delete(language.name.id)?;

        Ok(())
    }

    pub fn exists(
        &mut self,
        filter_criteria: LanguageSearchFilterCriteria
    ) -> Result<bool, DomainError> {
        let exists = self.language_sql.select_exists(filter_criteria)?;
        Ok(exists)
    }

    pub fn find(
        &mut self,
        search_criteria: LanguageSearchCriteria
    ) -> Result<Vec<Language>, DomainError> {
        let db_languages = self.language_sql.select(search_criteria)?;
        let language_name_ids = db_languages
            .iter()
            .map(|l| I18nTextId::from_unvalidated(l.i18n_text_id_name.clone()))
            .collect();

        let language_names =
            self.i18n_text_manager.get_in_bulk(language_name_ids)?;

        let mut languages: Vec<Language> = vec![];
        for db_language in db_languages {
            let language_name_id_value = db_language.i18n_text_id_name;
            let language_name: I18nText = language_names
                .iter()
                .find(|n| n.id.value() == &language_name_id_value)
                .unwrap()
                .clone();

            let language = db_language.to_domain(language_name);
            languages.push(language);
        }

        Ok(languages)
    }

    pub fn get(
        &mut self,
        language_id: LanguageId
    ) -> Result<Language, DomainError> {
        let db_languages = self.language_sql.select(
            LanguageSearchCriteria::has_id(language_id)
        )?;
        let db_language = db_languages
            .first()
            .cloned()
            .ok_or(error::LANGUAGE_NOT_FOUND)?;

        let language_name = self.i18n_text_manager.get(
            I18nTextId::from_unvalidated(db_language.i18n_text_id_name.clone())
        )?;

        let language = db_language.to_domain(language_name);

        Ok(language)
    }

    pub fn init(connection: Rc<RefCell<PgConnection>>) -> Self {
        Self {
            i18n_text_manager: I18nTextManager::init(Rc::clone(&connection)),
            i18n_translation_sql: I18nTranslationSqlExecutor::init(
                Rc::clone(&connection)
            ),
            language_sql: LanguageSqlExecutor::init(connection),
        }
    }

    pub fn update(&mut self, language: Language) -> Result<(), DomainError> {
        let db_language = DbLanguage::from_domain(language.clone());
        self.language_sql.update(&db_language)?;

        self.i18n_text_manager.update(language.name)?;

        Ok(())
    }
}
