use crate::core::shared::model::CoreSubmoduleName::User;
use crate::core::shared::model::DomainError;
use crate::core::shared::model::DomainErrorId;
use crate::core::shared::model::DomainErrorVisibility;
use crate::core::shared::model::ModuleName::Core;

pub const CANNOT_DELETE_LAST_ADMIN_USER_LEFT: DomainError = get_domain_error(
    "CANNOT_DELETE_LAST_ADMIN_USER_LEFT",
    "Cannot delete last admin user left.",
    DomainErrorVisibility::Public
);

pub const USER_NOT_FOUND: DomainError = get_domain_error(
    "USER_NOT_FOUND",
    "User not found.",
    DomainErrorVisibility::Public
);

pub const USER_PASSWORD_NOT_CORRECT: DomainError = get_domain_error(
    "USER_PASSWORD_NOT_CORRECT",
    "User password not correct.",
    DomainErrorVisibility::Public
);

pub const USER_SESSION_NOT_FOUND: DomainError = get_domain_error(
    "USER_SESSION_NOT_FOUND",
    "User session not found.",
    DomainErrorVisibility::Public
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
            module: Core(User),
        },
        message,
        visibility,
    }
}
