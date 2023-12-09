use core::time::Duration;
use leptos::AnimatedShow;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::component;
use leptos::tracing;
use leptos::view;

#[component]
pub fn LoadingTopBar(on: ReadSignal<bool>) -> impl IntoView {
    view! {
        <AnimatedShow
            hide_class="core-shared-loading_top_bar-hide"
            hide_delay=Duration::from_millis(1000)
            show_class="core-shared-loading_top_bar-show"
            when=on
        >
            <div class="core-shared-loading_top_bar-bar"></div>
        </AnimatedShow>
    }
}
