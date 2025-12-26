mod helpers;
mod vault;

use crate::vault::types::VaultHandle;
use crate::vault::vault_service::VaultService;

use std::sync::Mutex;
use tauri::{Manager, State, WindowEvent};

struct AppState {
    vault: VaultService,
    // TODO: move this to vault service
    opened_vault: Mutex<Option<VaultHandle>>,
}

fn cleanup_opened_vault(app: &tauri::AppHandle) {
    let Some(state) = app.try_state::<AppState>() else {
        return;
    };

    let maybe_handle = {
        let mut guard = state
            .opened_vault
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        guard.take()
    };

    if let Some(handle) = maybe_handle {
        let _ = state.vault.close_vault(&handle);
    }
}

#[tauri::command]
fn create_vault(state: State<AppState>, path: String) -> Result<VaultHandle, String> {
    state
        .vault
        .create_vault(&path)
        .map_err(|err| err.to_string())?;

    let handle = state
        .vault
        .load_vault(&path)
        .map_err(|err| err.to_string())?;

    let mut opened = state.opened_vault.lock().unwrap();
    *opened = Some(handle.clone());

    Ok(handle)
}

#[tauri::command]
fn load_vault(state: State<AppState>, path: String) -> Result<VaultHandle, String> {
    let handle = state
        .vault
        .load_vault(&path)
        .map_err(|err| err.to_string())?;

    let mut opened = state.opened_vault.lock().unwrap();
    *opened = Some(handle.clone());

    Ok(handle)
}

#[tauri::command]
fn close_vault(state: State<AppState>) -> Result<(), String> {
    state
        .vault
        .close_vault(state.opened_vault.lock().unwrap().as_ref().ok_or("no vault opened")?)
        .map_err(|err| err.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_version = app.package_info().version.to_string();

            let workspace_root = app
                .path()
                .app_data_dir()
                .expect("app_data_dir")
                .join("vault-workspaces");

            app.manage(AppState {
                vault: VaultService::new(app_version, 1, workspace_root),
                opened_vault: Mutex::new(None),
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                WindowEvent::CloseRequested { .. } => {
                    cleanup_opened_vault(&window.app_handle());
                }
                WindowEvent::Destroyed => {
                    cleanup_opened_vault(&window.app_handle());
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![create_vault, load_vault, close_vault])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
