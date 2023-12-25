use chrono::DateTime;
use chrono::Utc;
use diesel::pg::Pg;
use diesel::prelude::AsChangeset;
use diesel::prelude::Identifiable;
use diesel::prelude::Insertable;
use diesel::prelude::Queryable;
use diesel::prelude::Selectable;
use tukosmo_domain::core::language::model::I18nText;
use tukosmo_domain::core::language::model::I18nTextCreationDate;
use tukosmo_domain::core::language::model::I18nTextDefaultText;
use tukosmo_domain::core::language::model::I18nTextId;
use tukosmo_domain::core::language::model::I18nTextUpdateDate;
use uuid::Uuid;

use crate::core::shared::diesel_orm::schema::i18n_text;
use super::db_i18n_translation::DbI18nTranslation;

#[derive(
    AsChangeset,
    Clone,
    Debug,
    Identifiable,
    Insertable,
    PartialEq,
    Queryable,
    Selectable
)]
#[diesel(table_name = i18n_text)]
#[diesel(check_for_backend(Pg))]
pub struct DbI18nText {
    pub creation_date: DateTime<Utc>,
    pub default_text: String,
    pub id: Uuid,
    pub update_date: DateTime<Utc>,
}

impl DbI18nText {
    pub fn from_domain(i18n_text: I18nText) -> Self {
        Self {
            creation_date: i18n_text.creation_date.value().clone(),
            default_text: i18n_text.default_text.value().to_string(),
            id: i18n_text.id.value().clone(),
            update_date: i18n_text.update_date.value().clone(),
        }
    }

    pub fn to_domain(
        self,
        db_translations: Vec<DbI18nTranslation>
    ) -> I18nText {
        let translations = db_translations
            .iter()
            .map(|t| t.clone().to_domain())
            .collect();
        I18nText {
            creation_date: I18nTextCreationDate::from_unvalidated(
                self.creation_date.clone()
            ),
            default_text: I18nTextDefaultText::from_unvalidated(
                self.default_text.clone()
            ),
            id: I18nTextId::from_unvalidated(self.id.clone()),
            translations,
            update_date: I18nTextUpdateDate::from_unvalidated(
                self.update_date.clone()
            ),
        }
    }
}
