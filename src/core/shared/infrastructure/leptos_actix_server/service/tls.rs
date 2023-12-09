use acme_micro::Directory;
use acme_micro::DirectoryUrl;
use acme_micro::create_p384_key;
use openssl::asn1::Asn1Time;
use openssl::asn1::Asn1TimeRef;
use openssl::x509::X509;
use rcgen;
use reqwest;
use rustls;
use rustls_pemfile;
use std::convert::TryInto;
use std::io::BufReader;
use std::time::Duration;
use tokio;
use tukosmo_domain::core::shared::error;
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::shared::repository::DataRepository;

use crate::core::shared::leptos_actix_server::repository::FsDataRepository;
use crate::core::shared::leptos_actix_server::service::server::Handle;
use crate::core::shared::leptos_actix_server::service::server::start_acme_challenge_server;

pub const HSTS_HEADER_VALUE: &'static str = "max-age=63072000"; // 2 years
const INTERMEDIATE_CERTIFICATE_LETS_ENCRYPT_URL: &'static str =
    "https://letsencrypt.org/certs/lets-encrypt-r3.pem";
const MILLISECONDS_UNTIL_ACME_CHALLENGE_VALIDATION: u64 = 5000;
const MILLISECONDS_UNTIL_ACME_CSR_VALIDATION: u64 = 5000;
const MIN_SECONDS_WITHOUT_RENEWAL: i32 = 2592000; // 30 days

fn cert_needs_renewal(cert: &X509) -> Result<bool, DomainError> {
    let server_config = FsDataRepository::init()?.get_server_config()?;
    if server_config.has_development_mode() {
        return Ok(false);
    }
    let expiration_time = cert.not_after();
    let current_time = Asn1Time::days_from_now(0).unwrap();

    let has_expired = expiration_time < current_time;

    let time_difference = expiration_time.diff(&current_time).unwrap();
    let seconds_diff = time_difference.secs.abs();
    let diff_less_than_wanted = seconds_diff < MIN_SECONDS_WITHOUT_RENEWAL;

    let needs_renewal = has_expired || diff_less_than_wanted;

    Ok(needs_renewal)
}

async fn generate_new_certificate() -> Result<(), DomainError> {
    let fs_data_repository = FsDataRepository::init()?;

    fs_data_repository.clean_certs_dir()?;

    let server_config = fs_data_repository.get_server_config()?;
    if server_config.has_production_mode() {
        obtain_lets_encrypt_certificate().await?;
    } else {
        obtain_development_certificate().await?;
    }

    Ok(())
}

pub async fn load_rustls_config() -> Result<rustls::ServerConfig, DomainError> {
    let fs_data_repository = FsDataRepository::init()?;

    let cert_does_not_exist =
        !fs_data_repository.check_tls_certificate_file_exists()?;
    if cert_does_not_exist {
        generate_new_certificate().await?;
    } else {
        let needs_renewal = match
            fs_data_repository.get_tls_certificate_file_bytes()
        {
            Ok(cert_bytes) => {
                let cert = X509::from_pem(&cert_bytes).map_err(
                    |_e| error::CANNOT_PARSE_TLS_CERTIFICATE
                )?;
                cert_needs_renewal(&cert)?
            }
            Err(_) => true, // TODO: Fix error doesn't imply needs renewal
        };
        if needs_renewal {
            generate_new_certificate().await?;
        }
    }

    let rustls_config = rustls::ServerConfig
        ::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_chain_file = fs_data_repository.get_tls_certificate_chain_file()?;
    let cert_chain_file_buffer = &mut BufReader::new(cert_chain_file);

    let cert_pkey_file = fs_data_repository.get_tls_certificate_pkey_file()?;
    let cert_pkey_file_buffer = &mut BufReader::new(cert_pkey_file);

    let cert_chain = rustls_pemfile
        ::certs(cert_chain_file_buffer)
        .map_err(|_e| error::CANNOT_PARSE_TLS_CERTIFICATE)?
        .into_iter()
        .map(rustls::Certificate)
        .collect();

    let mut pkeys: Vec<rustls::PrivateKey> = rustls_pemfile
        ::pkcs8_private_keys(cert_pkey_file_buffer)
        .map_err(|_e| error::CANNOT_PARSE_TLS_CERTIFICATE_PKEY)?
        .into_iter()
        .map(rustls::PrivateKey)
        .collect();

    if pkeys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    Ok(rustls_config.with_single_cert(cert_chain, pkeys.remove(0)).unwrap())
}

async fn obtain_development_certificate() -> Result<(), DomainError> {
    let fs_data_repository = FsDataRepository::init()?;
    let server_config = fs_data_repository.get_server_config()?;

    let domain = server_config.domain;
    let subject_alt_names = vec![domain];

    let cert = rcgen
        ::generate_simple_self_signed(subject_alt_names)
        .map_err(|_e| error::CANNOT_GENERATE_TLS_CERTIFICATE)?;

    let cert_pkey_str = cert.serialize_private_key_pem();
    let cert_str = cert
        .serialize_pem()
        .map_err(|_e| error::CANNOT_GENERATE_TLS_CERTIFICATE)?;

    fs_data_repository.write_tls_certificate_pkey_file(&cert_pkey_str)?;
    fs_data_repository.write_tls_certificate_file(&cert_str)?;
    fs_data_repository.write_tls_certificate_chain_file(&cert_str)?;

    Ok(())
}

async fn obtain_lets_encrypt_certificate() -> Result<(), DomainError> {
    let fs_data_repository = FsDataRepository::init()?;
    let server_config = fs_data_repository.get_server_config()?;

    fs_data_repository.create_new_acme_challenge_dir()?;

    let server = start_acme_challenge_server(&server_config.domain)?;

    let url = if server_config.has_production_mode() {
        DirectoryUrl::LetsEncrypt
    } else {
        DirectoryUrl::LetsEncryptStaging
    };
    let dir = Directory::from_url(url).unwrap();
    let user_email_mailto: String = "mailto:{email}".replace(
        "{email}",
        &server_config.admin_email
    );
    let contact = vec![user_email_mailto];
    let account = dir.register_account(contact.clone()).unwrap();
    let account_pkey = account.acme_private_key_pem().unwrap();
    let loaded_account = dir.load_account(&account_pkey, contact).unwrap();
    let mut new_order = loaded_account
        .new_order(&server_config.domain, &[])
        .unwrap();

    let csr_order = loop {
        if let Some(csr_order) = new_order.confirm_validations() {
            break csr_order;
        }

        let auths = new_order.authorizations().unwrap();
        let challenge = auths[0].http_challenge().unwrap();

        // For HTTP, the challenge is a text file that needs to be accessible
        // over the web for the domain we are trying to get a certificate for:
        // http://mydomain.io/.well-known/acme-challenge/<token>
        // (the token is the filename and the proof is the content of the file)
        let token = challenge.http_token();
        let proof = challenge.http_proof().unwrap();
        fs_data_repository.write_acme_challenge_token_file(token, &proof)?;

        challenge
            .validate(
                Duration::from_millis(
                    MILLISECONDS_UNTIL_ACME_CHALLENGE_VALIDATION
                )
            )
            .unwrap();

        new_order.refresh().unwrap();
    };

    let cert_pkey = create_p384_key().unwrap();
    let cert_order = csr_order
        .finalize_pkey(
            cert_pkey,
            Duration::from_millis(MILLISECONDS_UNTIL_ACME_CSR_VALIDATION)
        )
        .unwrap();
    let cert = cert_order.download_cert().unwrap();

    server.handle.stop(true).await;
    match server.thread.await {
        Ok(_) => {}
        Err(e) => {
            panic!("SERVER ERROR: {}", e);
        }
    }

    fs_data_repository.remove_acme_challenge_dir()?;

    let intermediate_cert_str = reqwest::blocking
        ::get(INTERMEDIATE_CERTIFICATE_LETS_ENCRYPT_URL)
        .unwrap()
        .text()
        .unwrap();

    fs_data_repository.write_tls_certificate_pkey_file(&cert.private_key())?;
    fs_data_repository.write_tls_certificate_file(&cert.certificate())?;
    fs_data_repository.write_tls_intermediate_certificate_file(
        &intermediate_cert_str
    )?;

    let cert_str = fs_data_repository.get_tls_certificate_file_string()?;
    let intermediate_cert_str =
        fs_data_repository.get_tls_intermediate_certificate_file_string()?;
    let cert_chain_str = cert_str + "\n" + &intermediate_cert_str;
    fs_data_repository.write_tls_certificate_chain_file(&cert_chain_str)?;

    Ok(())
}

pub fn spawn_renewal_thread(
    handle: Handle
) -> Result<tokio::task::JoinHandle<()>, DomainError> {
    let fs_data_repository = FsDataRepository::init()?;

    let cert_bytes = fs_data_repository.get_tls_certificate_file_bytes()?;
    let cert = X509::from_pem(&cert_bytes).map_err(
        |_e| error::CANNOT_PARSE_TLS_CERTIFICATE
    )?;

    let expiration_time: &Asn1TimeRef = cert.not_after();
    let current_time = Asn1Time::days_from_now(0).unwrap();

    let time_until_expiration = expiration_time.diff(&current_time).unwrap();

    let seconds_until_expiration = time_until_expiration.secs.abs();

    let renewal_thread = tokio::spawn(async move {
        let seconds_until_renewal =
            seconds_until_expiration - MIN_SECONDS_WITHOUT_RENEWAL;
        let more_than_1_hour_until_renewal = seconds_until_renewal > 3600;
        if more_than_1_hour_until_renewal {
            tokio::time::sleep(
                Duration::from_secs(seconds_until_renewal.try_into().unwrap())
            ).await;
        }

        // TODO: Prevent loss of input data while Tukosmo server is off.
        // Currently (2022-09-24), Actix Web doesn't support hot reload
        // of TLS/SSL credentials: https://github.com/actix/actix-net/issues/13
        let _ = handle.stop(true);
    });

    Ok(renewal_thread)
}
