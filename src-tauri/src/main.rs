#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod scan;
mod window_style;
mod ignore;

use regex::Regex;
use serde::Serialize;
use std::process::Command;
use std::sync::{Arc, Mutex};
use sysinfo::{DiskExt, System, SystemExt};
use tauri::api::process::CommandChild;
use tauri::Manager;
use crate::ignore::{IgnoreConfig, IgnorePattern};

#[cfg(target_os = "macos")]
use window_vibrancy::NSVisualEffectMaterial;

#[cfg(target_os = "linux")]
use {std::fs::metadata, std::path::PathBuf};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SquirrelDisk<'a> {
    name: &'a str,
    s_mount_point: String,
    total_space: u64,
    available_space: u64,
    is_removable: bool,
}

fn main() {
    tauri::Builder::default()
        .manage(MyState(Default::default()))
        .setup(|app| {
            // Load ignore config
            let config = IgnoreConfig::load(app.config().as_ref())
                .unwrap_or_default();
            app.manage(IgnoreState(Arc::new(Mutex::new(config))));
            
            let window = app.get_window("main").unwrap();
            // window.open_devtools();
            #[cfg(target_os = "macos")]
            window_vibrancy::apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Error applying blurred bg");

            #[cfg(target_os = "windows")]
            window_vibrancy::apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Error applying blurred bg");

            #[cfg(any(windows, target_os = "macos"))]
            window_style::set_window_styles(&window).unwrap();

            // app.listen_global("scan_stop", |event| {
            //     let s = app.state::<MyState>();
            //     s.0.lock().unwrap().take().unwrap().kill();
            // });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_disks,
            start_scanning,
            stop_scanning,
            show_in_folder,
            get_ignore_patterns,
            add_ignore_pattern,
            remove_ignore_pattern,
            toggle_ignore_pattern
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn show_in_folder(path: String) {
    #[cfg(target_os = "windows")]
    {
        let re = Regex::new(r"/").unwrap();
        let result = re.replace_all(&path, "\\");
        Command::new("explorer")
            .args(["/select,", format!("{}", result).as_str()]) // The comma after select is not a typo
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        // if path.contains(",") {
        // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
        let new_path = match metadata(&path).unwrap().is_dir() {
            true => path,
            false => {
                let mut path2 = PathBuf::from(path);
                path2.pop();
                path2.into_os_string().into_string().unwrap()
            }
        };
        Command::new("xdg-open").arg(&new_path).spawn().unwrap();
        // } else {
        //     Command::new("dbus-send")
        //         .args([
        //             "--session",
        //             "--dest=org.freedesktop.FileManager1",
        //             "--type=method_call",
        //             "/org/freedesktop/FileManager1",
        //             "org.freedesktop.FileManager1.ShowItems",
        //             format!("array:string:\"file://{path}\"").as_str(),
        //             "string:\"\"",
        //         ])
        //         .spawn()
        //         .unwrap();
        // }
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").args(["-R", &path]).spawn().unwrap();
    }
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_disks() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut vec: Vec<SquirrelDisk> = Vec::new();

    for disk in sys.disks() {
        vec.push(SquirrelDisk {
            name: disk.name().to_str().unwrap(),
            s_mount_point: disk.mount_point().display().to_string(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            is_removable: disk.is_removable(),
        });
    }
    serde_json::to_string(&vec).unwrap().into()
}

pub struct MyState(Mutex<Option<CommandChild>>);
pub struct IgnoreState(Arc<Mutex<IgnoreConfig>>);

#[tauri::command]
fn start_scanning(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, MyState>,
    path: String,
    ratio: String,
) -> Result<(), ()> {
    scan::start(app_handle, state, path, ratio)
}

#[tauri::command]
fn stop_scanning(
    _app_handle: tauri::AppHandle,
    state: tauri::State<'_, MyState>,
    _path: String,
) -> Result<(), ()> {
    scan::stop(state);
    Ok(())
}

#[tauri::command]
fn get_ignore_patterns(
    ignore_state: tauri::State<'_, IgnoreState>,
) -> Result<Vec<IgnorePattern>, String> {
    let config = ignore_state.0.lock()
        .map_err(|e| format!("Failed to lock ignore state: {}", e))?;
    Ok(config.patterns.clone())
}

#[tauri::command]
fn add_ignore_pattern(
    app_handle: tauri::AppHandle,
    ignore_state: tauri::State<'_, IgnoreState>,
    pattern: String,
) -> Result<(), String> {
    let mut config = ignore_state.0.lock()
        .map_err(|e| format!("Failed to lock ignore state: {}", e))?;
    
    // Check if pattern already exists
    if config.patterns.iter().any(|p| p.pattern == pattern) {
        return Err("Pattern already exists".to_string());
    }
    
    config.patterns.push(IgnorePattern {
        pattern,
        enabled: true,
    });
    
    config.save(app_handle.config().as_ref())?;
    Ok(())
}

#[tauri::command]
fn remove_ignore_pattern(
    app_handle: tauri::AppHandle,
    ignore_state: tauri::State<'_, IgnoreState>,
    pattern: String,
) -> Result<(), String> {
    let mut config = ignore_state.0.lock()
        .map_err(|e| format!("Failed to lock ignore state: {}", e))?;
    
    config.patterns.retain(|p| p.pattern != pattern);
    config.save(app_handle.config().as_ref())?;
    Ok(())
}

#[tauri::command]
fn toggle_ignore_pattern(
    app_handle: tauri::AppHandle,
    ignore_state: tauri::State<'_, IgnoreState>,
    pattern: String,
) -> Result<(), String> {
    let mut config = ignore_state.0.lock()
        .map_err(|e| format!("Failed to lock ignore state: {}", e))?;
    
    if let Some(p) = config.patterns.iter_mut().find(|p| p.pattern == pattern) {
        p.enabled = !p.enabled;
        config.save(app_handle.config().as_ref())?;
        Ok(())
    } else {
        Err("Pattern not found".to_string())
    }
}
