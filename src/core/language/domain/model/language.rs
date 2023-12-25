use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::core::shared::error;
use crate::core::shared::model::DomainError;
use crate::core::shared::model::PaginationCriteria;
use super::I18nText;
use super::I18nTextValue;

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Language {
    pub code: LanguageCode,
    pub creation_date: LanguageCreationDate,
    pub name: I18nText,
    pub id: LanguageId,
    pub original_name: LanguageOriginalName,
    pub update_date: LanguageUpdateDate,
    pub website_subtitle: LanguageWebsiteSubtitle,
    pub website_title: LanguageWebsiteTitle,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct LanguageCode(String);

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct LanguageCreationDate(DateTime<Utc>);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LanguageId(Uuid);

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct LanguageOriginalName(String);

#[derive(Clone)]
pub struct LanguageSearchCriteria {
    pub filter: LanguageSearchFilterCriteria,
    pub order_by: Option<LanguageSearchCriteriaOrderBy>,
    pub pagination: Option<PaginationCriteria>,
}

#[derive(Clone)]
pub enum LanguageSearchCriteriaOrderBy {
    CreationDate,
    OriginalName,
}

#[derive(Clone)]
pub struct LanguageSearchFilterCriteria {
    pub code: Option<LanguageCode>,
    pub id: Option<LanguageId>,
    pub not_id: Option<LanguageId>,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct LanguageUpdateDate(DateTime<Utc>);

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct LanguageWebsiteSubtitle(String);

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct LanguageWebsiteTitle(String);

const LANGUAGE_CODE_MAX_LENGTH: &'static usize = &5;
const LANGUAGE_CODE_MIN_LENGTH: &'static usize = &2;

impl Language {
    pub fn modify(
        &mut self,
        code: LanguageCode,
        name_value: I18nTextValue,
        original_name: LanguageOriginalName,
        website_title: LanguageWebsiteTitle,
        website_subtitle: LanguageWebsiteSubtitle
    ) -> Result<(), DomainError> {
        self.code = code;
        self.name.modify(
            name_value,
            Self::validate_name_default_value,
            Self::validate_name_translation_value
        )?;
        self.update_date = LanguageUpdateDate::new();
        self.original_name = original_name;
        self.website_title = website_title;
        self.website_subtitle = website_subtitle;
        Ok(())
    }

    pub fn new(
        code: LanguageCode,
        name_value: I18nTextValue,
        original_name: LanguageOriginalName,
        website_title: LanguageWebsiteTitle,
        website_subtitle: LanguageWebsiteSubtitle
    ) -> Result<Self, DomainError> {
        let id = LanguageId::new();
        let name = I18nText::new(
            name_value,
            Self::validate_name_default_value,
            Self::validate_name_translation_value
        )?;
        let creation_date = LanguageCreationDate::new();
        let update_date = LanguageUpdateDate::new();
        let original_name = original_name;
        let website_title = website_title;
        let website_subtitle = website_subtitle;

        Ok(Language {
            code,
            creation_date,
            name,
            id,
            original_name,
            update_date,
            website_subtitle,
            website_title,
        })
    }

    pub fn validate_name_default_value(value: &String) -> Option<DomainError> {
        if 0 == value.len() {
            return Some(error::FIELD_CANNOT_BE_EMPTY);
        }
        Self::validate_name_value(value)
    }

    pub fn validate_name_translation_value(
        value: &String
    ) -> Option<DomainError> {
        Self::validate_name_value(value)
    }

    pub fn validate_name_value(_value: &String) -> Option<DomainError> {
        None
    }
}

impl LanguageCode {
    pub fn from_unvalidated(value: String) -> Self {
        Self(value)
    }

    pub fn new(value: String) -> Result<Self, DomainError> {
        match Self::validate(&value) {
            Some(validation_error) => Err(validation_error),
            None => Ok(Self(value)),
        }
    }

    pub fn validate(value: &String) -> Option<DomainError> {
        let value_length = value.len();
        if 0 == value_length {
            return Some(error::FIELD_CANNOT_BE_EMPTY);
        }
        if &value_length < LANGUAGE_CODE_MIN_LENGTH {
            return Some(error::TEXT_DOESNT_REACH_MIN_LENGTH);
        }
        if &value_length > LANGUAGE_CODE_MAX_LENGTH {
            return Some(error::TEXT_EXCEEDS_MAX_LENGTH);
        }
        None
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl LanguageCreationDate {
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

impl LanguageId {
    pub fn from_unvalidated(value: Uuid) -> Self {
        Self(value)
    }

    pub fn from_string(value: &str) -> Result<Self, DomainError> {
        match Uuid::parse_str(value) {
            Ok(value) => Ok(Self(value)),
            Err(_) => Err(error::INVALID_UUID),
        }
    }

    pub fn new() -> Self {
        let value = Uuid::new_v4();
        Self(value)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl LanguageOriginalName {
    pub fn from_unvalidated(value: String) -> Self {
        Self(value)
    }

    pub fn new(value: String) -> Result<Self, DomainError> {
        match Self::validate(&value) {
            Some(validation_error) => Err(validation_error),
            None => Ok(Self(value)),
        }
    }

    pub fn validate(value: &String) -> Option<DomainError> {
        if 0 == value.len() {
            return Some(error::FIELD_CANNOT_BE_EMPTY);
        }
        Language::validate_name_value(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl LanguageSearchCriteria {
    pub fn all() -> Self {
        let criteria = Self::default();
        criteria
    }

    pub fn all_ordered() -> Self {
        let mut criteria = Self::default();
        criteria.order_by = Some(LanguageSearchCriteriaOrderBy::OriginalName);
        criteria
    }

    fn default() -> Self {
        Self {
            filter: LanguageSearchFilterCriteria {
                code: None,
                id: None,
                not_id: None,
            },
            order_by: None,
            pagination: None,
        }
    }

    pub fn has_code(code: LanguageCode) -> Self {
        let mut criteria = Self::default();
        criteria.filter.code = Some(code);
        criteria
    }

    pub fn has_code_and_not_id(code: LanguageCode, not_id: LanguageId) -> Self {
        let mut criteria = Self::default();
        criteria.filter.code = Some(code);
        criteria.filter.not_id = Some(not_id);
        criteria
    }

    pub fn has_id(language_id: LanguageId) -> Self {
        let mut criteria = Self::default();
        criteria.filter.id = Some(language_id);
        criteria
    }

    pub fn paginated(page: i64, results_per_page: i64) -> Self {
        let mut criteria = Self::default();
        criteria.pagination = Some(PaginationCriteria {
            page,
            results_per_page,
        });
        criteria.order_by = Some(LanguageSearchCriteriaOrderBy::OriginalName);
        criteria
    }
}

impl LanguageUpdateDate {
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

impl LanguageWebsiteSubtitle {
    pub fn from_unvalidated(value: String) -> Self {
        Self(value)
    }

    pub fn new(value: String) -> Result<Self, DomainError> {
        match Self::validate(&value) {
            Some(validation_error) => Err(validation_error),
            None => Ok(Self(value)),
        }
    }

    pub fn validate(value: &String) -> Option<DomainError> {
        if 0 == value.len() {
            return Some(error::FIELD_CANNOT_BE_EMPTY);
        }
        None
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl LanguageWebsiteTitle {
    pub fn from_unvalidated(value: String) -> Self {
        Self(value)
    }

    pub fn new(value: String) -> Result<Self, DomainError> {
        match Self::validate(&value) {
            Some(validation_error) => Err(validation_error),
            None => Ok(Self(value)),
        }
    }

    pub fn validate(value: &String) -> Option<DomainError> {
        if 0 == value.len() {
            return Some(error::FIELD_CANNOT_BE_EMPTY);
        }
        None
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
