use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fmt;

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub enum CoreSubmoduleName {
    Language,
    Shared,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct DomainError {
    pub context: Vec<(String, String)>,
    pub id: DomainErrorId,
    pub message: &'static str,
    pub visibility: DomainErrorVisibility,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct DomainErrorId {
    pub error_code: &'static str,
    pub module: ModuleName,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub enum DomainErrorVisibility {
    Admin,
    Public,
    Server,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub enum ModuleName {
    Core(CoreSubmoduleName),
}

impl DomainError {
    pub fn get_full_code(&self) -> String {
        let prefix = match self.id.module {
            ModuleName::Core(CoreSubmoduleName::Language) => "CORE.LANGUAGE",
            ModuleName::Core(CoreSubmoduleName::Shared) => "CORE.SHARED",
        };
        let error_code = self.id.error_code.to_string();
        format!("{}.{}", prefix, error_code)
    }
}

impl Error for DomainError {}

impl fmt::Debug for DomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Show more info (module, layer, visibility?, etc.)
        write!(
            formatter,
            "DomainError: {} ({})",
            self.id.error_code,
            self.message
        )
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Show more info (module, layer, visibility?, etc.)
        write!(
            formatter,
            "DomainError: {} ({})",
            self.id.error_code,
            self.message
        )
    }
}
