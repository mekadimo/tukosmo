use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::core::shared::error;
use crate::core::shared::model::DomainError;
use super::I18nTextId;
use super::LanguageId;

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct I18nTranslation {
    pub creation_date: I18nTranslationCreationDate,
    pub id: I18nTranslationId,
    pub language_id: LanguageId,
    pub text: I18nTranslationText,
    pub update_date: I18nTranslationUpdateDate,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct I18nTranslationCreationDate(DateTime<Utc>);

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct I18nTranslationId(Uuid);

#[derive(Clone)]
pub struct I18nTranslationSearchCriteria {
    pub filter: I18nTranslationSearchFilterCriteria,
}

#[derive(Clone)]
pub struct I18nTranslationSearchFilterCriteria {
    pub i18n_text_id: Option<I18nTextId>,
    pub i18n_text_id_in: Option<Vec<I18nTextId>>,
    pub id: Option<I18nTranslationId>,
    pub id_in: Option<Vec<I18nTranslationId>>,
    pub id_not_in: Option<Vec<I18nTranslationId>>,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct I18nTranslationText(String);

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct I18nTranslationUpdateDate(DateTime<Utc>);

impl I18nTranslation {
    pub fn modify(&mut self, text_value: I18nTranslationText) {
        self.text = text_value;
        self.update_date = I18nTranslationUpdateDate::new();
    }

    pub fn new(language_id: LanguageId, text: I18nTranslationText) -> Self {
        let id = I18nTranslationId::new();
        let creation_date = I18nTranslationCreationDate::new();
        let update_date = I18nTranslationUpdateDate::new();
        I18nTranslation {
            creation_date,
            id,
            language_id,
            text,
            update_date,
        }
    }
}

impl I18nTranslationCreationDate {
    pub fn from_unvalidated(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    fn new() -> Self {
        let value = Utc::now();
        Self(value)
    }

    pub fn value(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl I18nTranslationId {
    pub fn from_unvalidated(value: Uuid) -> Self {
        Self(value)
    }

    pub fn from_string(value: &str) -> Result<Self, DomainError> {
        match Uuid::parse_str(value) {
            Ok(value) => Ok(Self(value)),
            Err(_) => Err(error::INVALID_UUID),
        }
    }

    fn new() -> Self {
        let value = Uuid::new_v4();
        Self(value)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl I18nTranslationSearchCriteria {
    fn default() -> Self {
        Self {
            filter: I18nTranslationSearchFilterCriteria {
                i18n_text_id: None,
                i18n_text_id_in: None,
                id: None,
                id_in: None,
                id_not_in: None,
            },
        }
    }

    pub fn has_i18n_text_id(i18n_text_id: I18nTextId) -> Self {
        let mut criteria = Self::default();
        criteria.filter.i18n_text_id = Some(i18n_text_id);
        criteria
    }

    pub fn has_i18n_text_id_in(i18n_text_ids: Vec<I18nTextId>) -> Self {
        let mut criteria = Self::default();
        criteria.filter.i18n_text_id_in = Some(i18n_text_ids);
        criteria
    }

    pub fn has_id(i18n_translation_id: I18nTranslationId) -> Self {
        let mut criteria = Self::default();
        criteria.filter.id = Some(i18n_translation_id);
        criteria
    }

    pub fn has_id_in(i18n_translation_ids: Vec<I18nTranslationId>) -> Self {
        let mut criteria = Self::default();
        criteria.filter.id_in = Some(i18n_translation_ids);
        criteria
    }

    pub fn has_i18n_text_id_and_id_not_in(
        i18n_text_id: I18nTextId,
        i18n_translation_ids: Vec<I18nTranslationId>
    ) -> Self {
        let mut criteria = Self::default();
        criteria.filter.i18n_text_id = Some(i18n_text_id);
        criteria.filter.id_not_in = Some(i18n_translation_ids);
        criteria
    }
}

impl I18nTranslationText {
    pub fn from_unvalidated(value: String) -> Self {
        Self(value)
    }

    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl I18nTranslationUpdateDate {
    pub fn from_unvalidated(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    fn new() -> Self {
        let value = Utc::now();
        Self(value)
    }

    pub fn value(&self) -> &DateTime<Utc> {
        &self.0
    }
}
