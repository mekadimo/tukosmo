use leptos::IntoView;
use leptos::SignalUpdate;
use leptos::component;
use leptos::create_signal;
use leptos::tracing;
use leptos::view;

use super::super::WebLayout;

const DEFAULT_COUNT: i64 = 0;
const DEFAULT_MESSAGE: &'static str = "Hello!";

fn change_message(message_draft: &mut String) {
    if message_draft == "Hello!" {
        *message_draft = "Goodbye!".to_string();
    } else {
        *message_draft = "Hello!".to_string();
    }
}

#[component]
pub fn WebHomeView() -> impl IntoView {
    let (message, set_message) = create_signal(DEFAULT_MESSAGE.to_string());
    let (count, set_count) = create_signal(DEFAULT_COUNT);

    let on_click_counter = move |_|
        set_count.update(|count_draft| {
            *count_draft += 1;
        });
    let on_click_message = move |_| set_message.update(change_message);

    view! {
        <WebLayout>
            <h1>"Tukosmo is running Leptos!"</h1>
            <button on:click=on_click_counter>"Click Me: " {count}</button>
            <button on:click=on_click_message>{message}</button>
        </WebLayout>
    }
}
