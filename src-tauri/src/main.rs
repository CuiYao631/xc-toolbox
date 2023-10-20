#![cfg_attr(not(debug_assertions), windows_subsystem = "Windows")]

use tauri::{
    App, AppHandle, CustomMenuItem, GlobalShortcutManager, Manager,
     SystemTray, SystemTrayEvent,SystemTrayMenu, SystemTrayMenuItem,
};

fn main() {
    let hide = CustomMenuItem::new("hide".to_string(), "显示窗口");
    let seting = CustomMenuItem::new("seting".to_string(), "设置");
    let about = CustomMenuItem::new("about".to_string(), "关于");
    let quit = CustomMenuItem::new("quit".to_string(), "退出软件");

    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(seting)
        .add_item(about)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |app| {
            if let Err(e) = register_shortcut(app) {
                eprintln!("error registering shortcut: {}", e);
            }
            // app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
            menu_hanfle(app, event);
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
//注册快捷键
const SHORTHAND: &str = "alt+space";
fn register_shortcut(app: &mut App) -> Result<(), tauri::Error> {
    let app_habdle = app.app_handle();
    let mut shortcuts = app_habdle.global_shortcut_manager();
    if !shortcuts.is_registered(SHORTHAND)? {
        shortcuts.register(SHORTHAND, move || toggle_launchpad(&app_habdle))?;
    }
    Ok(())
}

//启动面板
const WINDOW: &str = "launch";
fn toggle_launchpad(app: &AppHandle) {
    let window = app.get_window(WINDOW).expect("window not found");
    if let Ok(true) = window.is_visible() {
        let _ = window.hide();
    } else {
        let _ = window.show();
    }
}
//接收前端指令
#[tauri::command]
fn greet(window: tauri::Window, name: &str) {
    println!("Hello, {}!", name);
    toggle_launchpad(&window.app_handle());
}

//系统托盘
fn menu_hanfle(app: &AppHandle, event: SystemTrayEvent) {
    let window = app.get_window(WINDOW).expect("window not found");
    match event {
        // SystemTrayEvent::LeftClick { position, size, .. } => {
        //     println!("系统托盘左击事件");
        // }
        // SystemTrayEvent::RightClick { position, size, .. } => {
        //     println!("系统托盘右击事件");
        // }
        // SystemTrayEvent::DoubleClick { position, size, .. } => {
        //     println!("系统托盘双击事件");
        // }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app.tray_handle().get_item(&id);
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    if let Ok(true) = window.is_visible() {
                        let _ = window.hide();
                        let _ = item_handle.set_title("显示窗口");
                    } else {
                        let _ = window.show();
                        let _ = item_handle.set_title("隐藏窗口");
                    }
                }
                "seting" => {
                    seting(app);
                }
                "about" => {
                    about(app);
                }
                _ => {}
            }
        }
        _ => {}
    }
}

//设置窗口
fn seting(app: &AppHandle) {
    let _ = tauri::WindowBuilder::new(
        app,
        "seting",
        tauri::WindowUrl::App("/seting".into())
      ).
    title("设置").
      skip_taskbar(false).
      //transparent(true).
      resizable(false).
      fullscreen(false).
      always_on_top(true).
      inner_size(400.0, 300.0).
      build().unwrap();
}

//about窗口
fn about(app: &AppHandle) {
    let _ = tauri::WindowBuilder::new(
        app,
        "about",
        tauri::WindowUrl::App("/about".into())
      ).
    title("关于").
      skip_taskbar(false).
      //transparent(true).
      resizable(false).
      fullscreen(false).
      always_on_top(true).
      inner_size(400.0, 300.0).
      build().unwrap();
}


 
