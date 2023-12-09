use leptos::IntoView;
use leptos::component;
use leptos::tracing;
use leptos::view;

use crate::core::shared::leptos_ui::WebLayout;

#[component]
pub fn WebLoginView() -> impl IntoView {
    view! {
        <WebLayout>
            <h1>"Login page"</h1>
            <p>"Login form here!"</p>
        </WebLayout>
    }
}
