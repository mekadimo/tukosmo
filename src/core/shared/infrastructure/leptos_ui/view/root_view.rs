use leptos::IntoView;
use leptos::SignalGetUntracked;
use leptos::component;
use leptos::tracing;
use leptos::view;
use leptos_router::Redirect;

use super::super::GlobalContext;
use super::super::use_global_context;

#[component]
pub fn RootView() -> impl IntoView {
    let GlobalContext { current_language_reader, .. } = use_global_context();

    view! {
        <Redirect
            path=format!(
                "/{}",
                current_language_reader.get_untracked().code.value(),
            )
        />
    }
}
