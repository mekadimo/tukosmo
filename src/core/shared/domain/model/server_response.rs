use serde::{ Deserialize, Serialize };

use super::DomainError;

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub enum ServerResponse<T> {
    Error(ServerResponseError),
    Response(T),
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct ServerResponseError {
    pub error_code: String,
    pub context: Vec<(String, String)>,
}

impl<T> ServerResponse<T> {
    pub fn build<F>(server_function: F) -> Self
        where F: FnOnce() -> Result<T, DomainError>
    {
        match server_function() {
            Ok(response) => Self::Response(response),
            Err(domain_error) =>
                Self::Error(ServerResponseError {
                    error_code: domain_error.get_full_code(),
                    context: domain_error.context,
                }),
        }
    }
}
