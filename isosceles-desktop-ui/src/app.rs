use crate::clicker_view_elements::clickable_elements::*;
use leptos::ev::MouseEvent;
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (project_list, set_project_list) = signal::<Vec<String>>(Vec::from([]));

    let _greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: "" }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let _new_msg = invoke("greet", args).await.as_string().unwrap();
        });
    };

    let _get_version = move |_ev: MouseEvent| {
        _ev.prevent_default();
        spawn_local(async move {
            let result: Vec<String> =
                serde_wasm_bindgen::from_value(invoke("_dk_list_apps", JsValue::undefined()).await)
                    .unwrap();
            log::info!("projects: {result:?}");
            set_project_list.set(result);
        });
    };

    let _new_click = move |_ev: MouseEvent| {
        _ev.prevent_default();
    };

    view! {
        <main class="container-fluid min-vh-100 py-4 d-flex flex-column text-center justify-content-center align-items-center">
            <h1 class="mb-3">"Isosceles Desktop Tool"</h1>
            <p class="lead text-muted mb-4">"What are you doing today?"</p>
            <div class="d-flex gap-2 justify-content-center">
                <button
                    class="btn btn-primary btn-md"
                    data-bs-toggle="modal"
                    data-bs-target="#createModal"
                >
                    Create New +
                </button>
                <button
                    class="btn btn-outline-secondary btn-md"
                    data-bs-toggle="modal"
                    data-bs-target="#projectlisting"
                    on:click=_get_version
                >
                    View existing projects
                </button>
            </div>
            <div class="modal fade" id="createModal" tabindex="-1">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title">"New Project (disabled)"</h5>
                            <button class="btn-close" data-bs-dismiss="modal" />
                        </div>
                        <div class="modal-body">
                            "Not supported. Visit bevy.org to add new projects."
                        </div>
                        <div class="modal-footer">
                            <button class="btn btn-secondary" data-bs-dismiss="modal">
                                "Cancel"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
            <div class="modal fade" id="projectlisting" tabindex="-1">
                <div class="modal-dialog modal-dialog-centered">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title">"Run already existing projects."</h5>
                            <button class="btn-close" data-bs-dismiss="modal" />
                        </div>
                        <div class="table-responsive">
                            <table class="table table-hover align-middle mb-0">
                                <thead>
                                    <tr>
                                        <th scope="col">"game files"</th>
                                        <th scope="col" class="text-end">
                                            "runners"
                                        </th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {move || {
                                        project_list
                                            .get()
                                            .into_iter()
                                            .map(|i| {
                                                view! {
                                                    <tr>
                                                        <td class="mb-0">{i}</td>
                                                        <td>{__component_list_apps_click_group()}</td>
                                                    </tr>
                                                }
                                            })
                                            .collect_view()
                                    }}
                                </tbody>
                            </table>
                        </div>
                        <div class="modal-footer">
                            <button class="btn btn-secondary" data-bs-dismiss="modal">
                                "Close"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
