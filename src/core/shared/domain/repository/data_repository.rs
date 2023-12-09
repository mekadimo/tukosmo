use std::fs::File;

use super::super::model::DomainError;
use super::super::model::LocalI18n;
use super::super::model::ServerConfig;

pub trait DataRepository {
    fn check_tls_certificate_file_exists(&self) -> Result<bool, DomainError>;

    fn clean_certs_dir(&self) -> Result<(), DomainError>;

    fn create_new_acme_challenge_dir(&self) -> Result<(), DomainError>;

    fn get_local_i18n(
        &self,
        language_code: &str
    ) -> Result<LocalI18n, DomainError>;

    fn get_server_config(&self) -> Result<ServerConfig, DomainError>;

    fn get_tls_certificate_chain_file(&self) -> Result<File, DomainError>;

    fn get_tls_certificate_file(&self) -> Result<File, DomainError>;

    fn get_tls_certificate_file_bytes(&self) -> Result<Vec<u8>, DomainError>;

    fn get_tls_certificate_file_string(&self) -> Result<String, DomainError>;

    fn get_tls_certificate_pkey_file(&self) -> Result<File, DomainError>;

    fn get_tls_intermediate_certificate_file_string(
        &self
    ) -> Result<String, DomainError>;

    fn remove_acme_challenge_dir(&self) -> Result<(), DomainError>;

    fn write_acme_challenge_token_file(
        &self,
        token: &str,
        proof: &str
    ) -> Result<(), DomainError>;

    fn write_tls_certificate_chain_file(
        &self,
        content: &str
    ) -> Result<(), DomainError>;

    fn write_tls_certificate_file(
        &self,
        content: &str
    ) -> Result<(), DomainError>;

    fn write_tls_certificate_pkey_file(
        &self,
        content: &str
    ) -> Result<(), DomainError>;

    fn write_tls_intermediate_certificate_file(
        &self,
        content: &str
    ) -> Result<(), DomainError>;
}
