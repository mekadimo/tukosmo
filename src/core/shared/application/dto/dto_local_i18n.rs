use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoGetLocalI18n {
    pub language_code: String,
}
