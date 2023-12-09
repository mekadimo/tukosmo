// @generated automatically by Diesel CLI.

diesel::table! {
    i18n_text (id) {
        id -> Uuid,
        default_text -> Text,
        creation_date -> Timestamptz,
        update_date -> Timestamptz,
    }
}

diesel::table! {
    i18n_translation (id) {
        id -> Uuid,
        i18n_text_id -> Uuid,
        language_id -> Uuid,
        text -> Text,
        creation_date -> Timestamptz,
        update_date -> Timestamptz,
    }
}

diesel::table! {
    language (id) {
        id -> Uuid,
        code -> Text,
        original_name -> Text,
        i18n_text_id_name -> Uuid,
        website_title -> Text,
        website_subtitle -> Text,
        creation_date -> Timestamptz,
        update_date -> Timestamptz,
    }
}

diesel::joinable!(i18n_translation -> i18n_text (i18n_text_id));
diesel::joinable!(i18n_translation -> language (language_id));
diesel::joinable!(language -> i18n_text (i18n_text_id_name));

diesel::allow_tables_to_appear_in_same_query!(
    i18n_text,
    i18n_translation,
    language,
);
