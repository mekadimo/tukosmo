use console_error_panic_hook;
use console_log;
use leptos::{ mount_to_body, view };
use tukosmo_infrastructure::core::shared::leptos_ui::App;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(move || {
        view! { <App/> }
    });
}
