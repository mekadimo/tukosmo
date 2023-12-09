use crate::core::shared::model::CoreSubmoduleName::Language;
use crate::core::shared::model::DomainError;
use crate::core::shared::model::DomainErrorId;
use crate::core::shared::model::DomainErrorVisibility;
use crate::core::shared::model::ModuleName::Core;
use super::model::I18nTextId;

pub const CANNOT_DELETE_LAST_LANGUAGE_LEFT: DomainError = get_domain_error(
    "CANNOT_DELETE_LAST_LANGUAGE_LEFT",
    "Cannot delete last language left.",
    DomainErrorVisibility::Public
);

pub const I18N_TRANSLATION_NOT_FOUND: DomainError = get_domain_error(
    "I18N_TRANSLATION_NOT_FOUND",
    "I18n translation not found.",
    DomainErrorVisibility::Public
);

pub const LANGUAGE_CODE_ALREADY_EXISTS: DomainError = get_domain_error(
    "LANGUAGE_CODE_ALREADY_EXISTS",
    "This language code already exists.",
    DomainErrorVisibility::Public
);

pub const LANGUAGE_NOT_FOUND: DomainError = get_domain_error(
    "LANGUAGE_NOT_FOUND",
    "Language not found.",
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
            module: Core(Language),
        },
        message,
        visibility,
    }
}

fn get_domain_error_with_context(
    error_code: &'static str,
    message: &'static str,
    visibility: DomainErrorVisibility,
    context: Vec<(String, String)>
) -> DomainError {
    DomainError {
        context,
        id: DomainErrorId {
            error_code,
            module: Core(Language),
        },
        message,
        visibility,
    }
}

pub fn i18n_text_not_found(id: &I18nTextId) -> DomainError {
    get_domain_error_with_context(
        "I18N_TEXT_NOT_FOUND",
        "I18n text not found.",
        DomainErrorVisibility::Public,
        vec![("id".to_string(), id.value().to_string())]
    )
}
