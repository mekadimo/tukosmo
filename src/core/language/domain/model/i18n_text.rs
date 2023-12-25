use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::core::shared::error;
use crate::core::shared::model::DomainError;
use super::I18nTranslation;
use super::I18nTranslationText;
use super::LanguageId;

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct I18nText {
    pub creation_date: I18nTextCreationDate,
    pub default_text: I18nTextDefaultText,
    pub id: I18nTextId,
    pub translations: Vec<I18nTranslation>,
    pub update_date: I18nTextUpdateDate,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct I18nTextCreationDate(DateTime<Utc>);

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct I18nTextDefaultText(String);

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct I18nTextId(Uuid);

#[derive(Clone)]
pub struct I18nTextSearchCriteria {
    pub filter: I18nTextSearchFilterCriteria,
}

#[derive(Clone)]
pub struct I18nTextSearchFilterCriteria {
    pub id: Option<I18nTextId>,
    pub id_in: Option<Vec<I18nTextId>>,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct I18nTextUpdateDate(DateTime<Utc>);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct I18nTextValue {
    pub default_text: String,
    pub translations: Vec<(Uuid, String)>,
}

impl I18nText {
    pub fn modify(
        &mut self,
        value: I18nTextValue,
        validate_default: fn(&String) -> Option<DomainError>,
        validate_translation: fn(&String) -> Option<DomainError>
    ) -> Result<(), DomainError> {
        self.default_text = I18nTextDefaultText::new(
            value.default_text,
            validate_default
        )?;
        self.modify_translations(value.translations, validate_translation)?;
        self.update_date = I18nTextUpdateDate::new();
        Ok(())
    }

    fn modify_translations(
        &mut self,
        translation_values: Vec<(Uuid, String)>,
        validate_translation: fn(&String) -> Option<DomainError>
    ) -> Result<(), DomainError> {
        for (language_id, text_value) in translation_values {
            if let Some(validation_error) = validate_translation(&text_value) {
                return Err(validation_error);
            }

            if
                let Some(translation) = self.translations
                    .iter_mut()
                    .find(|t| t.language_id.value() == &language_id)
            {
                if !text_value.trim().is_empty() {
                    translation.modify(I18nTranslationText::new(text_value));
                } else {
                    self.translations.retain(
                        |t| t.language_id.value() != &language_id
                    );
                }
            } else {
                if !text_value.trim().is_empty() {
                    self.translations.push(
                        I18nTranslation::new(
                            LanguageId::from_unvalidated(language_id),
                            I18nTranslationText::new(text_value)
                        )
                    );
                }
            }
        }

        Ok(())
    }

    pub fn new(
        value: I18nTextValue,
        validate_default: fn(&String) -> Option<DomainError>,
        validate_translation: fn(&String) -> Option<DomainError>
    ) -> Result<Self, DomainError> {
        let id = I18nTextId::new();
        let default_text = I18nTextDefaultText::new(
            value.default_text,
            validate_default
        )?;
        let translations = Self::new_translations(
            value.translations,
            validate_translation
        )?;
        let creation_date = I18nTextCreationDate::new();
        let update_date = I18nTextUpdateDate::new();
        Ok(Self {
            creation_date,
            default_text,
            id,
            translations,
            update_date,
        })
    }

    fn new_translations(
        translation_values: Vec<(Uuid, String)>,
        validate_translation: fn(&String) -> Option<DomainError>
    ) -> Result<Vec<I18nTranslation>, DomainError> {
        for (_, translation_value) in translation_values.iter() {
            if
                let Some(validation_error) =
                    validate_translation(translation_value)
            {
                return Err(validation_error);
            }
        }
        let translations = translation_values
            .iter()
            .filter(|(_, value)| { !value.trim().is_empty() })
            .map(|(language_id, text_value)| {
                I18nTranslation::new(
                    LanguageId::from_unvalidated(*language_id),
                    I18nTranslationText::new(text_value.clone())
                )
            })
            .collect();
        Ok(translations)
    }

    pub fn translate(&self, language_id: LanguageId) -> String {
        let translation = self.translations
            .iter()
            .find(|t| t.language_id.value() == language_id.value());

        let translated_text = match translation {
            Some(translation) => translation.text.value(),
            None => self.default_text.value(),
        };
        translated_text.to_string()
    }
}

impl I18nTextCreationDate {
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

impl I18nTextDefaultText {
    pub fn from_unvalidated(value: String) -> Self {
        Self(value)
    }

    fn new(
        value: String,
        validate: fn(&String) -> Option<DomainError>
    ) -> Result<Self, DomainError> {
        match validate(&value) {
            Some(validation_error) => Err(validation_error),
            None => Ok(Self(value)),
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl I18nTextId {
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

impl I18nTextSearchCriteria {
    fn default() -> Self {
        Self {
            filter: I18nTextSearchFilterCriteria { id: None, id_in: None },
        }
    }

    pub fn has_id(i18n_text_id: I18nTextId) -> Self {
        let mut criteria = Self::default();
        criteria.filter.id = Some(i18n_text_id);
        criteria
    }

    pub fn has_id_in(i18n_text_ids: Vec<I18nTextId>) -> Self {
        let mut criteria = Self::default();
        criteria.filter.id_in = Some(i18n_text_ids);
        criteria
    }
}

impl I18nTextUpdateDate {
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
