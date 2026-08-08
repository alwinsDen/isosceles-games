use isosceles_core::__core_version_details;

#[tauri::command]
fn _dk_list_apps() -> &'static [&'static str] {
    return __core_version_details();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Debug)
                .clear_targets()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![_dk_list_apps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
