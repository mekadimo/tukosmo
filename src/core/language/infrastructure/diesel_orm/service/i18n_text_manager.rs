use diesel::pg::PgConnection;
use std::cell::RefCell;
use std::rc::Rc;
use tukosmo_domain::core::language::error;
use tukosmo_domain::core::language::model::I18nText;
use tukosmo_domain::core::language::model::I18nTextId;
use tukosmo_domain::core::language::model::I18nTextSearchCriteria;
use tukosmo_domain::core::language::model::I18nTranslationId;
use tukosmo_domain::core::language::model::I18nTranslationSearchCriteria;
use tukosmo_domain::core::shared::model::DomainError;

use crate::core::shared::diesel_orm::statement::ModelSqlExecutor;
use super::super::model::DbI18nText;
use super::super::model::DbI18nTranslation;
use super::super::statement::I18nTextSqlExecutor;
use super::super::statement::I18nTranslationSqlExecutor;

pub struct I18nTextManager {
    i18n_text_sql: I18nTextSqlExecutor,
    i18n_translation_sql: I18nTranslationSqlExecutor,
}

impl I18nTextManager {
    pub fn add(&mut self, i18n_text: I18nText) -> Result<(), DomainError> {
        let db_i18n_text = DbI18nText::from_domain(i18n_text.clone());
        let db_i18n_translations: Vec<DbI18nTranslation> =
            i18n_text.translations
                .into_iter()
                .map(|t| {
                    DbI18nTranslation::from_domain(
                        t,
                        I18nTextId::from_unvalidated(db_i18n_text.id.clone())
                    )
                })
                .collect();

        self.i18n_text_sql.insert(db_i18n_text)?;
        self.i18n_translation_sql.insert_in_bulk(db_i18n_translations)?;

        Ok(())
    }

    pub fn delete(
        &mut self,
        i18n_text_id: I18nTextId
    ) -> Result<(), DomainError> {
        self.i18n_text_sql.delete(
            I18nTextSearchCriteria::has_id(i18n_text_id).filter
        )?;

        Ok(())
    }

    pub fn get(
        &mut self,
        i18n_text_id: I18nTextId
    ) -> Result<I18nText, DomainError> {
        let db_i18n_texts = self.i18n_text_sql.select(
            I18nTextSearchCriteria::has_id(i18n_text_id.clone())
        )?;
        let db_i18n_text = db_i18n_texts
            .first()
            .cloned()
            .ok_or(error::i18n_text_not_found(&i18n_text_id))?;

        let db_i18n_translations = self.i18n_translation_sql.select(
            I18nTranslationSearchCriteria::has_i18n_text_id(i18n_text_id)
        )?;

        let i18n_text = db_i18n_text.to_domain(db_i18n_translations);
        Ok(i18n_text)
    }

    pub fn get_in_bulk(
        &mut self,
        i18n_text_ids: Vec<I18nTextId>
    ) -> Result<Vec<I18nText>, DomainError> {
        let db_i18n_texts = self.i18n_text_sql.select(
            I18nTextSearchCriteria::has_id_in(i18n_text_ids)
        )?;
        let i18n_text_ids = db_i18n_texts
            .iter()
            .map(|i| I18nTextId::from_unvalidated(i.id.clone()))
            .collect();

        let db_i18n_translations = self.i18n_translation_sql.select(
            I18nTranslationSearchCriteria::has_i18n_text_id_in(i18n_text_ids)
        )?;

        let mut i18n_texts: Vec<I18nText> = vec![];
        for db_i18n_text in db_i18n_texts {
            let db_translations: Vec<DbI18nTranslation> = db_i18n_translations
                .iter()
                .filter(|&t| t.clone().i18n_text_id == db_i18n_text.id.clone())
                .cloned()
                .collect();

            let i18n_text = db_i18n_text.to_domain(db_translations);
            i18n_texts.push(i18n_text);
        }

        Ok(i18n_texts)
    }

    pub fn init(connection: Rc<RefCell<PgConnection>>) -> Self {
        Self {
            i18n_text_sql: I18nTextSqlExecutor::init(Rc::clone(&connection)),
            i18n_translation_sql: I18nTranslationSqlExecutor::init(connection),
        }
    }

    pub fn update(&mut self, i18n_text: I18nText) -> Result<(), DomainError> {
        let db_i18n_text = DbI18nText::from_domain(i18n_text.clone());
        self.i18n_text_sql.update(&db_i18n_text)?;

        let db_i18n_translations: Vec<DbI18nTranslation> =
            i18n_text.translations
                .clone()
                .into_iter()
                .map(|t|
                    DbI18nTranslation::from_domain(
                        t.clone(),
                        i18n_text.id.clone()
                    )
                )
                .collect();
        self.i18n_translation_sql.upsert_in_bulk(db_i18n_translations)?;

        let i18n_translation_ids_to_retain: Vec<I18nTranslationId> =
            i18n_text.translations
                .into_iter()
                .map(|t| { t.id.clone() })
                .collect();
        self.i18n_translation_sql.delete(
            I18nTranslationSearchCriteria::has_i18n_text_id_and_id_not_in(
                i18n_text.id,
                i18n_translation_ids_to_retain
            ).filter
        )?;

        Ok(())
    }
}
