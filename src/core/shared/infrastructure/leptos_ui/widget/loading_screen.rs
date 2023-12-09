use leptos::IntoView;
use leptos::component;
use leptos::tracing;
use leptos::view;

#[component]
pub fn LoadingScreen() -> impl IntoView {
    view! {
        <div class="core-shared-loading_screen-container">
            <div class="core-shared-loading_screen-loader">
                <div class="core-shared-loading_screen-loader--dot"></div>
                <div class="core-shared-loading_screen-loader--dot"></div>
                <div class="core-shared-loading_screen-loader--dot"></div>
                <div class="core-shared-loading_screen-loader--dot"></div>
                <div class="core-shared-loading_screen-loader--dot"></div>
                <div class="core-shared-loading_screen-loader--dot"></div>
                <div class="core-shared-loading_screen-loader--text"></div>
            </div>
        </div>
    }
}
