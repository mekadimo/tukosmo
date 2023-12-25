use leptos::ServerFnError;
use leptos::server;
use tukosmo_application::core::user::dto::DtoAddUser;
use tukosmo_application::core::user::dto::DtoChangeUserPassword;
use tukosmo_application::core::user::dto::DtoEditUser;
use tukosmo_application::core::user::dto::DtoGetUser;
use tukosmo_application::core::user::dto::DtoGetUsersPaginated;
use tukosmo_application::core::user::dto::DtoUsersPaginated;
#[cfg(feature = "ssr")]
use tukosmo_application::core::user::use_case::UserUseCase;
use tukosmo_domain::core::user::model::User;
#[cfg(feature = "ssr")]
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::shared::model::ServerResponse;

#[server(ApiCoreUserUserAdd)]
pub async fn add(dto: DtoAddUser) -> Result<ServerResponse<()>, ServerFnError> {
    let response = ServerResponse::build(|| {
        let user_use_case = common()?;
        user_use_case.add(dto)
    });

    Ok(response)
}

#[server(ApiCoreUserUserChangePassword)]
pub async fn change_password(
    dto: DtoChangeUserPassword
) -> Result<ServerResponse<()>, ServerFnError> {
    let response = ServerResponse::build(|| {
        let user_use_case = common()?;
        user_use_case.change_password(dto)
    });

    Ok(response)
}

#[server(ApiCoreUserUserEdit)]
pub async fn edit(
    dto: DtoEditUser
) -> Result<ServerResponse<()>, ServerFnError> {
    let response = ServerResponse::build(|| {
        let user_use_case = common()?;
        user_use_case.edit(dto)
    });

    Ok(response)
}

#[server(ApiCoreUserUserGet)]
pub async fn get(
    dto: DtoGetUser
) -> Result<ServerResponse<User>, ServerFnError> {
    let response = ServerResponse::build(|| {
        let user_use_case = common()?;
        user_use_case.get(dto)
    });

    Ok(response)
}

#[server(ApiCoreUserUserListPaginated)]
pub async fn list_paginated(
    dto: DtoGetUsersPaginated
) -> Result<ServerResponse<DtoUsersPaginated>, ServerFnError> {
    let response = ServerResponse::build(|| {
        let user_use_case = common()?;
        user_use_case.get_users_paginated(dto)
    });

    Ok(response)
}

#[cfg(feature = "ssr")]
fn common() -> Result<UserUseCase, DomainError> {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::core::shared::diesel_orm::model::DbTransactionExecutor;

    let transaction_executor = DbTransactionExecutor::init()?;
    let user_use_case = UserUseCase::init(
        Rc::new(RefCell::new(transaction_executor))
    );

    Ok(user_use_case)
}
