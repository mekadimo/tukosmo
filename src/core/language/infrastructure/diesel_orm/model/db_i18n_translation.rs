use chrono::DateTime;
use chrono::Utc;
use diesel::pg::Pg;
use diesel::prelude::AsChangeset;
use diesel::prelude::Associations;
use diesel::prelude::Identifiable;
use diesel::prelude::Insertable;
use diesel::prelude::Queryable;
use diesel::prelude::Selectable;
use tukosmo_domain::core::language::model::I18nTranslation;
use tukosmo_domain::core::language::model::I18nTranslationCreationDate;
use tukosmo_domain::core::language::model::I18nTranslationId;
use tukosmo_domain::core::language::model::I18nTranslationText;
use tukosmo_domain::core::language::model::I18nTranslationUpdateDate;
use tukosmo_domain::core::language::model::I18nTextId;
use tukosmo_domain::core::language::model::LanguageId;
use uuid::Uuid;

use crate::core::shared::diesel_orm::schema::i18n_translation;
use super::db_i18n_text::DbI18nText;
use super::db_language::DbLanguage;

#[derive(
    AsChangeset,
    Associations,
    Clone,
    Debug,
    Identifiable,
    Insertable,
    PartialEq,
    Queryable,
    Selectable
)]
#[diesel(belongs_to(DbI18nText, foreign_key = i18n_text_id))]
#[diesel(belongs_to(DbLanguage, foreign_key = language_id))]
#[diesel(table_name = i18n_translation)]
#[diesel(check_for_backend(Pg))]
pub struct DbI18nTranslation {
    pub creation_date: DateTime<Utc>,
    pub i18n_text_id: Uuid,
    pub id: Uuid,
    pub language_id: Uuid,
    pub text: String,
    pub update_date: DateTime<Utc>,
}

impl DbI18nTranslation {
    pub fn from_domain(
        i18n_translation: I18nTranslation,
        i18n_text_id: I18nTextId
    ) -> Self {
        Self {
            creation_date: i18n_translation.creation_date.value().clone(),
            i18n_text_id: i18n_text_id.value().clone(),
            id: i18n_translation.id.value().clone(),
            language_id: i18n_translation.language_id.value().clone(),
            text: i18n_translation.text.value().to_string(),
            update_date: i18n_translation.update_date.value().clone(),
        }
    }

    pub fn to_domain(self) -> I18nTranslation {
        I18nTranslation {
            creation_date: I18nTranslationCreationDate::from_unvalidated(
                self.creation_date.clone()
            ),
            id: I18nTranslationId::from_unvalidated(self.id.clone()),
            language_id: LanguageId::from_unvalidated(self.language_id.clone()),
            text: I18nTranslationText::from_unvalidated(self.text.clone()),
            update_date: I18nTranslationUpdateDate::from_unvalidated(
                self.update_date.clone()
            ),
        }
    }
}
