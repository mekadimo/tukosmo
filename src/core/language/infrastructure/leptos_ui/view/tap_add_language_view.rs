use leptos::IntoView;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::component;
use leptos::create_rw_signal;
use leptos::spawn_local;
use leptos::tracing;
use leptos::view;
use leptos_router::NavigateOptions;
use leptos_router::use_navigate;
use tukosmo_application::core::language::dto::DtoAddLanguage;
use tukosmo_domain::core::shared::model::ServerResponseError;

use crate::core::language::leptos_actix_server::api::language_api;
use crate::core::language::leptos_ui::LanguageForm;
use crate::core::shared::leptos_ui::GlobalContext;
use crate::core::shared::leptos_ui::TapFormPage;
use crate::core::shared::leptos_ui::TapFormTextField;
use crate::core::shared::leptos_ui::TapLayout;
use crate::core::shared::leptos_ui::service::navigation;
use crate::core::shared::leptos_ui::service::server::manage_response;
use crate::core::shared::leptos_ui::use_global_context;
use crate::t;
use super::super::TapFormI18nTextField;

#[component]
pub fn TapAddLanguageView() -> impl IntoView {
    let global_context = use_global_context();
    let GlobalContext { current_language_reader, .. } = use_global_context();

    let waiting_response_signal = create_rw_signal(false);
    let server_error_signal =
        create_rw_signal::<Option<ServerResponseError>>(None);

    let form = LanguageForm::init_empty();

    let on_click_submit_button = move |_| {
        waiting_response_signal.set(true);
        let current_language_code = current_language_reader.get().code;

        spawn_local(async move {
            let dto_form = form.get_value().get_dto();
            let dto = DtoAddLanguage { form: dto_form };
            let server_response_add = language_api::add(dto).await;

            manage_response(
                server_response_add,
                move |_response| {
                    let current_language_code = current_language_code.clone();
                    spawn_local(async move {
                        let server_response_languages =
                            language_api::get_all_languages().await;
                        manage_response(
                            server_response_languages,
                            move |languages| {
                                let lang_code = current_language_code
                                    .value()
                                    .to_string();
                                global_context.refresh_languages(
                                    Some(lang_code),
                                    languages
                                );
                                let navigate = use_navigate();
                                let path = navigation::path_admin_languages(
                                    &current_language_code
                                );
                                navigate(&path, NavigateOptions::default());
                            },
                            move |server_error| {
                                server_error_signal.set(Some(server_error));
                                waiting_response_signal.set(false);
                            }
                        );
                    });
                },
                move |server_error| {
                    form.get_value().validate(&server_error.error_code);
                    server_error_signal.set(Some(server_error));

                    waiting_response_signal.set(false);
                }
            );
        });
    };

    view! {
        <TapLayout>
            <TapFormPage
                cancel_route_path=Box::new(move || navigation::path_admin_languages(
                    &current_language_reader.get().code
                ))
                on_click_submit_button=Box::new(on_click_submit_button)
                server_error_signal=server_error_signal
                title=Box::new(|| t!(main.add_language)())
                waiting_response_signal=waiting_response_signal
            >
                // TODO
                //<input name="csrf_token" type="hidden" value=csrf_token />

                <TapFormTextField
                    name=Box::new(|| t!(main.code)())
                    required=true
                    value=form.get_value().code
                />

                <TapFormTextField
                    name=Box::new(|| t!(main.original_name)())
                    required=true
                    value=form.get_value().original_name
                />

                <TapFormI18nTextField
                    name=Box::new(|| t!(main.name)())
                    required=true
                    value=form.get_value().name
                />

                <TapFormTextField
                    name=Box::new(|| t!(main.website_title_in_this_language)())
                    required=true
                    value=form.get_value().website_title
                />

                <TapFormTextField
                    name=Box::new(|| t!(main.website_subtitle_in_this_language)())
                    required=true
                    value=form.get_value().website_subtitle
                />
            </TapFormPage>
        </TapLayout>
    }
}
