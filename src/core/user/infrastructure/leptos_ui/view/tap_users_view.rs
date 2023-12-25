use leptos::For;
#[cfg(feature = "ssr")]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::Show;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::Transition;
use leptos::component;
use leptos::create_resource;
use leptos::create_signal;
use leptos::store_value;
use leptos::tracing;
use leptos::view;
use leptos_router::use_query_map;
use tukosmo_application::core::user::dto::DtoGetUsersPaginated;
use tukosmo_domain::core::user::model::User;
use tukosmo_domain::core::shared::model::ServerResponse;

use crate::core::shared::leptos_ui::GlobalContext;
use crate::core::shared::leptos_ui::TapLayout;
use crate::core::shared::leptos_ui::TapLoadingError;
use crate::core::shared::leptos_ui::TapLoadingLeptosError;
use crate::core::shared::leptos_ui::TapLoadingResource;
use crate::core::shared::leptos_ui::TapPaginationButtons;
use crate::core::shared::leptos_ui::service::navigation;
use crate::core::shared::leptos_ui::use_global_context;
use crate::core::user::leptos_actix_server::api::user_api;
use crate::t;
use crate::t_date_short;

const DEFAULT_RESULTS_PER_PAGE: i64 = 20;

#[component]
pub fn TapUsersView() -> impl IntoView {
    let url_query_map = use_query_map();

    let (users_reader, users_writer) = create_signal::<Vec<User>>(vec![]);
    let (total_results_reader, total_results_writer) = create_signal::<i64>(0);
    let (
        total_results_in_current_page_reader,
        total_results_in_current_page_writer,
    ) = create_signal::<i64>(0);
    let (total_pages_reader, total_pages_writer) = create_signal::<i64>(1);

    let response_data = create_resource(
        move || url_query_map.get(),
        move |url_query_map| async move {
            let current_page = match url_query_map.get("p") {
                Some(p) => p.parse::<i64>().unwrap_or(1),
                None => 1,
            };
            let results_per_page = match url_query_map.get("rpp") {
                Some(rpp) =>
                    rpp.parse::<i64>().unwrap_or(DEFAULT_RESULTS_PER_PAGE),
                None => DEFAULT_RESULTS_PER_PAGE,
            };

            let dto = DtoGetUsersPaginated {
                current_page,
                results_per_page,
            };
            let result = user_api::list_paginated(dto).await;
            result
        }
    );

    let results_per_page = move || {
        match url_query_map.get().get("rpp") {
            Some(rpp) => rpp.parse::<i64>().unwrap_or(DEFAULT_RESULTS_PER_PAGE),
            None => DEFAULT_RESULTS_PER_PAGE,
        }
    };

    let content = move ||
        response_data.get().map(|server_response| {
            match server_response {
                Ok(server_response) =>
                    match server_response {
                        ServerResponse::Response(dto_users_paginated) => {
                            total_results_in_current_page_writer.set(
                                dto_users_paginated.users
                                    .len()
                                    .try_into()
                                    .unwrap()
                            );
                            users_writer.set(dto_users_paginated.users);
                            let total_pages = if
                                dto_users_paginated.total_results %
                                    results_per_page() == 0
                            {
                                dto_users_paginated.total_results /
                                    results_per_page()
                            } else {
                                dto_users_paginated.total_results /
                                    results_per_page() +
                                    1
                            };
                            total_pages_writer.set(total_pages);
                            total_results_writer.set(
                                dto_users_paginated.total_results
                            );
                            view! {
                                <div>
                                    <TapUsersViewContent
                                        users_reader=users_reader
                                        total_pages_reader=total_pages_reader
                                        total_results_in_current_page_reader=total_results_in_current_page_reader
                                        total_results_reader=total_results_reader
                                    />
                                </div>
                            }
                        }
                        ServerResponse::Error(error) => {
                            view! {
                                <div>
                                    <TapLoadingError error=error />
                                </div>
                            }
                        }
                    }
                Err(error) => {
                    view! {
                        <div>
                            <TapLoadingLeptosError error=error />
                        </div>
                    }
                }
            }
        });

    view! {
        <TapLayout>
            <Transition fallback=move || view! { <TapLoadingResource /> }>
                {content}
            </Transition>
        </TapLayout>
    }
}

#[component]
fn TapUsersViewContent(
    users_reader: ReadSignal<Vec<User>>,
    total_pages_reader: ReadSignal<i64>,
    total_results_in_current_page_reader: ReadSignal<i64>,
    total_results_reader: ReadSignal<i64>
) -> impl IntoView {
    let GlobalContext { current_language_reader, .. } = use_global_context();
    let url_query_map = use_query_map();
    let current_page = move || {
        match url_query_map.get().get("p") {
            Some(p) => p.parse::<i64>().unwrap_or(1),
            None => 1,
        }
    };

    let pagination_is_visible = move || total_pages_reader.get() > 1;

    let view_table_row = move |user: User| {
        let user = store_value(user);
        view! {
            <tr>
                <td>
                    <a
                        href=move || navigation::path_admin_users_edit(
                            &current_language_reader.get().code,
                            &user.get_value().id
                        )
                    >
                        {move || user.get_value().email.value().to_string()}
                    </a>
                </td>
                <td>
                    <a
                        href=move || navigation::path_admin_users_edit(
                            &current_language_reader.get().code,
                            &user.get_value().id
                        )
                    >
                        {move || user.get_value().name.translate(current_language_reader.get().id)}
                    </a>
                </td>
                <td>
                    {move || t_date_short!(user.get_value().creation_date.value())()}
                </td>
                <td>
                    {move || t_date_short!(user.get_value().update_date.value())()}
                </td>
            </tr>
        }
    };

    view! {
        <div class="box is-marginless mb-6">
            <h1 class="title">
                {t!(main.users)}

                <a
                    class="button is-link is-pulled-right has-text-weight-normal mr-4"
                    href=move || navigation::path_admin_users_add(
                        &current_language_reader.get().code
                    )
                >
                    {t!(main.create_user)}
                </a>
            </h1>

            <Show when=pagination_is_visible>
                <h2 class="subtitle">
                    {move || t!(main.page_n)().with(current_page())}
                    " ("
                    {move || t!(main.n_results_of_m)().with(
                        total_results_reader.get(),
                        total_results_in_current_page_reader.get()
                    )}
                    ")"
                </h2>

                <TapPaginationButtons total_pages_reader=total_pages_reader />
            </Show>

            <table class="table is-bordered is-hoverable is-fullwidth">
                <thead>
                    <tr>
                        <th>{t!(main.email)}</th>
                        <th>{t!(main.name)}</th>
                        <th>{t!(main.creation)}</th>
                        <th>{t!(main.last_update)}</th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        children=view_table_row
                        each=users_reader
                        key=|user| user.id.value().to_string()
                    />
                </tbody>
            </table>

            <Show when=pagination_is_visible>
                <TapPaginationButtons total_pages_reader=total_pages_reader />
            </Show>
        </div>
    }
}
