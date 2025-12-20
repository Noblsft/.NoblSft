mod vault;

use tauri::Manager;
use tauri::State;
use crate::vault::vault_service::VaultService;

struct AppState {
    vault: VaultService,
}

#[tauri::command]
fn create_vault(state: State<AppState>, path: String) -> Result<(), String> {
    state.vault.create_vault(path).map_err(|err| err.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_version = app.package_info().version.to_string();

            app.manage(AppState {
                vault: VaultService::new(app_version, 1),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_vault])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
