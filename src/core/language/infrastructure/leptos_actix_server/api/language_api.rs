use leptos::ServerFnError;
use leptos::server;
use tukosmo_application::core::language::dto::DtoAddLanguage;
use tukosmo_application::core::language::dto::DtoDeleteLanguage;
use tukosmo_application::core::language::dto::DtoEditLanguage;
use tukosmo_application::core::language::dto::DtoGetLanguage;
use tukosmo_application::core::language::dto::DtoGetLanguagesPaginated;
use tukosmo_application::core::language::dto::DtoLanguagesPaginated;
#[cfg(feature = "ssr")]
use tukosmo_application::core::language::use_case::LanguageUseCase;
use tukosmo_domain::core::language::model::Language;
#[cfg(feature = "ssr")]
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::shared::model::ServerResponse;

#[server(ApiCoreLanguageLanguageAdd)]
pub async fn add(
    dto: DtoAddLanguage
) -> Result<ServerResponse<()>, ServerFnError> {
    let response = ServerResponse::build(|| {
        let language_use_case = common()?;
        language_use_case.add(dto)
    });

    Ok(response)
}

#[server(ApiCoreLanguageLanguageDelete)]
pub async fn delete(
    dto: DtoDeleteLanguage
) -> Result<ServerResponse<()>, ServerFnError> {
    let response = ServerResponse::build(|| {
        let language_use_case = common()?;
        language_use_case.delete(dto)
    });

    Ok(response)
}

#[server(ApiCoreLanguageLanguageEdit)]
pub async fn edit(
    dto: DtoEditLanguage
) -> Result<ServerResponse<()>, ServerFnError> {
    let response = ServerResponse::build(|| {
        let language_use_case = common()?;
        language_use_case.edit(dto)
    });

    Ok(response)
}

#[server(ApiCoreLanguageLanguageGet)]
pub async fn get(
    dto: DtoGetLanguage
) -> Result<ServerResponse<Language>, ServerFnError> {
    let response = ServerResponse::build(|| {
        let language_use_case = common()?;
        language_use_case.get(dto)
    });

    Ok(response)
}

#[server(ApiCoreLanguageLanguageGetAllLanguages)]
pub async fn get_all_languages() -> Result<
    ServerResponse<Vec<Language>>,
    ServerFnError
> {
    let response = ServerResponse::build(|| {
        let language_use_case = common()?;
        language_use_case.get_all_languages()
    });

    Ok(response)
}

#[server(ApiCoreLanguageLanguageListPaginated)]
pub async fn list_paginated(
    dto: DtoGetLanguagesPaginated
) -> Result<ServerResponse<DtoLanguagesPaginated>, ServerFnError> {
    let response = ServerResponse::build(|| {
        let language_use_case = common()?;
        language_use_case.get_languages_paginated(dto)
    });

    Ok(response)
}

#[cfg(feature = "ssr")]
fn common() -> Result<LanguageUseCase, DomainError> {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::core::shared::diesel_orm::model::DbTransactionExecutor;

    let transaction_executor = DbTransactionExecutor::init()?;
    let language_use_case = LanguageUseCase::init(
        Rc::new(RefCell::new(transaction_executor))
    );

    Ok(language_use_case)
}
