#![cfg_attr(not(debug_assertions), windows_subsystem = "Windows")]

use tauri::{
    App, AppHandle, CustomMenuItem, GlobalShortcutManager, Manager,
     SystemTray, SystemTrayEvent,SystemTrayMenu, SystemTrayMenuItem,
};
use serde::{Deserialize, Serialize};
use sysinfo::{System, SystemExt, CpuExt};

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
        .invoke_handler(tauri::generate_handler![
            greet,//接收前端指令
            send_data_to_frontend,//发送数据给前端
            system_info//获取系统信息
            ])
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
const WINDOW: &str = "home";
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
      inner_size(800.0, 600.0).
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
      transparent(true).
      resizable(false).
      fullscreen(false).
      always_on_top(true).
      inner_size(400.0, 600.0).
      build().unwrap();
}

#[tauri::command]
fn send_data_to_frontend(window: tauri::Window, data: String) {
    // 发送数据给前端
    let _=window.emit(
        "dataFromRust",  // 与前端 JavaScript 代码匹配的事件名称
        Some(data),       // 发送的数据
    );
}

#[derive(Serialize, Deserialize)]
struct SystemInfo {
    //操作系统
    os: String,
    //cpu型号
    cpu_name: String,
    //总内存
    memory_total: f32,
    //磁盘大小
    // disk_total: f32,
 
}

//获取系统信息
#[tauri::command]
fn system_info()->SystemInfo{
    //获取系统信息
    let mut sys = sysinfo::System::new_all();
    //刷新系统信息
    sys.refresh_all();
    let os = sys.long_os_version().unwrap();
    let cpu_name = sys.global_cpu_info().name().to_string();
    let memory_total = sys.total_memory() as f32 / 1024.0 / 1024.0 / 1024.0;
    //获取磁盘列表
    // let mut s = System::new();
    // s.refresh_disks_list();
    // for disk in s.disks() {
    //     println!("{:?}", disk);
    // }
   

    SystemInfo{
        os,
        cpu_name,
        memory_total,
    }
}


 
