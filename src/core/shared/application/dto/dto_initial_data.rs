use serde::{ Deserialize, Serialize };
use tukosmo_domain::core::language::model::Language;
use tukosmo_domain::core::shared::model::LocalI18n;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoGetInitialData {
    pub language_code: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DtoInitialData {
    pub language_code: String,
    pub languages: Vec<Language>,
    pub local_i18n: LocalI18n,
}
