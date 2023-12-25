use serde::Deserialize;
use serde::Serialize;
use tukosmo_domain::core::language::model::I18nTextValue;
use tukosmo_domain::core::user::model::User;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoAddUser {
    pub form: DtoAddUserForm,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoAddUserForm {
    pub email: String,
    pub is_admin: bool,
    pub is_suspended: bool,
    pub name: I18nTextValue,
    pub plaintext_password: String,
    pub plaintext_password_repeated: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoChangeUserPassword {
    pub form: DtoChangeUserPasswordForm,
    pub user_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoChangeUserPasswordForm {
    pub plaintext_new_password: String,
    pub plaintext_new_password_repeated: String,
    pub plaintext_old_password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoEditUser {
    pub form: DtoEditUserForm,
    pub user_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoEditUserForm {
    pub email: String,
    pub is_admin: bool,
    pub is_suspended: bool,
    pub name: I18nTextValue,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoGetUser {
    pub user_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoGetUsersPaginated {
    pub current_page: i64,
    pub results_per_page: i64,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DtoUsersPaginated {
    pub total_results: i64,
    pub users: Vec<User>,
}
