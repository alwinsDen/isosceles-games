mod app;
mod clicker_view_elements;
mod tauri_log;

use app::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    tauri_log::init();
    mount_to_body(|| {
        view! { <App /> }
    })
}
