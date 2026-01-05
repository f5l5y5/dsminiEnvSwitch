use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    #[serde(default = "default_id")]
    pub id: String,
    pub name: String,
    pub desc: Option<String>,
    pub ext: Value,
    #[serde(default)]
    pub target_file_path: Option<String>,  // 替换文件内容的路径(可选，单独设置时优先使用此值)
    #[serde(default = "default_timestamp")]
    pub created_at: String,
    #[serde(default = "default_timestamp")]
    pub updated_at: String,
}

fn default_id() -> String { String::new() }
fn default_timestamp() -> String { String::new() }

// 用于创建环境的输入类型（不含必需字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEnvironment {
    pub name: String,
    pub desc: Option<String>,
    pub ext: Value,
    #[serde(default)]
    pub target_file_path: Option<String>,  // 替换文件内容的路径(可选)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    #[serde(default = "default_close_to_tray")]
    pub close_to_tray: bool,
    #[serde(default)]
    pub current_environment_id: Option<String>,
    #[serde(default)]
    pub current_applied_environment_id: Option<String>,  // 当前应用的环境ID（点击"应用"按钮后记录）
    #[serde(default = "default_auto_backup")]
    pub auto_backup: bool,
    #[serde(default = "default_show_notifications")]
    pub show_notifications: bool,
    #[serde(default)]
    pub external_config_path: Option<String>,  // 外部配置文件路径
    #[serde(default)]
    pub target_file_path: Option<String>,      // 全局默认的替换文件内容路径
}

fn default_close_to_tray() -> bool { true }
fn default_auto_backup() -> bool { true }
fn default_show_notifications() -> bool { true }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub environments: Vec<Environment>,
    pub settings: AppSettings,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            environments: Vec::new(),
            settings: AppSettings {
                close_to_tray: true,
                current_environment_id: None,
                current_applied_environment_id: None,
                auto_backup: true,
                show_notifications: true,
                external_config_path: None,
                target_file_path: None,
            },
        }
    }
}
