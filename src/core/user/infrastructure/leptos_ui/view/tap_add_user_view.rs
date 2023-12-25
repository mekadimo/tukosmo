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
use tukosmo_application::core::user::dto::DtoAddUser;
use tukosmo_domain::core::shared::model::ServerResponseError;

use crate::core::language::leptos_ui::TapFormI18nTextField;
use crate::core::shared::leptos_ui::GlobalContext;
use crate::core::shared::leptos_ui::TapFormCheckboxField;
use crate::core::shared::leptos_ui::TapFormPage;
use crate::core::shared::leptos_ui::TapFormPasswordField;
use crate::core::shared::leptos_ui::TapFormTextField;
use crate::core::shared::leptos_ui::TapLayout;
use crate::core::shared::leptos_ui::service::navigation;
use crate::core::shared::leptos_ui::service::server::manage_response;
use crate::core::shared::leptos_ui::use_global_context;
use crate::core::user::leptos_actix_server::api::user_api;
use crate::core::user::leptos_ui::AddUserForm;
use crate::t;

#[component]
pub fn TapAddUserView() -> impl IntoView {
    let GlobalContext { current_language_reader, .. } = use_global_context();

    let waiting_response_signal = create_rw_signal(false);
    let server_error_signal =
        create_rw_signal::<Option<ServerResponseError>>(None);

    let form = AddUserForm::init_empty();

    let on_click_submit_button = move |_| {
        waiting_response_signal.set(true);
        let current_language_code = current_language_reader.get().code;

        spawn_local(async move {
            let dto_form = form.get_value().get_dto();
            let dto = DtoAddUser { form: dto_form };
            let server_response_add = user_api::add(dto).await;

            manage_response(
                server_response_add,
                move |_response| {
                    let navigate = use_navigate();
                    let path = navigation::path_admin_users(
                        &current_language_code
                    );
                    navigate(&path, NavigateOptions::default());
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
                cancel_route_path=Box::new(move || navigation::path_admin_users(
                    &current_language_reader.get().code
                ))
                on_click_submit_button=Box::new(on_click_submit_button)
                server_error_signal=server_error_signal
                title=Box::new(|| t!(main.create_user)())
                waiting_response_signal=waiting_response_signal
            >
                // TODO
                //<input name="csrf_token" type="hidden" value=csrf_token />

                <TapFormTextField
                    name=Box::new(|| t!(main.email)())
                    required=true
                    value=form.get_value().email
                />

                <TapFormPasswordField
                    name=Box::new(|| t!(main.password)())
                    required=true
                    value=form.get_value().plaintext_password
                />

                <TapFormPasswordField
                    name=Box::new(|| t!(main.password_repeat)())
                    required=true
                    value=form.get_value().plaintext_password_repeated
                />

                <TapFormI18nTextField
                    name=Box::new(|| t!(main.name)())
                    required=true
                    value=form.get_value().name
                />

                <TapFormCheckboxField
                    required=false
                    text=Box::new(|| t!(main.administrator)())
                    value=form.get_value().is_admin
                />

                <TapFormCheckboxField
                    required=false
                    text=Box::new(|| t!(main.suspended_account)())
                    value=form.get_value().is_suspended
                />
            </TapFormPage>
        </TapLayout>
    }
}
