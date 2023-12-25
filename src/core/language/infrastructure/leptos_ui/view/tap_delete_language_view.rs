use leptos::IntoView;
use leptos::SignalGet;
use leptos::SignalGetUntracked;
use leptos::SignalSet;
use leptos::Transition;
use leptos::component;
use leptos::create_resource;
use leptos::create_rw_signal;
use leptos::spawn_local;
use leptos::store_value;
use leptos::tracing;
use leptos::view;
use leptos_router::NavigateOptions;
use leptos_router::use_navigate;
use leptos_router::use_params_map;
use tukosmo_application::core::language::dto::DtoDeleteLanguage;
use tukosmo_application::core::language::dto::DtoGetLanguage;
use tukosmo_domain::core::language::model::Language;
use tukosmo_domain::core::shared::model::ServerResponse;
use tukosmo_domain::core::shared::model::ServerResponseError;

use crate::core::language::leptos_actix_server::api::language_api;
use crate::core::shared::leptos_ui::DeleteForm;
use crate::core::shared::leptos_ui::GlobalContext;
use crate::core::shared::leptos_ui::TapFormPage;
use crate::core::shared::leptos_ui::TapFormCheckboxField;
use crate::core::shared::leptos_ui::TapLayout;
use crate::core::shared::leptos_ui::TapLoadingError;
use crate::core::shared::leptos_ui::TapLoadingLeptosError;
use crate::core::shared::leptos_ui::TapLoadingResource;
use crate::core::shared::leptos_ui::service::navigation;
use crate::core::shared::leptos_ui::service::server::manage_response;
use crate::core::shared::leptos_ui::use_global_context;
use crate::t;

#[component]
pub fn TapDeleteLanguageView() -> impl IntoView {
    let params = use_params_map();

    let response_data = create_resource(
        move || params.get(),
        move |params| async move {
            let language_id = params.get("id").unwrap();

            let dto = DtoGetLanguage { language_id: language_id.to_string() };
            let result = language_api::get(dto).await;
            result
        }
    );

    let content = move ||
        response_data.get().map(|server_response| {
            match server_response {
                Ok(server_response) =>
                    match server_response {
                        ServerResponse::Response(language) => {
                            view! {
                                <div>
                                    <TapDeleteLanguageViewContent language=language />
                                </div>
                            }
                        }
                        ServerResponse::Error(error) => {
                            view! {
                                <div>
                                    <TapLoadingError error=error />
                                </div>
                            }
                        }
                    }
                Err(error) => {
                    view! {
                        <div>
                            <TapLoadingLeptosError error=error />
                        </div>
                    }
                }
            }
        });

    view! {
        <TapLayout>
            <Transition fallback=move || view! { <TapLoadingResource /> }>
                {content}
            </Transition>
        </TapLayout>
    }
}

#[component]
fn TapDeleteLanguageViewContent(language: Language) -> impl IntoView {
    let global_context = use_global_context();
    let GlobalContext { current_language_reader, .. } = use_global_context();

    let waiting_response_signal = create_rw_signal(false);
    let server_error_signal =
        create_rw_signal::<Option<ServerResponseError>>(None);

    let form = DeleteForm::init_empty();

    let stored_language_id = store_value(language.id);
    let on_click_submit_button = move |_| {
        waiting_response_signal.set(true);
        let current_language_code = current_language_reader.get().code;
        let current_language_id = current_language_reader.get().id;

        spawn_local(async move {
            let dto_form = form.get_value().get_dto();
            let dto = DtoDeleteLanguage {
                form: dto_form,
                language_id: stored_language_id.get_value().value().to_string(),
            };
            let server_response_delete = language_api::delete(dto).await;

            manage_response(
                server_response_delete,
                move |_response| {
                    let current_language_code = current_language_code.clone();
                    let current_language_id = current_language_id.clone();
                    spawn_local(async move {
                        let server_response_languages =
                            language_api::get_all_languages().await;
                        manage_response(
                            server_response_languages,
                            move |languages| {
                                let effective_language_code = if
                                    current_language_id.value() ==
                                    stored_language_id.get_value().value()
                                {
                                    None
                                } else {
                                    Some(current_language_code.clone())
                                };
                                global_context.refresh_languages(
                                    effective_language_code,
                                    languages
                                );

                                let navigate = use_navigate();
                                let path = navigation::path_admin_languages(
                                    &current_language_reader.get_untracked().code
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
                    form.get_value().validate();
                    server_error_signal.set(Some(server_error));

                    waiting_response_signal.set(false);
                }
            );
        });
    };

    view! {
        <TapFormPage
            cancel_route_path=Box::new(move || navigation::path_admin_languages(
                &current_language_reader.get().code
            ))
            on_click_submit_button=Box::new(on_click_submit_button)
            server_error_signal=server_error_signal
            title=Box::new(|| t!(main.delete_language)())
            waiting_response_signal=waiting_response_signal
        >
            // TODO
            //<input name="csrf_token" type="hidden" value=csrf_token />

            <TapFormCheckboxField
                required=true
                text=Box::new(|| t!(main.i_understand_the_consequences_of_performing_this_action)())
                value=form.get_value().requested
            />
        </TapFormPage>
    }
}
