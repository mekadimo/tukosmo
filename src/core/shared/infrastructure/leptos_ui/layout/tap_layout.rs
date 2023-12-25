use leptos::Children;
use leptos::ChildrenFn;
use leptos::For;
#[cfg(feature = "ssr")]
use leptos::IntoAttribute;
#[cfg(feature = "ssr")]
use leptos::IntoClass;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::SignalGet;
use leptos::SignalUpdate;
use leptos::WriteSignal;
use leptos::component;
use leptos::create_signal;
use leptos::tracing;
use leptos::view;
use leptos_meta::Html;
use leptos_router::Location;
use leptos_router::NavigateOptions;
use leptos_router::use_location;
use leptos_router::use_navigate;
use tukosmo_domain::core::language::model::Language;

use crate::t;
use super::I18nLayout;
use super::super::GlobalContext;
use super::super::service::navigation;
use super::super::use_global_context;
use super::super::widget::icon;

#[component]
pub fn LanguagesModal(
    set_show: WriteSignal<bool>,
    show: ReadSignal<bool>
) -> impl IntoView {
    let on_click_close_languages_modal = move |_|
        set_show.update(|draft| {
            *draft = false;
        });

    let GlobalContext { current_language_reader, languages_reader, .. } =
        use_global_context();

    view! {
        <div
            class="modal"
            class=("is-active", move || show())
        >
            <div
                class="modal-background"
                on:click=on_click_close_languages_modal
            ></div>

            <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title">
                        {t!(main.select_a_language)}
                    </p>
                    <button
                        class="delete"
                        on:click=on_click_close_languages_modal
                    ></button>
                </header>

                <section class="modal-card-body">
                    <ul>
                        <For
                            children=move |language: Language| {
                                view! {
                                    <LanguagesModalListElement
                                        language={language}
                                        set_show={set_show}
                                    />
                                }
                            }
                            each=languages_reader
                            key=|language| language.id.value().to_string()
                        />
                    </ul>
                </section>

                <footer class="modal-card-foot">
                    <a
                        class="button is-link"
                        href=move || navigation::path_admin_languages(
                            &current_language_reader.get().code
                        )
                    >
                        {t!(main.see_languages)}
                    </a>
                </footer>
            </div>
        </div>
    }
}

#[component]
pub fn LanguagesModalListElement(
    language: Language,
    set_show: WriteSignal<bool>
) -> impl IntoView {
    let Location {
        pathname: current_uri_path,
        search: current_uri_query,
        ..
    } = use_location();

    let navigate_to = move |language_code: &str| {
        let path = navigation::change_uri_language(
            &current_uri_path(),
            &current_uri_query(),
            language_code
        );
        let navigate = use_navigate();
        set_show.update(|draft| {
            *draft = false;
        });
        navigate(&path, NavigateOptions::default());
    };

    let GlobalContext { current_language_reader, .. } = use_global_context();

    view! {
        <li>
            <a on:click=move |_| navigate_to(language.code.value())>
                {move || language.original_name.value().to_string()}
                // TODO: Show " (<translated_name>)" only if
                // current_language != language
                " ("
                {move || language.name.translate(current_language_reader.get().id)}
                ")"
            </a>
        </li>
    }
}

#[component]
fn Navbar() -> impl IntoView {
    let GlobalContext { current_language_reader, website_title_reader, .. } =
        use_global_context();

    let (languages_modal_is_open, set_languages_modal_is_open) =
        create_signal(false);
    let (
        navbar_menu_is_showed_in_small_screen,
        set_navbar_menu_is_showed_in_small_screen,
    ) = create_signal(false);
    let (user_dropdown_is_active, set_user_dropdown_is_active) =
        create_signal(false);

    let on_click_open_languages_modal = move |_|
        set_languages_modal_is_open.update(|draft| {
            *draft = true;
        });

    let on_click_toggle_navbar_menu_in_small_screen = move |_|
        set_navbar_menu_is_showed_in_small_screen.update(|draft| {
            *draft = !draft.clone();
        });

    let on_click_toggle_user_dropdown = move |_|
        set_user_dropdown_is_active.update(|draft| {
            *draft = !draft.clone();
        });

    view! {
        <nav class="navbar is-white">
            <div class="navbar-brand">
                <a
                    class="navbar-item brand-text tap-logo"
                    href=move || navigation::path_admin_dashboard(
                        &current_language_reader.get().code
                    )
                    title=t!(main.tukosmo_admin_panel)
                >
                    ""
                </a>
                <div
                    class="navbar-burger burger"
                    on:click=on_click_toggle_navbar_menu_in_small_screen
                >
                    <span></span>
                    <span></span>
                    <span></span>
                </div>
            </div>

            <div
                class="navbar-menu"
                class=(
                    "is-active",
                    move || navbar_menu_is_showed_in_small_screen()
                )
            >
                <div class="navbar-start">
                    <a
                        class="navbar-item"
                        href=move || navigation::path_home(
                            &current_language_reader.get().code
                        )
                        target="_blank"
                        title=t!(main.visit_website)
                    >
                        <icon::House />
                        {website_title_reader}
                    </a>

                    <a class="navbar-item" href="" target="_blank">
                        <icon::LifeSaver />
                        {t!(main.help)}
                    </a>
                </div>

                <div class="navbar-end">
                    <a
                        class="navbar-item is-uppercase"
                        on:click=on_click_open_languages_modal
                        title={t!(main.select_a_language)}
                    >
                        <icon::Globe />
                        {move || current_language_reader.get().code.value().to_string()}
                    </a>
                    <LanguagesModal
                        set_show=set_languages_modal_is_open
                        show=languages_modal_is_open
                    />

                    <div
                        class="navbar-item has-dropdown"
                        class=("is-active", move || user_dropdown_is_active())
                    >
                        <a
                            class="navbar-link"
                            on:click=on_click_toggle_user_dropdown
                        >
                            "Lajto (test@test.com)"
                        </a>

                        <div class="navbar-dropdown is-right">
                            <a
                                class="navbar-item"
                                href=""
                            >
                                {t!(main.account)}
                            </a>

                            <a
                                class="navbar-item"
                                href=""
                            >
                                {t!(main.sessions)}
                            </a>

                            <hr class="navbar-divider" />

                            <a
                                class="navbar-item"
                                href=""
                            >
                                {t!(main.logout_w_verb)}
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    let GlobalContext { current_language_reader, .. } = use_global_context();

    view! {
        // TODO: Toggle is-hidden-mobile somehow
        <aside class="menu is-hidden-mobile">
            <ul class="menu-list">
                <li>
                    <a
                        href=move || navigation::path_admin_dashboard(
                            &current_language_reader.get().code
                        )
                    >
                        <icon::Grid />
                        {t!(main.dashboard)}
                    </a>
                </li>

                <SidebarElement>
                    <icon::PieChart />
                    {t!(main.statistics)}
                </SidebarElement>
            </ul>

            <SidebarDataMenu />

            <SidebarModulesMenu />

            <SidebarAppearanceMenu />

            <SidebarSettingsMenu />
        </aside>
    }
}

#[component]
fn SidebarAppearanceMenu() -> impl IntoView {
    view! {
        <p class="menu-label">
            {t!(main.appearance)}
        </p>
        <ul class="menu-list">
            <SidebarElement>
                <icon::Star />
                {t!(main.favicon)}
            </SidebarElement>

            <SidebarElement>
                <icon::Palette />
                {t!(main.theme)}
            </SidebarElement>

            <SidebarElement>
                <icon::List />
                {t!(main.menus)}
            </SidebarElement>

            <SidebarElement>
                <icon::LayoutSidebar />
                {t!(main.widgets)}
            </SidebarElement>
        </ul>
    }
}

#[component]
fn SidebarDataMenu() -> impl IntoView {
    let GlobalContext { current_language_reader, .. } = use_global_context();

    view! {
        <p class="menu-label">
            {t!(main.data)}
        </p>
        <ul class="menu-list">
            <li>
                <a
                    href=move || navigation::path_admin_languages(
                        &current_language_reader.get().code
                    )
                >
                    <icon::Translate />
                    {t!(main.languages)}
                </a>
            </li>

            <li>
                <a
                    href=move || navigation::path_admin_users(
                        &current_language_reader.get().code
                    )
                >
                    <icon::Users />
                    {t!(main.users)}
                </a>
            </li>

            <SidebarElement>
                <icon::Document />
                {t!(main.pages)}
            </SidebarElement>

            <SidebarElement>
                <icon::Tag />
                {t!(main.tags)}
            </SidebarElement>

            <SidebarElement>
                <icon::Archive />
                {t!(main.files)}
            </SidebarElement>
        </ul>
    }
}

#[component]
fn SidebarElement(children: Children) -> impl IntoView {
    view! {
        <li>
            <a href="">
                {children()}
            </a>
        </li>
    }
}

#[component]
fn SidebarModulesMenu() -> impl IntoView {
    view! {
        <p class="menu-label">
            {t!(main.modules)}
        </p>
        <ul class="menu-list">
            <SidebarElement>
                <icon::Blockquote />
                {t!(main.blog)}
            </SidebarElement>

            <SidebarElement>
                <icon::Gallery />
                {t!(main.gallery)}
            </SidebarElement>

            <SidebarElement>
                <icon::QuestionSquare />
                {t!(main.faq)}
            </SidebarElement>
        
            <SidebarElement>
                <icon::CloudDownload />
                {t!(main.downloads)}
            </SidebarElement>

            <SidebarElement>
                <icon::CreditCard />
                {t!(main.payments)}
            </SidebarElement>

            <SidebarElement>
                <icon::BookmarkStar />
                {t!(main.subscriptions)}
            </SidebarElement>

            <SidebarElement>
                <icon::Shop />
                {t!(main.shop)}
            </SidebarElement>

            <SidebarElement>
                <icon::Kanban />
                {t!(main.tasks)}
            </SidebarElement>
        </ul>
    }
}

#[component]
fn SidebarSettingsMenu() -> impl IntoView {
    view! {
        <p class="menu-label">
            {t!(main.settings)}
        </p>
        <ul class="menu-list">
            <SidebarElement>
                <icon::Website />
                {t!(main.website)}
            </SidebarElement>

            <SidebarElement>
                <icon::Ethernet />
                {t!(main.domain_w_web)}
            </SidebarElement>

            <SidebarElement>
                <icon::Stars />
                {t!(main.tukosmo)}
            </SidebarElement>

            <SidebarElement>
                <icon::Database />
                {t!(main.server)}
            </SidebarElement>
        </ul>
    }
}

#[component]
pub fn TapLayout(children: ChildrenFn) -> impl IntoView {
    view! {
        <Html class="tap" />
        <I18nLayout>
            <Navbar />
            <div class="columns">
                <div class="column is-2">
                    <Sidebar />
                </div>
                <div class="column is-10">
                    {children()}
                </div>
            </div>
        </I18nLayout>
    }
}
