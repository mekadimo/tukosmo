use toml;
use tukosmo_domain::core::shared::error;
use tukosmo_domain::core::shared::model::DomainError;

use super::file_system;

pub fn parse_file<T>(file_path: &str) -> Result<T, DomainError>
    where T: serde::de::DeserializeOwned
{
    if !file_path.ends_with(".toml") {
        return Err(error::INVALID_TOML_EXTENSION);
    }
    let toml_str = file_system::read_file_as_string(file_path)?;
    let parsed_structure = toml
        ::from_str(&toml_str)
        .map_err(|_e| error::CANNOT_PARSE_TOML_FILE)?;
    Ok(parsed_structure)
}
