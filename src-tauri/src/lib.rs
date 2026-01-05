mod models;
mod store;
mod commands;
mod tray;

use tauri::{Manager, WindowEvent};
use store::ConfigStore;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let config_store = ConfigStore::new(app.handle().clone());
            app.manage(config_store);

            // 创建系统托盘并保存引用
            let tray = tray::create_tray(app.handle())?;
            app.manage(Arc::new(Mutex::new(Some(tray))));

            // 在后台加载配置并更新缓存（在托盘创建之后）
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // 获取 ConfigStore 并加载配置以更新缓存
                let store = app_handle.state::<ConfigStore>();
                let _ = store.load().await;

                // 配置加载完成后更新托盘菜单
                let tray_state = app_handle.state::<Arc<Mutex<Option<tauri::tray::TrayIcon>>>>();
                if let Ok(tray_guard) = tray_state.try_lock() {
                    if let Some(tray_icon) = tray_guard.as_ref() {
                        let _ = tray::update_tray_menu(&app_handle, tray_icon).await;
                    }
                }
                drop(tray_state);
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // 从缓存读取关闭行为设置
                let app = window.app_handle();
                let store = app.state::<ConfigStore>();

                // 使用阻塞方式获取配置
                let rt = tauri::async_runtime::handle();
                let should_hide = rt.block_on(async {
                    store.get_close_to_tray().await
                });

                if should_hide {
                    // 启用了"最小化到托盘"，隐藏窗口
                    api.prevent_close();
                    let _ = window.hide();
                }
                // 否则允许窗口正常关闭，程序会退出
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::config::load_config,
            commands::config::save_config,
            commands::config::import_config_from_json,
            commands::config::export_config_to_json,
            commands::config::add_environment,
            commands::config::update_environment,
            commands::config::delete_environment,
            commands::config::switch_environment,
            commands::files::apply_environment,
            commands::files::read_file,
            commands::files::select_file_path,
            commands::files::select_folder_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
