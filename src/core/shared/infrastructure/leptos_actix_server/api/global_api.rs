use leptos::ServerFnError;
use leptos::server;
use tukosmo_application::core::shared::dto::DtoGetInitialData;
use tukosmo_application::core::shared::dto::DtoGetLocalI18n;
use tukosmo_application::core::shared::dto::DtoInitialData;
#[cfg(feature = "ssr")]
use tukosmo_application::core::shared::use_case::GlobalUseCase;
#[cfg(feature = "ssr")]
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::shared::model::LocalI18n;
use tukosmo_domain::core::shared::model::ServerResponse;

#[cfg(feature = "ssr")]
fn common() -> Result<GlobalUseCase, DomainError> {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::core::shared::leptos_actix_server::repository::FsDataRepository;
    use crate::core::shared::diesel_orm::model::DbTransactionExecutor;

    let data_repository = FsDataRepository::init()?;

    let transaction_executor = DbTransactionExecutor::init()?;
    let global_use_case = GlobalUseCase::init(
        Rc::new(RefCell::new(data_repository)),
        Rc::new(RefCell::new(transaction_executor))
    );

    Ok(global_use_case)
}

#[server(ApiCoreSharedGlobalInitialData)]
pub async fn initial_data(
    dto: DtoGetInitialData
) -> Result<ServerResponse<DtoInitialData>, ServerFnError> {
    let response = ServerResponse::build(|| {
        let global_use_case = common()?;
        global_use_case.get_initial_data(dto)
    });

    Ok(response)
}

#[server(ApiCoreSharedGlobalLocalI18n)]
pub async fn local_i18n(
    dto: DtoGetLocalI18n
) -> Result<ServerResponse<LocalI18n>, ServerFnError> {
    let response = ServerResponse::build(|| {
        let global_use_case = common()?;
        global_use_case.get_local_i18n(dto)
    });

    Ok(response)
}
