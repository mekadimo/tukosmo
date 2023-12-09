use std::fs;
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::shared::error;

pub fn check_directory_exists(
    directory_path: &str
) -> Result<bool, DomainError> {
    match fs::metadata(directory_path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(true)
            } else {
                Err(error::PATH_NOT_POINTING_TO_DIRECTORY)
            }
        }
        Err(e) =>
            match e.kind() {
                std::io::ErrorKind::NotFound => Ok(false),
                _ => Err(error::CANNOT_READ_PATH_FS_METADATA),
            }
    }
}

pub fn check_file_exists(file_path: &str) -> Result<bool, DomainError> {
    match fs::metadata(file_path) {
        Ok(metadata) => {
            if metadata.is_file() {
                Ok(true)
            } else {
                Err(error::PATH_NOT_POINTING_TO_FILE)
            }
        }
        Err(e) =>
            match e.kind() {
                std::io::ErrorKind::NotFound => Ok(false),
                _ => Err(error::CANNOT_READ_PATH_FS_METADATA),
            }
    }
}

pub fn create_directory(directory_path: &str) -> Result<(), DomainError> {
    let directory_exists = check_directory_exists(directory_path)?;
    if directory_exists {
        return Err(error::DIRECTORY_ALREADY_EXISTS);
    }

    fs
        ::create_dir(directory_path)
        .map_err(|_e| error::CANNOT_CREATE_DIRECTORY)?;
    Ok(())
}

pub fn read_file(file_path: &str) -> Result<fs::File, DomainError> {
    let file = fs::File::open(file_path).map_err(|_e| error::CANNOT_READ_FILE)?;
    Ok(file)
}

pub fn read_file_as_string(file_path: &str) -> Result<String, DomainError> {
    let file_str = fs
        ::read_to_string(file_path)
        .map_err(|_e| error::CANNOT_READ_FILE)?;
    Ok(file_str)
}

pub fn read_file_as_vec_u8(file_path: &str) -> Result<Vec<u8>, DomainError> {
    let file_bytes = fs::read(file_path).map_err(|_e| error::CANNOT_READ_FILE)?;
    Ok(file_bytes)
}

pub fn remove_directory(directory_path: &str) -> Result<(), DomainError> {
    let directory_exists = check_directory_exists(directory_path)?;
    if !directory_exists {
        return Err(error::DIRECTORY_DOES_NOT_EXIST);
    }

    fs
        ::remove_dir_all(directory_path)
        .map_err(|_e| error::CANNOT_REMOVE_DIRECTORY)?;
    Ok(())
}

pub fn write_file(
    file_path: &str,
    file_content: &str
) -> Result<(), DomainError> {
    fs::write(file_path, file_content).map_err(|_e| error::CANNOT_WRITE_FILE)?;
    Ok(())
}
