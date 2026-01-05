use std::sync::Arc;
use std::fs;
use std::path::Path;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use tokio::sync::{Mutex, RwLock};
use crate::models::{AppConfig, Environment, CreateEnvironment};

pub struct ConfigStore {
    app: AppHandle,
    store_lock: Arc<Mutex<()>>,
    // 使用 RwLock 缓存配置，支持并发读写
    config_cache: Arc<RwLock<Option<AppConfig>>>,
}

impl ConfigStore {
    pub fn new(app: AppHandle) -> Self {
        Self {
            app,
            store_lock: Arc::new(Mutex::new(())),
            config_cache: Arc::new(RwLock::new(None)),
        }
    }

    // 获取缓存的 close_to_tray 设置（同步读取）
    pub async fn get_close_to_tray(&self) -> bool {
        if let Some(config) = self.config_cache.read().await.as_ref() {
            config.settings.close_to_tray
        } else {
            true // 默认值
        }
    }

    pub async fn load(&self) -> Result<AppConfig, String> {
        let _lock = self.store_lock.lock().await;

        let store = self.app.store("config.json")
            .map_err(|e| format!("Failed to open store: {}", e))?;

        let config = match store.get("config") {
            Some(value) => {
                serde_json::from_value(value.clone())
                    .map_err(|e| format!("Failed to deserialize config: {}", e))?
            }
            None => AppConfig::default(),
        };

        // 更新缓存
        *self.config_cache.write().await = Some(config.clone());

        Ok(config)
    }

    pub async fn save(&self, config: &AppConfig) -> Result<(), String> {
        let _lock = self.store_lock.lock().await;

        let store = self.app.store("config.json")
            .map_err(|e| format!("Failed to open store: {}", e))?;

        let value = serde_json::to_value(config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        store.set("config", value.clone());
        store.save()
            .map_err(|e| format!("Failed to save store: {}", e))?;

        // 更新缓存
        *self.config_cache.write().await = Some(config.clone());

        // 同步到外部 JSON 文件（如果配置了路径）
        if let Some(ref external_path) = config.settings.external_config_path {
            if !external_path.is_empty() {
                self.export_to_json(config, external_path).await?;
            }
        }

        Ok(())
    }

    // 从外部 JSON 文件导入配置
    pub async fn import_from_json(&self, path: &str) -> Result<AppConfig, String> {
        // 读取 JSON 文件
        let json_content = fs::read_to_string(path)
            .map_err(|e| format!("无法读取文件: {}", e))?;

        // 解析 JSON
        let config: AppConfig = serde_json::from_str(&json_content)
            .map_err(|e| format!("JSON 格式错误: {}", e))?;

        // 保存到内部存储
        self.save(&config).await?;

        Ok(config)
    }

    // 导出配置到外部 JSON 文件
    pub async fn export_to_json(&self, config: &AppConfig, path: &str) -> Result<(), String> {
        println!("[DEBUG] export_to_json: path = {}", path);

        // 确保父目录存在
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("无法创建目录: {}", e))?;
        }

        // 序列化为格式化的 JSON
        let json_content = serde_json::to_string_pretty(config)
            .map_err(|e| format!("序列化失败: {}", e))?;

        println!("[DEBUG] export_to_json: 写入 {} 字节", json_content.len());

        // 写入文件
        fs::write(path, json_content)
            .map_err(|e| format!("无法写入文件: {}", e))?;

        println!("[DEBUG] export_to_json: 写入成功");

        Ok(())
    }

    pub async fn add_environment(&self, env: CreateEnvironment) -> Result<Environment, String> {
        let mut config = self.load().await?;

        // 创建完整的 Environment 对象
        let new_env = Environment {
            id: uuid::Uuid::new_v4().to_string(),
            name: env.name,
            desc: env.desc,
            ext: env.ext,
            target_file_path: env.target_file_path,
            created_at: chrono::Local::now().to_rfc3339(),
            updated_at: chrono::Local::now().to_rfc3339(),
        };

        config.environments.push(new_env.clone());
        self.save(&config).await?;

        Ok(new_env)
    }

    pub async fn update_environment(&self, id: &str, updated_env: Environment) -> Result<(), String> {
        let mut config = self.load().await?;

        if let Some(env) = config.environments.iter_mut().find(|e| e.id == id) {
            env.name = updated_env.name;
            env.desc = updated_env.desc;
            env.ext = updated_env.ext;
            env.updated_at = chrono::Local::now().to_rfc3339();
        } else {
            return Err("Environment not found".to_string());
        }

        self.save(&config).await?;
        Ok(())
    }

    pub async fn delete_environment(&self, id: &str) -> Result<(), String> {
        let mut config = self.load().await?;
        let initial_len = config.environments.len();
        config.environments.retain(|e| e.id != id);

        if config.environments.len() == initial_len {
            return Err("Environment not found".to_string());
        }

        self.save(&config).await?;
        Ok(())
    }

    pub async fn set_current_applied_environment(&self, env_id: &str) -> Result<(), String> {
        let mut config = self.load().await?;

        // 验证环境是否存在
        if !config.environments.iter().any(|e| e.id == env_id) {
            return Err("Environment not found".to_string());
        }

        config.settings.current_applied_environment_id = Some(env_id.to_string());
        self.save(&config).await?;
        Ok(())
    }

    pub async fn set_current_environment(&self, env_id: &str) -> Result<(), String> {
        let mut config = self.load().await?;

        // 验证环境是否存在
        if !config.environments.iter().any(|e| e.id == env_id) {
            return Err("Environment not found".to_string());
        }

        config.settings.current_environment_id = Some(env_id.to_string());
        self.save(&config).await?;
        Ok(())
    }
}
