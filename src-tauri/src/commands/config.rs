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

#[tauri::command]
pub async fn sync_remote_environments(url: String, store: State<'_, ConfigStore>) -> Result<Vec<crate::models::Environment>, String> {
    println!("[SYNC] 开始同步远程配置: {}", url);

    // 验证URL格式
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("URL必须以http://或https://开头".to_string());
    }

    // 发送HTTP请求获取远程配置
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    println!("[SYNC] 发送HTTP请求...");
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求远程配置失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("远程服务器返回错误: {}", response.status()));
    }

    println!("[SYNC] 请求成功，解析响应JSON...");
    // 解析响应JSON
    let remote_config: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("解析远程配置失败: {}", e))?;

    // 提取environments数组
    let environments = remote_config
        .get("environments")
        .and_then(|v| v.as_array())
        .ok_or("远程配置格式错误: 缺少environments字段".to_string())?;

    println!("[SYNC] 找到 {} 个远程环境", environments.len());

    // 解析environments
    let parsed_environments: Vec<crate::models::Environment> = serde_json::from_value(serde_json::Value::Array(environments.clone()))
        .map_err(|e| format!("解析environments失败: {}", e))?;

    // 加载当前配置
    println!("[SYNC] 加载当前配置...");
    let mut current_config = store.load().await?;
    println!("[SYNC] 当前配置中有 {} 个环境", current_config.environments.len());

    // 合并远程环境配置到本地
    let mut updated = false;
    let mut added_count = 0;
    let mut updated_count = 0;

    for remote_env in &parsed_environments {
        // 检查本地是否已存在相同ID的环境
        if let Some(local_env) = current_config.environments.iter_mut().find(|e| e.id == remote_env.id) {
            // 更新已存在的环境
            local_env.name = remote_env.name.clone();
            local_env.desc = remote_env.desc.clone();
            local_env.ext = remote_env.ext.clone();
            local_env.target_file_path = remote_env.target_file_path.clone();
            local_env.updated_at = remote_env.updated_at.clone();
            updated = true;
            updated_count += 1;
        } else {
            // 添加新环境
            current_config.environments.push(remote_env.clone());
            updated = true;
            added_count += 1;
        }
    }

    println!("[SYNC] 合并完成: 新增 {} 个，更新 {} 个", added_count, updated_count);
    println!("[SYNC] 合并后总共 {} 个环境", current_config.environments.len());

    // 保存更新后的配置
    if updated {
        println!("[SYNC] 保存配置到存储...");
        // 同时更新远程配置URL
        current_config.settings.remote_config_url = Some(url);
        store.save(&current_config).await?;
        println!("[SYNC] 配置保存成功");
    }

    Ok(parsed_environments)
}
