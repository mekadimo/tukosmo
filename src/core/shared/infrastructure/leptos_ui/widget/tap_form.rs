use leptos::Children;
#[cfg(feature = "ssr")]
use leptos::IntoAttribute;
#[cfg(feature = "ssr")]
use leptos::IntoClass;
use leptos::IntoView;
use leptos::RwSignal;
use leptos::Show;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::StoredValue;
use leptos::component;
use leptos::event_target_value;
use leptos::tracing;
use leptos::view;
use tukosmo_domain::core::shared::model::ServerResponseError;
use web_sys::MouseEvent;

use crate::core::shared::leptos_ui::FormFieldValue;
use crate::t;
use crate::t_error;

#[component]
pub fn TapFormCheckboxField(
    required: bool,
    text: Box<dyn Fn() -> String>,
    value: StoredValue<FormFieldValue<bool>>
) -> impl IntoView {
    view! {
        <div class="field">
            <label
                class="checkbox"
                class=(
                    "is-danger",
                    move || value.get_value().has_error(),
                )
            >
                <input
                    on:change=move |_event| {
                        value.get_value().update(|draft| {
                            *draft = !*draft;
                        });
                    }
                    prop:checked=value.get_value().signal
                    type="checkbox"
                />
                {move || text()}
                <Show when=move || required>
                    "*"
                </Show>
            </label>
            <Show when=move || value.get_value().has_error()>
                <p class="help is-danger">
                    {move || {
                        let domain_error = value.get_value().get_validation_error();
                        match domain_error {
                            Some(domain_error) => {
                                let full_code = domain_error.get_full_code();
                                t_error!(&full_code, &domain_error.context)()
                            },
                            None => "".to_string(),
                        }
                    }}
                </p>
            </Show>
        </div>
    }
}

#[component]
pub fn TapFormPage(
    cancel_route_path: Box<dyn Fn() -> String>,
    children: Children,
    on_click_submit_button: Box<dyn Fn(MouseEvent)>,
    server_error_signal: RwSignal<Option<ServerResponseError>>,
    title: Box<dyn Fn() -> String>,
    waiting_response_signal: RwSignal<bool>,
    #[prop(optional_no_strip)] delete_route_path: Option<
        Box<dyn Fn() -> String>
    >
) -> impl IntoView {
    let delete_button_included = *&delete_route_path.is_none();

    view! {
        <div class="box is-marginless mb-6">
            <h1 class="title">
                {move || title()}

                <a
                    class="button is-danger is-pulled-right has-text-weight-normal mr-4"
                    // TODO: This should be done with <Show>, but couldn't
                    // figure out how to do that without compiler errors...
                    class=("is-hidden", delete_button_included)
                    href=move || {
                        if let Some(delete_route_path) = &delete_route_path {
                            delete_route_path()
                        } else {
                            "".to_string()
                        }
                    }
                >
                    {t!(main.delete)}
                </a>
            </h1>

            <div class="tap-form">
                {children()}

                <Show when=move || server_error_signal.get().is_some()>
                    <div class="notification is-danger">
                        <button
                            class="delete"
                            on:click=move |_| server_error_signal.set(None)
                        ></button>
                        {move || {
                            let server_error = server_error_signal.get();
                            match server_error {
                                Some(server_error) => {
                                    t_error!(&server_error.error_code, &server_error.context)()
                                },
                                None => "".to_string(),
                            }
                        }}
                    </div>
                </Show>

                <div class="field is-grouped">
                    <div class="control">
                        <button
                            class="button is-link"
                            class=("is-loading", waiting_response_signal)
                            on:click=on_click_submit_button
                        >
                            {t!(main.submit)}
                        </button>
                    </div>
                    <div class="control">
                        <a
                            class="button is-link is-light"
                            href=move || cancel_route_path()
                        >
                            {t!(main.cancel)}
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn TapFormTextField(
    name: Box<dyn Fn() -> String>,
    required: bool,
    value: StoredValue<FormFieldValue<String>>
) -> impl IntoView {
    view! {
        <div class="field">
            <label class="label">
                {move || name()}
                <Show when=move || required>
                    "*"
                </Show>
            </label>
            <p class="control">
                <input
                    class="input"
                    class=(
                        "is-danger",
                        move || value.get_value().has_error(),
                    )
                    on:input=move |event| {
                        value.get_value().set(event_target_value(&event));
                    }
                    prop:value=value.get_value().signal
                    type="text"
                />
            </p>
            <Show when=move || value.get_value().has_error()>
                <p class="help is-danger">
                    {move || {
                        let domain_error = value.get_value().get_validation_error();
                        match domain_error {
                            Some(domain_error) => {
                                let full_code = domain_error.get_full_code();
                                t_error!(&full_code, &domain_error.context)()
                            },
                            None => "".to_string(),
                        }
                    }}
                </p>
            </Show>
        </div>
    }
}
