// Shared Tauri app setup for desktop and mobile. On Android the mobile entry
// point macro exposes the `start_app` symbol Gradle loads from the .so file.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("failed to start Belhana client");
}
