use super::model::CoreSubmoduleName::Shared;
use super::model::DomainError;
use super::model::DomainErrorId;
use super::model::DomainErrorVisibility;
use super::model::ModuleName::Core;

pub const CANNOT_BEGIN_TRANSACTION: DomainError = get_domain_error(
    "CANNOT_BEGIN_TRANSACTION",
    "Cannot begin transaction.",
    DomainErrorVisibility::Admin
);

pub const CANNOT_COMMIT_TRANSACTION: DomainError = get_domain_error(
    "CANNOT_COMMIT_TRANSACTION",
    "Cannot commit transaction.",
    DomainErrorVisibility::Admin
);

pub const CANNOT_CREATE_DIRECTORY: DomainError = get_domain_error(
    "CANNOT_CREATE_DIRECTORY",
    "Cannot create directory.",
    DomainErrorVisibility::Server
);

pub const CANNOT_ESTABLISH_DATABASE_CONNECTION: DomainError = get_domain_error(
    "CANNOT_ESTABLISH_DATABASE_CONNECTION",
    "Cannot establish connection with the database.",
    DomainErrorVisibility::Server
);

pub const CANNOT_EXECUTE_DELETE_ON_DATABASE: DomainError = get_domain_error(
    "CANNOT_EXECUTE_DELETE_ON_DATABASE",
    "Cannot execute DELETE on database.",
    DomainErrorVisibility::Server
);

pub const CANNOT_EXECUTE_INSERT_ON_DATABASE: DomainError = get_domain_error(
    "CANNOT_EXECUTE_INSERT_ON_DATABASE",
    "Cannot execute INSERT on database.",
    DomainErrorVisibility::Server
);

pub const CANNOT_EXECUTE_SELECT_ON_DATABASE: DomainError = get_domain_error(
    "CANNOT_EXECUTE_SELECT_ON_DATABASE",
    "Cannot execute SELECT on database.",
    DomainErrorVisibility::Server
);

pub const CANNOT_EXECUTE_UPDATE_ON_DATABASE: DomainError = get_domain_error(
    "CANNOT_EXECUTE_UPDATE_ON_DATABASE",
    "Cannot execute UPDATE on database.",
    DomainErrorVisibility::Server
);

pub const CANNOT_GENERATE_TLS_CERTIFICATE: DomainError = get_domain_error(
    "CANNOT_GENERATE_TLS_CERTIFICATE",
    "Cannot generate the TLS certificate.",
    DomainErrorVisibility::Server
);

pub const CANNOT_OBTAIN_DATABASE_CREDENTIALS: DomainError = get_domain_error(
    "CANNOT_OBTAIN_DATABASE_CREDENTIALS",
    "Cannot obtain database credentials from DATABASE_URL env var.",
    DomainErrorVisibility::Server
);

pub const CANNOT_OBTAIN_TUKOSMO_DATA_DIR_ENV_VAR: DomainError =
    get_domain_error(
        "CANNOT_OBTAIN_TUKOSMO_DATA_DIR_ENV_VAR",
        "Cannot obtain TUKOSMO_DATA_DIR env var.",
        DomainErrorVisibility::Server
    );

pub const CANNOT_PARSE_TLS_CERTIFICATE: DomainError = get_domain_error(
    "CANNOT_PARSE_TLS_CERTIFICATE",
    "Cannot parse the TLS certificate.",
    DomainErrorVisibility::Server
);

pub const CANNOT_PARSE_TLS_CERTIFICATE_PKEY: DomainError = get_domain_error(
    "CANNOT_PARSE_TLS_CERTIFICATE_PKEY",
    "Cannot parse the TLS certificate private key.",
    DomainErrorVisibility::Server
);

pub const CANNOT_PARSE_TOML_FILE: DomainError = get_domain_error(
    "CANNOT_PARSE_TOML_FILE",
    "Cannot parse the TOML file.",
    DomainErrorVisibility::Server
);

pub const CANNOT_READ_FILE: DomainError = get_domain_error(
    "CANNOT_READ_FILE",
    "Cannot read file.",
    DomainErrorVisibility::Server
);

pub const CANNOT_READ_PATH_FS_METADATA: DomainError = get_domain_error(
    "CANNOT_READ_PATH_FS_METADATA",
    "Cannot read path file system metadata.",
    DomainErrorVisibility::Server
);

pub const CANNOT_REMOVE_DIRECTORY: DomainError = get_domain_error(
    "CANNOT_REMOVE_DIRECTORY",
    "Cannot remove directory.",
    DomainErrorVisibility::Server
);

pub const CANNOT_ROLLBACK_TRANSACTION: DomainError = get_domain_error(
    "CANNOT_ROLLBACK_TRANSACTION",
    "Cannot rollback transaction.",
    DomainErrorVisibility::Admin
);

pub const CANNOT_WRITE_FILE: DomainError = get_domain_error(
    "CANNOT_WRITE_FILE",
    "Cannot write file.",
    DomainErrorVisibility::Server
);

pub const DATA_DIR_DOES_NOT_EXIST: DomainError = get_domain_error(
    "DATA_DIR_DOES_NOT_EXIST",
    "The provided data directory does not exist.",
    DomainErrorVisibility::Server
);

pub const DIRECTORY_ALREADY_EXISTS: DomainError = get_domain_error(
    "DIRECTORY_ALREADY_EXISTS",
    "The directory already exists.",
    DomainErrorVisibility::Server
);

pub const DIRECTORY_DOES_NOT_EXIST: DomainError = get_domain_error(
    "DIRECTORY_DOES_NOT_EXIST",
    "The directory does not exist.",
    DomainErrorVisibility::Server
);

pub const FIELD_CANNOT_BE_EMPTY: DomainError = get_domain_error(
    "FIELD_CANNOT_BE_EMPTY",
    "This field cannot be empty.",
    DomainErrorVisibility::Public
);

pub const INVALID_TOML_EXTENSION: DomainError = get_domain_error(
    "INVALID_TOML_EXTENSION",
    "File doesn't have a valid TOML extension.",
    DomainErrorVisibility::Server
);

pub const INVALID_UUID: DomainError = get_domain_error(
    "INVALID_UUID",
    "This text is not a valid UUID.",
    DomainErrorVisibility::Public
);

pub const NOTHING_TO_DELETE_ON_DATABASE: DomainError = get_domain_error(
    "NOTHING_TO_DELETE_ON_DATABASE",
    "The executed DELETE didn't remove any row.",
    DomainErrorVisibility::Server
);

pub const NOTHING_TO_UPDATE_ON_DATABASE: DomainError = get_domain_error(
    "NOTHING_TO_UPDATE_ON_DATABASE",
    "The executed UPDATE didn't update any row.",
    DomainErrorVisibility::Server
);

pub const PATH_NOT_POINTING_TO_DIRECTORY: DomainError = get_domain_error(
    "PATH_NOT_POINTING_TO_DIRECTORY",
    "The path does not point to a directory.",
    DomainErrorVisibility::Server
);

pub const PATH_NOT_POINTING_TO_FILE: DomainError = get_domain_error(
    "PATH_NOT_POINTING_TO_FILE",
    "The path does not point to a file.",
    DomainErrorVisibility::Server
);

pub const TEXT_DOESNT_REACH_MIN_LENGTH: DomainError = get_domain_error(
    "TEXT_DOESNT_REACH_MIN_LENGTH",
    "The text doesn't reach the min length.",
    DomainErrorVisibility::Public
);

pub const TEXT_EXCEEDS_MAX_LENGTH: DomainError = get_domain_error(
    "TEXT_EXCEEDS_MAX_LENGTH",
    "The text exceeds the max length.",
    DomainErrorVisibility::Public
);

pub const UNDESIRED_DELETES_ON_DATABASE: DomainError = get_domain_error(
    "UNDESIRED_DELETES_ON_DATABASE",
    "The executed DELETE tried to remove more than one row.",
    DomainErrorVisibility::Server
);

pub const UNDESIRED_UPDATES_ON_DATABASE: DomainError = get_domain_error(
    "UNDESIRED_UPDATES_ON_DATABASE",
    "The executed UPDATE tried to change more than one row.",
    DomainErrorVisibility::Server
);

const fn get_domain_error(
    error_code: &'static str,
    message: &'static str,
    visibility: DomainErrorVisibility
) -> DomainError {
    DomainError {
        context: vec![],
        id: DomainErrorId {
            error_code,
            module: Core(Shared),
        },
        message,
        visibility,
    }
}
