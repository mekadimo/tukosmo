use chrono::DateTime;
use chrono::Utc;
use diesel::pg::Pg;
use diesel::prelude::AsChangeset;
use diesel::prelude::Associations;
use diesel::prelude::Identifiable;
use diesel::prelude::Insertable;
use diesel::prelude::Queryable;
use diesel::prelude::Selectable;
use tukosmo_domain::core::language::model::I18nText;
use tukosmo_domain::core::language::model::Language;
use tukosmo_domain::core::language::model::LanguageCode;
use tukosmo_domain::core::language::model::LanguageCreationDate;
use tukosmo_domain::core::language::model::LanguageId;
use tukosmo_domain::core::language::model::LanguageOriginalName;
use tukosmo_domain::core::language::model::LanguageUpdateDate;
use tukosmo_domain::core::language::model::LanguageWebsiteSubtitle;
use tukosmo_domain::core::language::model::LanguageWebsiteTitle;
use uuid::Uuid;

use crate::core::shared::diesel_orm::schema::language;
use super::db_i18n_text::DbI18nText;

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
#[diesel(belongs_to(DbI18nText, foreign_key = i18n_text_id_name))]
#[diesel(table_name = language)]
#[diesel(check_for_backend(Pg))]
pub struct DbLanguage {
    pub code: String,
    pub creation_date: DateTime<Utc>,
    pub i18n_text_id_name: Uuid,
    pub id: Uuid,
    pub original_name: String,
    pub update_date: DateTime<Utc>,
    pub website_subtitle: String,
    pub website_title: String,
}

impl DbLanguage {
    pub fn from_domain(language: Language) -> Self {
        Self {
            code: language.code.value().to_string(),
            creation_date: language.creation_date.value().clone(),
            i18n_text_id_name: language.name.id.value().clone(),
            id: language.id.value().clone(),
            original_name: language.original_name.value().to_string(),
            update_date: language.update_date.value().clone(),
            website_subtitle: language.website_subtitle.value().to_string(),
            website_title: language.website_title.value().to_string(),
        }
    }

    pub fn to_domain(self, language_name: I18nText) -> Language {
        Language {
            code: LanguageCode::from(self.code.clone()),
            creation_date: LanguageCreationDate::from(
                self.creation_date.clone()
            ),
            name: language_name,
            id: LanguageId::from(self.id.clone()),
            original_name: LanguageOriginalName::from(
                self.original_name.clone()
            ),
            update_date: LanguageUpdateDate::from(self.update_date.clone()),
            website_subtitle: LanguageWebsiteSubtitle::from(
                self.website_subtitle.clone()
            ),
            website_title: LanguageWebsiteTitle::from(
                self.website_title.clone()
            ),
        }
    }
}
