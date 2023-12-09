use chrono::Datelike;
use chrono::Utc;
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
fn Footer() -> impl IntoView {
    let GlobalContext { website_title_reader, .. } = use_global_context();

    let current_year = Utc::now().year();

    view! {
        <div class="core-shared-web_layout-footer">
            <div class="core-shared-web_layout-footer-container">
                <div class="core-shared-web_layout-footer-credits">
                    {website_title_reader}
                    " © "
                    {current_year}
                </div>
            </div>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    let GlobalContext {
        current_language_reader,
        website_subtitle_reader,
        website_title_reader,
        ..
    } = use_global_context();

    view! {
        <header class="core-shared-web_layout-header">
            <div class="core-shared-web_layout-header-container">
                <div class="core-shared-web_layout-header-branding">
                    <h1 class="core-shared-web_layout-header-title">
                        <a
                            href=move || navigation::path_home(
                                &current_language_reader.get().code
                            )
                        >
                            {website_title_reader}
                        </a>
                    </h1>

                    <p class="core-shared-web_layout-header-description">
                        {website_subtitle_reader}
                    </p>
                </div>
            </div>
        </header>
    }
}

#[component]
fn Languages(
    set_show: WriteSignal<bool>,
    show: ReadSignal<bool>
) -> impl IntoView {
    let on_click_close_languages_modal = move |_|
        set_show.update(|draft| {
            *draft = false;
        });

    let GlobalContext { languages_reader, .. } = use_global_context();

    view! {
        <div
            class="core-shared-web_layout-languages animated-03"
            class=("animated-hidden", move || !show())
        >
            <div
                class="core-shared-web_layout-languages-bg"
                on:click=on_click_close_languages_modal
            ></div>

            <div class="core-shared-web_layout-languages-content">
                <button
                    class="core-shared-web_layout-languages-close"
                    on:click=on_click_close_languages_modal
                    title={t!(main.close)}
                ></button>
                <h3>{t!(main.select_a_language)}</h3>

                <ul>
                    <For
                        children=move |language: Language| {
                            view! {
                                <LanguagesListElement
                                    language={language}
                                    set_show={set_show}
                                />
                            }
                        }
                        each=languages_reader
                        key=|language| language.id.value().to_string()
                    />
                </ul>
            </div>
        </div>
    }
}

#[component]
fn LanguagesListElement(
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
fn Navigation() -> impl IntoView {
    let GlobalContext { current_language_reader, .. } = use_global_context();

    let (languages_modal_is_open, set_languages_modal_is_open) =
        create_signal(false);
    let (
        navbar_menu_is_showed_in_small_screen,
        set_navbar_menu_is_showed_in_small_screen,
    ) = create_signal(false);

    let on_click_open_languages_modal = move |_|
        set_languages_modal_is_open.update(|draft| {
            *draft = true;
        });

    let on_click_toggle_navbar_menu_in_small_screen = move |_|
        set_navbar_menu_is_showed_in_small_screen.update(|draft| {
            *draft = !draft.clone();
        });

    view! {
        <nav class="core-shared-web_layout-navigation">
            <div class="core-shared-web_layout-navigation-container">
                <button
                    class="core-shared-web_layout-navigation-burger"
                    on:click=on_click_toggle_navbar_menu_in_small_screen
                >
                    <span class="core-shared-web_layout-navigation-burger-text">
                        {t!(main.menu)}
                    </span>

                    <span class="core-shared-web_layout-navigation-burger-icon">
                        <icon::NavigationBurger />
                    </span>
                </button>

                <ul
                    class="core-shared-web_layout-navigation-ul"
                    class=(
                        "is-active",
                        move || navbar_menu_is_showed_in_small_screen()
                    )
                >
                    <li>
                        <a href="">
                            {t!(main.blog)}
                        </a>
                    </li>
                    <li>
                        <a
                            href=move || navigation::path_login(
                                &current_language_reader.get().code
                            )
                        >
                            {t!(main.login_w_noun)}
                        </a>
                    </li>
                    <li>
                        <a
                            href=move || navigation::path_admin_dashboard(
                                &current_language_reader.get().code
                            )
                        >
                            {t!(main.administration_panel)}
                        </a>
                    </li>
                    <li>
                        <button on:click=on_click_open_languages_modal>
                            <icon::Globe />
                            {move || current_language_reader.get().code.value().to_string()}
                        </button>
                        <Languages
                            set_show=set_languages_modal_is_open
                            show=languages_modal_is_open
                        />
                    </li>
                </ul>
            </div>
        </nav>
    }
}

#[component]
pub fn WebLayout(children: ChildrenFn) -> impl IntoView {
    view! {
        <Html class="web" />
        <I18nLayout>
            <div class="core-shared-web_layout">
                <div class="core-shared-web_layout-top">
                    <Header />
                    <Navigation />
                </div>

                <div class="core-shared-web_layout-content">
                    <div class="core-shared-web_layout-container">
                        <div class="core-shared-web_layout-content-inside">
                            <main class="core-shared-web_layout-main">
                                {children()}
                            </main>

                            <aside class="core-shared-web_layout-sidebar">
                                // TODO: Widgets
                            </aside>
                        </div>
                    </div>
                </div>

                <Footer />
            </div>
        </I18nLayout>
    }
}
