use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, State};
use crate::store::ConfigStore;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn apply_environment(
    env_id: String,
    store: State<'_, ConfigStore>,
    app: AppHandle,
    tray: State<'_, Arc<Mutex<Option<tauri::tray::TrayIcon>>>>,
) -> Result<String, String> {
    let config = store.load().await?;
    let environment = config
        .environments
        .iter()
        .find(|e| e.id == env_id)
        .ok_or("Environment not found")?;

    // 获取目标文件路径（优先使用环境自己的，如果没有则使用全局的）
    let target_file_path = environment
        .target_file_path
        .as_ref()
        .or(config.settings.target_file_path.as_ref())
        .ok_or("未设置目标文件路径，请先在设置或环境配置中设置")?;

    // 将 ext 序列化为格式化的 JSON 字符串
    let new_content = serde_json::to_string_pretty(&environment.ext)
        .map_err(|e| format!("Failed to serialize environment config: {}", e))?;

    // 如果文件存在，读取并比较内容
    if PathBuf::from(target_file_path).exists() {
        let current_content = fs::read_to_string(target_file_path)
            .map_err(|e| format!("Failed to read target file: {}", e))?;

        // 比较内容是否相同
        if current_content.trim() == new_content.trim() {
            return Ok(format!("文件内容已是最新，无需更新\n文件: {}", target_file_path));
        }
    }

    // 写入文件（使用全局的自动备份设置）
    write_file(target_file_path, &new_content, config.settings.auto_backup).await?;

    // 记录当前应用的环境ID
    store.set_current_applied_environment(&env_id).await?;

    // 更新托盘菜单
    if let Some(tray_guard) = tray.lock().await.as_ref() {
        let _ = crate::tray::update_tray_menu(&app, tray_guard).await;
    }

    Ok(format!("已应用环境: {}\n目标文件: {}", environment.name, target_file_path))
}

async fn write_file(path: &str, content: &str, auto_backup: bool) -> Result<(), String> {
    let path_buf = PathBuf::from(path);

    // 自动备份
    if auto_backup && path_buf.exists() {
        let backup_path = format!(
            "{}.backup_{}",
            path,
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        );
        fs::copy(&path_buf, &backup_path)
            .map_err(|e| format!("Failed to backup file: {}", e))?;
    }

    // 确保父目录存在
    if let Some(parent) = path_buf.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create parent directory: {}", e))?;
    }

    // 写入文件
    fs::write(&path_buf, content)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
pub async fn select_file_path() -> Result<Option<String>, String> {
    // Note: In Tauri 2.x, file dialog should be called from the frontend
    // using @tauri-apps/plugin-dialog
    Ok(None)
}

#[tauri::command]
pub async fn select_folder_path() -> Result<Option<String>, String> {
    // Note: In Tauri 2.x, file dialog should be called from the frontend
    // using @tauri-apps/plugin-dialog
    Ok(None)
}
