use leptos::IntoView;
use leptos::component;
use leptos::tracing;
use leptos::view;
use leptos_icons::Icon as LeptosIcon;
use leptos_icons::BsIcon::BsArchiveFill;
use leptos_icons::BsIcon::BsBlockquoteLeft;
use leptos_icons::BsIcon::BsBookmarkStarFill;
use leptos_icons::BsIcon::BsCloudDownloadFill;
use leptos_icons::BsIcon::BsCreditCardFill;
use leptos_icons::BsIcon::BsDatabaseFill;
use leptos_icons::BsIcon::BsEthernet;
use leptos_icons::BsIcon::BsFileEarmarkTextFill;
use leptos_icons::BsIcon::BsGlobe2;
use leptos_icons::BsIcon::BsGrid1x2Fill;
use leptos_icons::BsIcon::BsHouseFill;
use leptos_icons::BsIcon::BsImages;
use leptos_icons::BsIcon::BsKanbanFill;
use leptos_icons::BsIcon::BsLayoutSidebarInsetReverse;
use leptos_icons::BsIcon::BsLayoutTextWindow;
use leptos_icons::BsIcon::BsLifePreserver;
use leptos_icons::BsIcon::BsList;
use leptos_icons::BsIcon::BsListTask;
use leptos_icons::BsIcon::BsPalette2;
use leptos_icons::BsIcon::BsPeopleFill;
use leptos_icons::BsIcon::BsPieChartFill;
use leptos_icons::BsIcon::BsQuestionSquareFill;
use leptos_icons::BsIcon::BsShop;
use leptos_icons::BsIcon::BsStarFill;
use leptos_icons::BsIcon::BsStars;
use leptos_icons::BsIcon::BsTagFill;
use leptos_icons::BsIcon::BsTranslate;

#[component]
pub fn Archive() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsArchiveFill) /> }
}

#[component]
pub fn Blockquote() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsBlockquoteLeft) /> }
}

#[component]
pub fn BookmarkStar() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsBookmarkStarFill) /> }
}

#[component]
pub fn CloudDownload() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsCloudDownloadFill) /> }
}

#[component]
pub fn CreditCard() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsCreditCardFill) /> }
}

#[component]
pub fn Database() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsDatabaseFill) /> }
}

#[component]
pub fn Document() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsFileEarmarkTextFill) /> }
}

#[component]
pub fn Ethernet() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsEthernet) /> }
}

#[component]
pub fn Gallery() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsImages) /> }
}

#[component]
pub fn Globe() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsGlobe2) /> }
}

#[component]
pub fn Grid() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsGrid1x2Fill) /> }
}

#[component]
pub fn House() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsHouseFill) /> }
}

#[component]
fn Icon(icon: LeptosIcon) -> impl IntoView {
    view! {
        <span class="tukosmo-icon">
            <LeptosIcon icon=icon />
        </span>
    }
}

#[component]
pub fn Kanban() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsKanbanFill) /> }
}

#[component]
pub fn LayoutSidebar() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsLayoutSidebarInsetReverse) /> }
}

#[component]
pub fn LifeSaver() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsLifePreserver) /> }
}

#[component]
pub fn List() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsListTask) /> }
}

#[component]
pub fn NavigationBurger() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsList) /> }
}

#[component]
pub fn Palette() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsPalette2) /> }
}

#[component]
pub fn PieChart() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsPieChartFill) /> }
}

#[component]
pub fn QuestionSquare() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsQuestionSquareFill) /> }
}

#[component]
pub fn Shop() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsShop) /> }
}

#[component]
pub fn Star() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsStarFill) /> }
}

#[component]
pub fn Stars() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsStars) /> }
}

#[component]
pub fn Tag() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsTagFill) /> }
}

#[component]
pub fn Translate() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsTranslate) /> }
}

#[component]
pub fn Users() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsPeopleFill) /> }
}

#[component]
pub fn Website() -> impl IntoView {
    view! { <Icon icon=LeptosIcon::from(BsLayoutTextWindow) /> }
}
