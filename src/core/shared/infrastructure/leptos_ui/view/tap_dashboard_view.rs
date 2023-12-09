use leptos::IntoView;
use leptos::component;
use leptos::tracing;
use leptos::view;

use crate::t;
use super::super::TapLayout;

#[component]
pub fn TapDashboardView() -> impl IntoView {
    view! {
        <TapLayout>
            <h1>{t!(main.dashboard)}</h1>
        </TapLayout>
    }
}
