use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoDeleteForm {
    pub requested: bool,
}
