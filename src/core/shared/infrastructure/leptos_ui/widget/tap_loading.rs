use leptos::IntoView;
use leptos::ServerFnError;
use leptos::component;
use leptos::store_value;
use leptos::tracing;
use leptos::view;
use tukosmo_domain::core::shared::model::ServerResponseError;

use crate::t_error;

#[component]
pub fn TapLoadingError(error: ServerResponseError) -> impl IntoView {
    let server_error = store_value(error);
    view! {
        <div>
            <h2>
                {server_error.get_value().error_code}
            </h2>
            <p>
                {move || {
                    let server_error = server_error.get_value();
                    t_error!(&server_error.error_code, &server_error.context)()
                }}
            </p>
        </div>
    }
}

#[component]
pub fn TapLoadingLeptosError(error: ServerFnError) -> impl IntoView {
    // TODO: Create function that transforms ServerFnError in DomainError
    // and call TapLoadingError
    match error {
        ServerFnError::Registration(_) => {
            view! {
                <p>"Leptos error"</p>
            }
        }
        ServerFnError::Request(_) => {
            view! {
                <p>"Leptos error"</p>
            }
        }
        ServerFnError::ServerError(_) => {
            view! {
                <p>"Leptos error"</p>
            }
        }
        ServerFnError::Deserialization(_) => {
            view! {
                <p>"Leptos error"</p>
            }
        }
        ServerFnError::Serialization(_) => {
            view! {
                <p>"Leptos error"</p>
            }
        }
        ServerFnError::Args(_) => {
            view! {
                <p>"Leptos error"</p>
            }
        }
        ServerFnError::MissingArg(_) => {
            view! {
                <p>"Leptos error"</p>
            }
        }
    }
}

#[component]
pub fn TapLoadingResource() -> impl IntoView {
    view! {
        // TODO: i18n
        <div>"Loading..."</div>
    }
}
