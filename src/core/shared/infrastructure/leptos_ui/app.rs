use leptos::IntoView;
use leptos::SignalGet;
use leptos::Transition;
use leptos::component;
use leptos::create_resource;
use leptos::provide_context;
use leptos::tracing;
use leptos::view;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_meta::provide_meta_context;
use leptos_router::Outlet;
use leptos_router::Route;
use leptos_router::Router;
use leptos_router::Routes;
use leptos_router::Location;
use leptos_router::use_location;
use tukosmo_application::core::shared::dto::DtoGetInitialData;
use tukosmo_application::core::shared::dto::DtoInitialData;

use crate::core::language::leptos_ui::TapAddLanguageView;
use crate::core::language::leptos_ui::TapDeleteLanguageView;
use crate::core::language::leptos_ui::TapEditLanguageView;
use crate::core::language::leptos_ui::TapLanguagesView;
use crate::core::shared::leptos_actix_server::api::global_api;
use crate::core::shared::leptos_ui::context::GlobalContext;
use crate::core::shared::leptos_ui::widget::LoadingScreen;
use crate::core::user::leptos_ui::WebLoginView;
use super::RootView;
use super::TapDashboardView;
use super::WebHomeView;
use super::WebNotFoundView;
use super::service::navigation;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/site.css" />

        <Title text="Tukosmo" />

        <Router>
            <Routes>
                <Route path="" view=InitialDataLoader>
                    <Route path="" view=RootView />
                    <Route
                        path=navigation::CODE_PATH_ADMIN
                        view=TapDashboardView
                    />
                    <Route
                        path=navigation::CODE_PATH_ADMIN_DASHBOARD
                        view=TapDashboardView
                    />
                    <Route
                        path=navigation::CODE_PATH_ADMIN_LANGUAGES
                        view=TapLanguagesView
                    />
                    <Route
                        path=navigation::CODE_PATH_ADMIN_LANGUAGES_ADD
                        view=TapAddLanguageView
                    />
                    <Route
                        path=navigation::CODE_PATH_ADMIN_LANGUAGES_DELETE
                        view=TapDeleteLanguageView
                    />
                    <Route
                        path=navigation::CODE_PATH_ADMIN_LANGUAGES_EDIT
                        view=TapEditLanguageView
                    />
                    <Route
                        path=navigation::CODE_PATH_LOGIN
                        view=WebLoginView
                    />
                    <Route
                        path=navigation::CODE_PATH_LOGOUT
                        view=|| view! { "logout" } // TODO
                    />
                    <Route path=navigation::CODE_PATH_HOME view=WebHomeView />
                    <Route path="/*any" view=WebNotFoundView />
                </Route>
            </Routes>
        </Router>
    }
}

#[component]
fn InitialDataLoader() -> impl IntoView {
    let Location { pathname: current_uri_path, .. } = use_location();

    let response_data = create_resource(
        || (),
        move |_| async move {
            let uri_path = current_uri_path.get();
            let language_code = match
                navigation::get_language_code_from_uri(&uri_path)
            {
                Some(language_code) => language_code,
                // TODO: Take this value from the default config in the database
                None => "en".to_string(),
            };
            let dto = DtoGetInitialData { language_code };

            // TODO: Return DomainError
            let global_dto = global_api::initial_data(dto).await.unwrap();
            global_dto
        }
    );

    let content = move ||
        response_data
            .get()
            .map(|DtoInitialData { language_code, languages, local_i18n }| {
                provide_context(
                    GlobalContext::init(&language_code, languages, local_i18n)
                );

                view! { <Outlet/> }
            });

    view! {
        <Transition fallback=move || view! { <LoadingScreen /> }>
            {content}
        </Transition>
    }
}
