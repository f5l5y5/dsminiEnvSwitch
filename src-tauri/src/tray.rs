use tauri::{
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu, CheckMenuItem},
    AppHandle, Manager, Runtime, Emitter,
    image::Image,
};
use crate::store::ConfigStore;

pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<TrayIcon<R>> {
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let show_item = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;

    let menu = Menu::with_items(
        app,
        &[&show_item, &separator, &quit_item],
    )?;

    // 加载并解码 PNG 图标
    let icon_data = include_bytes!("../icons/32x32.png");
    let img = image::load_from_memory(icon_data)
        .expect("Failed to load icon image");
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    let tray = TrayIconBuilder::new()
        .icon(Image::new_owned(
            rgba.into_raw(),
            width,
            height
        ))
        .menu(&menu)
        .on_menu_event(move |app, event| {
            handle_tray_event(app, event.id().as_ref());
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(tray)
}

fn handle_tray_event<R: Runtime>(app: &AppHandle<R>, event_id: &str) {
    match event_id {
        "quit" => {
            app.exit(0);
        }
        "show" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        id if id.starts_with("env_") => {
            let env_id = id.strip_prefix("env_").unwrap().to_string();
            let _ = app.emit("tray-switch-environment", env_id);
        }
        _ => {}
    }
}

pub async fn update_tray_menu<R: Runtime>(
    app: &AppHandle<R>,
    tray: &TrayIcon<R>,
) -> tauri::Result<()> {
    let store = app.state::<ConfigStore>();
    let config = store.load().await.map_err(|e| {
        tauri::Error::AssetNotFound(format!("Failed to load config: {}", e))
    })?;

    // 获取已应用的环境ID和名称
    let current_env_id = config.settings.current_applied_environment_id.as_ref().map(|s| s.as_str());
    let current_env_name = current_env_id
        .and_then(|id| config.environments.iter().find(|e| e.id == id))
        .map(|env| env.name.as_str());

    // 创建所有菜单项
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let show_item = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
    let separator1 = PredefinedMenuItem::separator(app)?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    let separator3 = PredefinedMenuItem::separator(app)?;

    // 当前环境显示项
    let current_env_item = MenuItem::with_id(
        app,
        "current_env",
        &format!("当前环境: {}", current_env_name.unwrap_or("未选择")),
        false,
        None::<&str>,
    )?;

    // 如果有环境,创建环境菜单项
    if !config.environments.is_empty() {
        let env_items: Vec<_> = config
            .environments
            .iter()
            .map(|env| {
                let is_selected = current_env_id == Some(env.id.as_str());
                CheckMenuItem::with_id(
                    app,
                    format!("env_{}", env.id),
                    &env.name,
                    true,
                    is_selected,
                    None::<&str>,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        let submenu_items: Vec<&dyn tauri::menu::IsMenuItem<R>> =
            env_items.iter().map(|item| item as &dyn tauri::menu::IsMenuItem<R>).collect();

        let env_submenu = Submenu::with_items(app, "切换环境", true, &submenu_items)?;

        let menu_items: Vec<&dyn tauri::menu::IsMenuItem<R>> = vec![
            &current_env_item,
            &separator1,
            &show_item,
            &separator2,
            &env_submenu,
            &separator3,
            &quit_item,
        ];

        let new_menu = Menu::with_items(app, &menu_items)?;
        tray.set_menu(Some(new_menu))?;
    } else {
        let menu_items: Vec<&dyn tauri::menu::IsMenuItem<R>> = vec![
            &current_env_item,
            &separator1,
            &show_item,
            &separator2,
            &quit_item,
        ];

        let new_menu = Menu::with_items(app, &menu_items)?;
        tray.set_menu(Some(new_menu))?;
    }

    // 设置托盘 tooltip
    let tooltip_text = if let Some(name) = current_env_name {
        format!("当前环境: {}", name)
    } else {
        "未选择环境".to_string()
    };
    tray.set_tooltip(Some(&tooltip_text))?;

    Ok(())
}
