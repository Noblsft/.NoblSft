mod helpers;
mod vault;

use crate::vault::vault_service::VaultService;
use crate::vault::types::VaultHandle;
use tauri::Manager;
use tauri::State;

struct AppState {
    vault: VaultService,
}

#[tauri::command]
fn create_vault(state: State<AppState>, path: String) -> Result<(), String> {
    state
        .vault
        .create_vault(path)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn load_vault(state: State<AppState>, path: String) -> Result<VaultHandle, String> {
    state
        .vault
        .load_vault(path)
        .map_err(|err| err.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_version = app.package_info().version.to_string();
            let workspace_root = app.path().app_data_dir().expect("app_data_dir").join("vault-workspaces");

            app.manage(AppState {
                vault: VaultService::new(app_version, 1, workspace_root),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_vault, load_vault])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
