// Belhana desktop shell. All domain logic lives in the shared API server;
// this binary only hosts the SvelteKit SPA in a WebView and points it at
// the local API origin (see app.html + BELHANA_API_BASE).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    belhana_desktop_lib::run();
}
