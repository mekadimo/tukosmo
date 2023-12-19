use serde::Deserialize;
use serde::Serialize;
use tukosmo_domain::core::language::model::I18nTextValue;
use tukosmo_domain::core::language::model::Language;

use crate::core::shared::dto::DtoDeleteForm;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoAddLanguage {
    pub form: DtoLanguageForm,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoDeleteLanguage {
    pub form: DtoDeleteForm,
    pub language_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoEditLanguage {
    pub form: DtoLanguageForm,
    pub language_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoGetLanguage {
    pub language_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoGetLanguagesPaginated {
    pub current_page: i64,
    pub results_per_page: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoLanguageForm {
    pub code: String,
    pub name: I18nTextValue,
    pub original_name: String,
    pub website_title: String,
    pub website_subtitle: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DtoLanguagesPaginated {
    pub languages: Vec<Language>,
    pub total_results: i64,
}
