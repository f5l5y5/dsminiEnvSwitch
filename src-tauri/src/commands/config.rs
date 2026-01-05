use tauri::{AppHandle, State};
use crate::models::{AppConfig, Environment, CreateEnvironment};
use crate::store::ConfigStore;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn load_config(store: State<'_, ConfigStore>) -> Result<AppConfig, String> {
    store.load().await
}

#[tauri::command]
pub async fn save_config(config: AppConfig, store: State<'_, ConfigStore>) -> Result<(), String> {
    store.save(&config).await
}

#[tauri::command]
pub async fn import_config_from_json(path: String, store: State<'_, ConfigStore>) -> Result<AppConfig, String> {
    store.import_from_json(&path).await
}

#[tauri::command]
pub async fn export_config_to_json(path: String, store: State<'_, ConfigStore>) -> Result<(), String> {
    let config = store.load().await?;
    store.export_to_json(&config, &path).await
}

#[tauri::command]
pub async fn add_environment(
    environment: CreateEnvironment,
    store: State<'_, ConfigStore>,
    app: AppHandle,
    tray: State<'_, Arc<Mutex<Option<tauri::tray::TrayIcon>>>>,
) -> Result<Environment, String> {
    let result = store.add_environment(environment).await?;

    // 更新托盘菜单
    if let Some(tray_guard) = tray.lock().await.as_ref() {
        let _ = crate::tray::update_tray_menu(&app, tray_guard).await;
    }

    Ok(result)
}

#[tauri::command]
pub async fn update_environment(
    id: String,
    environment: Environment,
    store: State<'_, ConfigStore>,
    app: AppHandle,
    tray: State<'_, Arc<Mutex<Option<tauri::tray::TrayIcon>>>>,
) -> Result<(), String> {
    let result = store.update_environment(&id, environment).await?;

    // 更新托盘菜单
    if let Some(tray_guard) = tray.lock().await.as_ref() {
        let _ = crate::tray::update_tray_menu(&app, tray_guard).await;
    }

    Ok(result)
}

#[tauri::command]
pub async fn delete_environment(
    id: String,
    store: State<'_, ConfigStore>,
    app: AppHandle,
    tray: State<'_, Arc<Mutex<Option<tauri::tray::TrayIcon>>>>,
) -> Result<(), String> {
    let result = store.delete_environment(&id).await?;

    // 更新托盘菜单
    if let Some(tray_guard) = tray.lock().await.as_ref() {
        let _ = crate::tray::update_tray_menu(&app, tray_guard).await;
    }

    Ok(result)
}

#[tauri::command]
pub async fn switch_environment(
    env_id: String,
    store: State<'_, ConfigStore>,
    app: AppHandle,
    tray: State<'_, Arc<Mutex<Option<tauri::tray::TrayIcon>>>>,
) -> Result<(), String> {
    let result = store.set_current_environment(&env_id).await?;

    // 更新托盘菜单
    if let Some(tray_guard) = tray.lock().await.as_ref() {
        let _ = crate::tray::update_tray_menu(&app, tray_guard).await;
    }

    Ok(result)
}
