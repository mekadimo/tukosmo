use leptos::ChildrenFn;
use leptos::IntoView;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::component;
use leptos::create_effect;
use leptos::create_resource;
use leptos::tracing;
use leptos::view;
use leptos_meta::Html;
use leptos_router::use_params_map;
use tukosmo_application::core::shared::dto::DtoGetLocalI18n;
use tukosmo_domain::core::shared::model::ServerResponse;

use crate::core::shared::leptos_actix_server::api::global_api;
use super::super::context::GlobalContext;
use super::super::context::use_global_context;
use super::super::widget::LoadingTopBar;

#[component]
pub fn I18nLayout(children: ChildrenFn) -> impl IntoView {
    let global_context = use_global_context();
    let GlobalContext {
        current_language_reader,
        languages_reader,
        loading_top_bar_enabled_reader,
        loading_top_bar_enabled_writer,
        ..
    } = use_global_context();

    let params = use_params_map();

    let changed_language_data = create_resource(
        move || (
            params.get(),
            current_language_reader.get(),
            languages_reader.get(),
        ),
        move |(params, current_language, languages)| async move {
            let param_language_code = params.get("language_code").unwrap();
            if param_language_code == current_language.code.value() {
                return;
            }

            let new_current_language = languages
                .iter()
                .find(|l| l.code.value() == param_language_code)
                // TODO: Raise 404 error if language_code is not in the list
                .unwrap()
                .clone();

            let dto = DtoGetLocalI18n {
                language_code: param_language_code.to_string(),
            };
            let result = global_api::local_i18n(dto).await;
            // TODO: Look for a better way of managing this error (no unwrap)
            match result.unwrap() {
                ServerResponse::Response(local_i18n) => {
                    global_context.change_language(
                        new_current_language,
                        local_i18n
                    );
                }
                ServerResponse::Error(_e) => {
                    // TODO: Look for a better way of managing this error
                    // (maybe launching error notification?)
                    panic!("Error loading new language.");
                }
            }
        }
    );

    create_effect(move |_| {
        // TODO: The loading top bar never shows up; it should be visible while:
        // - Navigating to a different page
        // - Changing current page's language
        // - Sending form, etc.
        loading_top_bar_enabled_writer.set(
            changed_language_data.get().is_none()
        );
    });

    view! {
        <Html lang=move || current_language_reader.get().code.value().to_string() />
        // TODO: Web and TAP should have different loading bars
        // (separate components and styles)
        <LoadingTopBar on=loading_top_bar_enabled_reader />
        {children()}
    }
}
