// Hide the console window on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]

use aiyara_domain::{AppError, Container};
use aiyara_engine::{EngineConnector, FakeConnector};
use std::sync::Arc;
use tauri::State;

/// Shared connector. Swapped for the real WSLC connector in a later ticket.
struct Engine(Arc<dyn EngineConnector>);

/// Thin wrapper: all logic lives in the `EngineConnector` implementation.
#[tauri::command]
async fn list_containers(engine: State<'_, Engine>) -> Result<Vec<Container>, AppError> {
    let connector = engine.0.clone();
    let mut all = Vec::new();
    for scope in connector.scopes().await? {
        all.extend(connector.list_containers(&scope).await?);
    }
    Ok(all)
}

fn main() {
    tauri::Builder::default()
        .manage(Engine(Arc::new(FakeConnector)))
        .invoke_handler(tauri::generate_handler![list_containers])
        .run(tauri::generate_context!())
        .expect("error while running the Tauri application");
}
