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
use tukosmo_domain::core::shared::model::ServerResponse;

use crate::core::language::leptos_ui::TapAddLanguageView;
use crate::core::language::leptos_ui::TapDeleteLanguageView;
use crate::core::language::leptos_ui::TapEditLanguageView;
use crate::core::language::leptos_ui::TapLanguagesView;
use crate::core::shared::leptos_actix_server::api::global_api;
use crate::core::shared::leptos_ui::context::GlobalContext;
use crate::core::shared::leptos_ui::widget::LoadingScreen;
use crate::core::user::leptos_ui::TapAddUserView;
use crate::core::user::leptos_ui::TapEditUserView;
use crate::core::user::leptos_ui::TapUsersView;
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
                        path=navigation::CODE_PATH_ADMIN_USERS
                        view=TapUsersView
                    />
                    <Route
                        path=navigation::CODE_PATH_ADMIN_USERS_ADD
                        view=TapAddUserView
                    />
                    <Route
                        path=navigation::CODE_PATH_ADMIN_USERS_EDIT
                        view=TapEditUserView
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
            let language_code = navigation::get_language_code_from_uri(
                &uri_path
            );

            let dto = DtoGetInitialData { language_code };
            let result = global_api::initial_data(dto).await;
            result
        }
    );

    let content = move ||
        response_data.get().map(|server_response| {
            // TODO: Look for a better way of managing this error (no unwrap)
            match server_response.unwrap() {
                ServerResponse::Response(dto_initial_data) => {
                    provide_context(
                        GlobalContext::init(
                            &dto_initial_data.language_code,
                            dto_initial_data.languages,
                            dto_initial_data.local_i18n
                        )
                    );

                    view! { <Outlet/> }
                }
                ServerResponse::Error(_e) => {
                    // TODO: Look for a better way of managing this error
                    panic!("Error during initialization.");
                }
            }
        });

    view! {
        <Transition fallback=move || view! { <LoadingScreen /> }>
            {content}
        </Transition>
    }
}
