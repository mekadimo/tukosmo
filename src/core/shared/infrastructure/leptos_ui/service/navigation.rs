use tukosmo_domain::core::language::model::LanguageCode;
use tukosmo_domain::core::language::model::LanguageId;
use tukosmo_domain::core::user::model::UserId;

pub const CODE_PATH_ADMIN: &'static str = "/:language_code/admin";
pub const CODE_PATH_ADMIN_DASHBOARD: &'static str =
    "/:language_code/admin/dashboard";
pub const CODE_PATH_ADMIN_LANGUAGES: &'static str =
    "/:language_code/admin/languages";
pub const CODE_PATH_ADMIN_LANGUAGES_ADD: &'static str =
    "/:language_code/admin/languages/add";
pub const CODE_PATH_ADMIN_LANGUAGES_DELETE: &'static str =
    "/:language_code/admin/languages/delete/:id";
pub const CODE_PATH_ADMIN_LANGUAGES_EDIT: &'static str =
    "/:language_code/admin/languages/edit/:id";
pub const CODE_PATH_ADMIN_USERS: &'static str = "/:language_code/admin/users";
pub const CODE_PATH_ADMIN_USERS_ADD: &'static str =
    "/:language_code/admin/users/add";
pub const CODE_PATH_ADMIN_USERS_CHANGE_PASSWORD: &'static str =
    "/:language_code/admin/users/change_password/:id";
pub const CODE_PATH_ADMIN_USERS_EDIT: &'static str =
    "/:language_code/admin/users/edit/:id";
pub const CODE_PATH_HOME: &'static str = "/:language_code/";
pub const CODE_PATH_LOGIN: &'static str = "/:language_code/login";
pub const CODE_PATH_LOGOUT: &'static str = "/:language_code/logout";

const PARAM_ID: &'static str = ":id";
const PARAM_LANGUAGE_CODE: &'static str = ":language_code";

pub fn change_uri_language(
    uri_path: &str,
    uri_query: &str,
    new_language_code: &str
) -> String {
    let uri = &(match uri_query {
        "" => uri_path.to_string(),
        _ => format!("{}?{}", uri_path, uri_query),
    });
    let mut new_uri = format!("/{new_language_code}");
    if
        let Some((_, subroute)) = uri
            .match_indices("/")
            .nth(1)
            .map(|(index, _)| uri.split_at(index))
    {
        if !subroute.is_empty() {
            new_uri.push_str(subroute);
        }
    }
    new_uri
}

// TODO: This function is terrible and should be implemented properly
pub fn change_uri_query_param(
    uri_path: &str,
    uri_query: &str,
    query_param_name: &str,
    // TODO: `query_param_value` must be escaped for valid URL!
    query_param_value: &str
) -> String {
    let new_query = match uri_query {
        "" => {
            let new_query = format!(
                "{}={}",
                query_param_name,
                query_param_value
            );
            new_query
        }
        _ => {
            let has_target_query_param = uri_query
                .split('&')
                .find(|param| {
                    let mut parts = param.splitn(2, '=');
                    let name = parts.next().unwrap();
                    name == query_param_name
                })
                .is_some();
            if !has_target_query_param {
                format!(
                    "{}&{}={}",
                    uri_query,
                    query_param_name,
                    query_param_value
                )
            } else {
                let modified_query = uri_query
                    .split('&')
                    .map(|param| {
                        let mut parts = param.splitn(2, '=');
                        let name = parts.next().unwrap();
                        if name == query_param_name {
                            format!("{}={}", name, query_param_value)
                        } else {
                            let value = parts.next();
                            match value {
                                Some(val) => format!("{}={}", name, val),
                                None => format!("{}=", name),
                            }
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("&");
                modified_query
            }
        }
    };

    format!("{}?{}", uri_path, new_query)
}

pub fn get_language_code_from_uri(uri_path: &str) -> Option<String> {
    if
        let Some((uri_prefix, _)) = uri_path
            .match_indices("/")
            .nth(1)
            .map(|(index, _)| uri_path.split_at(index))
    {
        match uri_prefix.to_string().strip_prefix("/") {
            Some(language_code) => Some(language_code.to_string()),
            None => None,
        }
    } else {
        None
    }
}

pub fn path_admin(language_code: &LanguageCode) -> String {
    CODE_PATH_ADMIN.replace(PARAM_LANGUAGE_CODE, language_code.value())
}

pub fn path_admin_dashboard(language_code: &LanguageCode) -> String {
    CODE_PATH_ADMIN_DASHBOARD.replace(
        PARAM_LANGUAGE_CODE,
        language_code.value()
    )
}

pub fn path_admin_languages(language_code: &LanguageCode) -> String {
    CODE_PATH_ADMIN_LANGUAGES.replace(
        PARAM_LANGUAGE_CODE,
        language_code.value()
    )
}

pub fn path_admin_languages_add(language_code: &LanguageCode) -> String {
    CODE_PATH_ADMIN_LANGUAGES_ADD.replace(
        PARAM_LANGUAGE_CODE,
        language_code.value()
    )
}

pub fn path_admin_languages_delete(
    language_code: &LanguageCode,
    language_id: &LanguageId
) -> String {
    CODE_PATH_ADMIN_LANGUAGES_DELETE.replace(
        PARAM_LANGUAGE_CODE,
        language_code.value()
    ).replace(PARAM_ID, &language_id.value().to_string())
}

pub fn path_admin_languages_edit(
    language_code: &LanguageCode,
    language_id: &LanguageId
) -> String {
    CODE_PATH_ADMIN_LANGUAGES_EDIT.replace(
        PARAM_LANGUAGE_CODE,
        language_code.value()
    ).replace(PARAM_ID, &language_id.value().to_string())
}

pub fn path_admin_users(language_code: &LanguageCode) -> String {
    CODE_PATH_ADMIN_USERS.replace(PARAM_LANGUAGE_CODE, language_code.value())
}

pub fn path_admin_users_add(language_code: &LanguageCode) -> String {
    CODE_PATH_ADMIN_USERS_ADD.replace(
        PARAM_LANGUAGE_CODE,
        language_code.value()
    )
}

pub fn path_admin_users_edit(
    language_code: &LanguageCode,
    user_id: &UserId
) -> String {
    CODE_PATH_ADMIN_USERS_EDIT.replace(
        PARAM_LANGUAGE_CODE,
        language_code.value()
    ).replace(PARAM_ID, &user_id.value().to_string())
}

pub fn path_home(language_code: &LanguageCode) -> String {
    CODE_PATH_HOME.replace(PARAM_LANGUAGE_CODE, language_code.value())
}

pub fn path_login(language_code: &LanguageCode) -> String {
    CODE_PATH_LOGIN.replace(PARAM_LANGUAGE_CODE, language_code.value())
}

pub fn path_logout(language_code: &LanguageCode) -> String {
    CODE_PATH_LOGOUT.replace(PARAM_LANGUAGE_CODE, language_code.value())
}
