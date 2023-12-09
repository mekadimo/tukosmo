use leptos::IntoView;
use leptos::component;
use leptos::tracing;
use leptos::view;

#[component]
pub fn WebNotFoundView() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        use actix_web::http::StatusCode;
        use leptos::expect_context;
        use leptos_actix::ResponseOptions;

        let response_options = expect_context::<ResponseOptions>();
        response_options.set_status(StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
