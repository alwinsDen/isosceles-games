use leptos::{IntoView, component, prelude::*};

#[component]
pub fn list_apps_click_group() -> impl IntoView {
    view! {
        <div class="d-flex flex-row gap-1 flex-end justify-content-end">
            <button class="btn btn-primary btn-sm">"🏃DR"</button>
            <button class="btn btn-secondary btn-sm">"🏃‍♀️RR"</button>
            <button class="btn">"👀"</button>
        </div>
    }
}
