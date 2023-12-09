use dotenvy::dotenv;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str;
use std::env;
use std::fs;
use tukosmo_domain::core::shared::error;
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::shared::model::LocalI18n;
use tukosmo_domain::core::shared::model::ServerConfig;
use tukosmo_domain::core::shared::repository::DataRepository;

use super::super::service::file_system;
use super::super::service::toml;

#[derive(Clone, Deserialize, Serialize)]
pub struct FsDataRepository {
    pub acme_challenge_dir_path: String,
    pub cert_chain_file_path: String,
    pub cert_file_path: String,
    pub cert_pkey_file_path: String,
    pub certs_dir_path: String,
    pub data_dir_path: String,
    pub intermediate_cert_file_path: String,
    pub locale_dir_path: String,
    pub tukosmo_toml_file_path: String,
}

const DATA_DIR_ENV_VAR: &'static str = "TUKOSMO_DATA_DIR";

impl DataRepository for FsDataRepository {
    fn check_tls_certificate_file_exists(&self) -> Result<bool, DomainError> {
        let file_exists = file_system::check_file_exists(&self.cert_file_path)?;
        Ok(file_exists)
    }

    fn clean_certs_dir(&self) -> Result<(), DomainError> {
        file_system::remove_directory(&self.certs_dir_path)?;
        file_system::create_directory(&self.certs_dir_path)?;
        Ok(())
    }

    fn create_new_acme_challenge_dir(&self) -> Result<(), DomainError> {
        file_system::remove_directory(&self.acme_challenge_dir_path)?;
        file_system::create_directory(&self.acme_challenge_dir_path)?;
        Ok(())
    }

    fn get_local_i18n(
        &self,
        language_code: &str
    ) -> Result<LocalI18n, DomainError> {
        let applied_language_code =
            LocalI18n::get_applied_language_code(language_code);
        let locale_file_path = format!(
            "{}/{}.json",
            self.locale_dir_path,
            applied_language_code
        );
        let i18n_json_str = file_system::read_file_as_string(
            &locale_file_path
        )?;
        // TODO: Create new DomainError and return Err() instead of unwrap()
        let local_i18n: LocalI18n = from_str(&i18n_json_str).unwrap();
        Ok(local_i18n)
    }

    fn get_server_config(&self) -> Result<ServerConfig, DomainError> {
        let server_config: ServerConfig = toml::parse_file(
            &self.tukosmo_toml_file_path
        )?;
        Ok(server_config)
    }

    fn get_tls_certificate_chain_file(&self) -> Result<fs::File, DomainError> {
        let cert_chain_file = file_system::read_file(
            &self.cert_chain_file_path
        )?;
        Ok(cert_chain_file)
    }

    fn get_tls_certificate_file(&self) -> Result<fs::File, DomainError> {
        let cert_file = file_system::read_file(&self.cert_file_path)?;
        Ok(cert_file)
    }

    fn get_tls_certificate_file_bytes(&self) -> Result<Vec<u8>, DomainError> {
        let cert_file_bytes = file_system::read_file_as_vec_u8(
            &self.cert_file_path
        )?;
        Ok(cert_file_bytes)
    }

    fn get_tls_certificate_file_string(&self) -> Result<String, DomainError> {
        let cert_file_string = file_system::read_file_as_string(
            &self.cert_file_path
        )?;
        Ok(cert_file_string)
    }

    fn get_tls_certificate_pkey_file(&self) -> Result<fs::File, DomainError> {
        let cert_pkey_file = file_system::read_file(&self.cert_pkey_file_path)?;
        Ok(cert_pkey_file)
    }

    fn get_tls_intermediate_certificate_file_string(
        &self
    ) -> Result<String, DomainError> {
        let intermediate_cert_file_string = file_system::read_file_as_string(
            &self.intermediate_cert_file_path
        )?;
        Ok(intermediate_cert_file_string)
    }

    fn remove_acme_challenge_dir(&self) -> Result<(), DomainError> {
        file_system::remove_directory(&self.acme_challenge_dir_path)?;
        Ok(())
    }

    fn write_acme_challenge_token_file(
        &self,
        token: &str,
        proof: &str
    ) -> Result<(), DomainError> {
        let acme_challenge_token_file_path = format!(
            "{}/{}",
            self.acme_challenge_dir_path,
            token
        );
        file_system::write_file(&acme_challenge_token_file_path, proof)?;
        Ok(())
    }

    fn write_tls_certificate_chain_file(
        &self,
        content: &str
    ) -> Result<(), DomainError> {
        file_system::write_file(&self.cert_chain_file_path, content)?;
        Ok(())
    }

    fn write_tls_certificate_file(
        &self,
        content: &str
    ) -> Result<(), DomainError> {
        file_system::write_file(&self.cert_file_path, content)?;
        Ok(())
    }

    fn write_tls_certificate_pkey_file(
        &self,
        content: &str
    ) -> Result<(), DomainError> {
        file_system::write_file(&self.cert_pkey_file_path, content)?;
        Ok(())
    }

    fn write_tls_intermediate_certificate_file(
        &self,
        content: &str
    ) -> Result<(), DomainError> {
        file_system::write_file(&self.intermediate_cert_file_path, content)?;
        Ok(())
    }
}

impl FsDataRepository {
    pub fn init() -> Result<FsDataRepository, DomainError> {
        dotenv().ok();
        let data_dir_path = env
            ::var(DATA_DIR_ENV_VAR)
            .map_err(|_e| error::CANNOT_OBTAIN_TUKOSMO_DATA_DIR_ENV_VAR)?;

        let data_dir_exists = file_system::check_directory_exists(
            &data_dir_path
        )?;
        if !data_dir_exists {
            return Err(error::DATA_DIR_DOES_NOT_EXIST);
        }

        let acme_challenge_dir_path = format!(
            "{}/tmp/acme-challenge",
            &data_dir_path
        );

        let certs_dir_path = format!("{}/certs", &data_dir_path);
        let cert_file_path = format!("{}/server-cert.pem", &certs_dir_path);
        let cert_chain_file_path = format!(
            "{}/server-cert-chain.pem",
            &certs_dir_path
        );
        let cert_pkey_file_path = format!(
            "{}/server-cert-pkey.pem",
            &certs_dir_path
        );
        let intermediate_cert_file_path = format!(
            "{}/intermediate-cert.pem",
            &certs_dir_path
        );

        let locale_dir_path = format!("{}/assets/locale", &data_dir_path);

        let tukosmo_toml_file_path = format!("{}/Tukosmo.toml", &data_dir_path);

        Ok(FsDataRepository {
            acme_challenge_dir_path,
            cert_chain_file_path,
            cert_file_path,
            cert_pkey_file_path,
            certs_dir_path,
            data_dir_path,
            intermediate_cert_file_path,
            locale_dir_path,
            tukosmo_toml_file_path,
        })
    }
}
