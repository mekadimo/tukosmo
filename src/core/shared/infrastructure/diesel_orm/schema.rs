// @generated automatically by Diesel CLI.

diesel::table! {
    i18n_text (id) {
        creation_date -> Timestamptz,
        default_text -> Text,
        id -> Uuid,
        update_date -> Timestamptz,
    }
}

diesel::table! {
    i18n_translation (id) {
        creation_date -> Timestamptz,
        i18n_text_id -> Uuid,
        id -> Uuid,
        language_id -> Uuid,
        text -> Text,
        update_date -> Timestamptz,
    }
}

diesel::table! {
    language (id) {
        code -> Text,
        creation_date -> Timestamptz,
        i18n_text_id_name -> Uuid,
        id -> Uuid,
        original_name -> Text,
        update_date -> Timestamptz,
        website_subtitle -> Text,
        website_title -> Text,
    }
}

diesel::table! {
    user_ (id) {
        creation_date -> Timestamptz,
        email -> Text,
        encrypted_password -> Text,
        i18n_text_id_name -> Uuid,
        id -> Uuid,
        is_admin -> Bool,
        is_suspended -> Bool,
        update_date -> Timestamptz,
    }
}

diesel::table! {
    user_session (id) {
        creation_date -> Timestamptz,
        csrf_token -> Uuid,
        id -> Uuid,
        ip -> Text,
        last_request_date -> Timestamptz,
        user_agent_request_header -> Text,
        user_id -> Uuid,
    }
}

diesel::joinable!(i18n_translation -> i18n_text (i18n_text_id));
diesel::joinable!(i18n_translation -> language (language_id));
diesel::joinable!(language -> i18n_text (i18n_text_id_name));
diesel::joinable!(user_ -> i18n_text (i18n_text_id_name));
diesel::joinable!(user_session -> user_ (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    i18n_text,
    i18n_translation,
    language,
    user_,
    user_session
);
