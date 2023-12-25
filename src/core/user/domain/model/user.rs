use chrono::DateTime;
use chrono::Utc;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::core::shared::error;
use crate::core::shared::model::DomainError;
use crate::core::shared::model::PaginationCriteria;
use crate::core::language::model::I18nText;
use crate::core::language::model::I18nTextValue;

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct User {
    pub creation_date: UserCreationDate,
    pub email: UserEmail,
    pub id: UserId,
    pub is_admin: UserIsAdmin,
    pub is_suspended: UserIsSuspended,
    pub name: I18nText,
    pub update_date: UserUpdateDate,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct UserCreationDate(DateTime<Utc>);

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct UserEmail(String);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UserId(Uuid);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UserIsAdmin(bool);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UserIsSuspended(bool);

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct UserPlaintextPassword(String);

#[derive(Clone)]
pub struct UserSearchCriteria {
    pub filter: UserSearchFilterCriteria,
    pub order_by: Option<UserSearchCriteriaOrderBy>,
    pub pagination: Option<PaginationCriteria>,
}

#[derive(Clone)]
pub enum UserSearchCriteriaOrderBy {
    CreationDate,
    Email,
}

#[derive(Clone)]
pub struct UserSearchFilterCriteria {
    pub email: Option<UserEmail>,
    pub id: Option<UserId>,
    pub is_admin: Option<UserIsAdmin>,
    pub not_id: Option<UserId>,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct UserUpdateDate(DateTime<Utc>);

const USER_EMAIL_MAX_LENGTH: &'static usize = &320;
const USER_PASSWORD_MAX_LENGTH: &'static usize = &32;

impl User {
    pub fn modify(
        &mut self,
        email: UserEmail,
        name_value: I18nTextValue,
        is_admin: UserIsAdmin,
        is_suspended: UserIsSuspended
    ) -> Result<(), DomainError> {
        self.email = email;
        self.name.modify(
            name_value,
            Self::validate_name_default_value,
            Self::validate_name_translation_value
        )?;
        self.is_admin = is_admin;
        self.is_suspended = is_suspended;
        self.update_date = UserUpdateDate::new();
        Ok(())
    }

    pub fn new(
        email: UserEmail,
        name_value: I18nTextValue,
        is_admin: UserIsAdmin,
        is_suspended: UserIsSuspended
    ) -> Result<Self, DomainError> {
        let id = UserId::new();
        let name = I18nText::new(
            name_value,
            Self::validate_name_default_value,
            Self::validate_name_translation_value
        )?;
        let creation_date = UserCreationDate::new();
        let update_date = UserUpdateDate::new();

        Ok(User {
            creation_date,
            email,
            id,
            is_admin,
            is_suspended,
            name,
            update_date,
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

impl UserCreationDate {
    pub fn from(value: DateTime<Utc>) -> Self {
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

impl UserEmail {
    pub fn from(value: String) -> Self {
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
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
        ).unwrap();
        if !email_regex.is_match(value) {
            return Some(error::INVALID_EMAIL);
        }
        if &value_length > USER_EMAIL_MAX_LENGTH {
            return Some(error::TEXT_EXCEEDS_MAX_LENGTH);
        }
        None
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl UserId {
    pub fn from(value: Uuid) -> Self {
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

impl UserIsAdmin {
    pub fn from(value: bool) -> Self {
        Self(value)
    }

    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &bool {
        &self.0
    }
}

impl UserIsSuspended {
    pub fn from(value: bool) -> Self {
        Self(value)
    }

    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &bool {
        &self.0
    }
}

impl UserPlaintextPassword {
    pub fn new(value: String) -> Result<Self, DomainError> {
        match Self::validate(&value) {
            Some(validation_error) => Err(validation_error),
            None => Ok(Self(value)),
        }
    }

    pub fn plaintext_value(&self) -> &str {
        &self.0
    }

    pub fn validate(value: &String) -> Option<DomainError> {
        let value_length = value.len();
        if 0 == value_length {
            return Some(error::FIELD_CANNOT_BE_EMPTY);
        }
        if &value_length > USER_PASSWORD_MAX_LENGTH {
            return Some(error::TEXT_EXCEEDS_MAX_LENGTH);
        }
        // TODO: Min length, allowed characters, etc.
        None
    }
}

impl UserSearchCriteria {
    pub fn all() -> Self {
        let criteria = Self::default();
        criteria
    }

    pub fn all_ordered() -> Self {
        let mut criteria = Self::default();
        criteria.order_by = Some(UserSearchCriteriaOrderBy::Email);
        criteria
    }

    fn default() -> Self {
        Self {
            filter: UserSearchFilterCriteria {
                email: None,
                id: None,
                is_admin: None,
                not_id: None,
            },
            order_by: None,
            pagination: None,
        }
    }

    pub fn has_email(email: UserEmail) -> Self {
        let mut criteria = Self::default();
        criteria.filter.email = Some(email);
        criteria
    }

    pub fn has_email_and_not_id(email: UserEmail, not_id: UserId) -> Self {
        let mut criteria = Self::default();
        criteria.filter.email = Some(email);
        criteria.filter.not_id = Some(not_id);
        criteria
    }

    pub fn has_id(user_id: UserId) -> Self {
        let mut criteria = Self::default();
        criteria.filter.id = Some(user_id);
        criteria
    }

    pub fn is_admin(is_admin: UserIsAdmin) -> Self {
        let mut criteria = Self::default();
        criteria.filter.is_admin = Some(is_admin);
        criteria
    }

    pub fn paginated(page: i64, results_per_page: i64) -> Self {
        let mut criteria = Self::default();
        criteria.pagination = Some(PaginationCriteria {
            page,
            results_per_page,
        });
        criteria.order_by = Some(UserSearchCriteriaOrderBy::Email);
        criteria
    }
}

impl UserUpdateDate {
    pub fn from(value: DateTime<Utc>) -> Self {
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
