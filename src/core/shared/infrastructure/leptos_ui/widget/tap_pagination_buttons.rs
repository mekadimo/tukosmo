use leptos::For;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::Show;
use leptos::SignalGet;
use leptos::component;
use leptos::tracing;
use leptos::view;
use leptos_router::Location;
use leptos_router::use_location;
use leptos_router::use_query_map;

use crate::t;
use super::super::service::navigation;

#[component]
pub fn TapPaginationButtons(
    total_pages_reader: ReadSignal<i64>
) -> impl IntoView {
    let Location {
        pathname: current_uri_path,
        search: current_uri_query,
        ..
    } = use_location();

    let url_query_map = use_query_map();
    let current_page = move || {
        match url_query_map.get().get("p") {
            Some(p) => p.parse::<i64>().unwrap_or(1),
            None => 1,
        }
    };

    let previous_current_page = move || current_page() - 1;
    let next_current_page = move || current_page() + 1;
    let page_numbers = move ||
        (1..total_pages_reader() + 1).collect::<Vec<i64>>();

    view! {
        <nav
            aria-label="pagination"
            class="pagination is-centered"
            role="navigation"
        >
            <Show when=move || 1 == current_page()>
                <button class="pagination-previous" disabled=true>
                    {t!(main.previous_w_page)}
                </button>
            </Show>
            <Show when=move || 1 != current_page()>
                <a
                    class="pagination-previous"
                    href=move || navigation::change_uri_query_param(
                        &current_uri_path(),
                        &current_uri_query(),
                        "p",
                        &(current_page() - 1).to_string()
                    )
                >
                    {t!(main.previous_w_page)}
                </a>
            </Show>

            <Show when=move || current_page() == total_pages_reader()>
                <button class="pagination-next" disabled=true>
                    {t!(main.next_w_page)}
                </button>
            </Show>
            <Show when=move || current_page() != total_pages_reader()>
                <a
                    class="pagination-next"
                    href=move || navigation::change_uri_query_param(
                        &current_uri_path(),
                        &current_uri_query(),
                        "p",
                        &(current_page() + 1).to_string()
                    )
                >
                    {t!(main.next_w_page)}
                </a>
            </Show>

            <ul class="pagination-list">
                <For
                    children=move |p: i64| {
                        let is_first_page = 1 == p;
                        let is_previous_page = move || p == previous_current_page();
                        let is_current_page = move || p == current_page();
                        let is_next_page = move || p == next_current_page();
                        let is_last_page = move || p == total_pages_reader();

                        let include_ellipsis_before = move ||
                            is_previous_page() && current_page() > 3;
                        let include_ellipsis_after = move ||
                            is_next_page() && current_page() < total_pages_reader() - 2;

                        view! {
                            <Show when=include_ellipsis_before>
                                <li>
                                    <span class="pagination-ellipsis">
                                        "…"
                                    </span>
                                </li>
                            </Show>

                            <Show when=move || {
                                is_first_page ||
                                is_previous_page() ||
                                is_current_page() ||
                                is_next_page() ||
                                is_last_page()
                            }>
                                <li>
                                    <a
                                        class="pagination-link"
                                        class=("is-current", is_current_page)
                                        href=move || navigation::change_uri_query_param(
                                            &current_uri_path(),
                                            &current_uri_query(),
                                            "p",
                                            &(p.to_string())
                                        )
                                    >
                                        {p}
                                    </a>
                                </li>

                            </Show>

                            <Show when=include_ellipsis_after>
                                <li>
                                    <span class="pagination-ellipsis">
                                        "…"
                                    </span>
                                </li>
                            </Show>
                        }
                    }
                    each=page_numbers
                    key=|p| p.to_string()
                />
            </ul>
        </nav>
    }
}
