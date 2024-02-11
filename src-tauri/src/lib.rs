// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::{command, start_events};
use tauri::{
    menu::{Menu, MenuItem},
    AppHandle, Manager,
};
use tauri_plugin_log::{Target, TargetKind};

fn get_maximized_menu(app: &AppHandle) -> Menu<tauri::Wry> {
    Menu::with_items(
        app,
        &[&MenuItem::with_id(app, "quit", "&Quit", true, None::<&str>).unwrap()],
    )
    .unwrap()
}

fn get_minimized_menu(app: &AppHandle) -> Menu<tauri::Wry> {
    Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, "show", "&Show", true, None::<&str>).unwrap(),
            &MenuItem::with_id(app, "quit", "&Quit", true, None::<&str>).unwrap(),
        ],
    )
    .unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .clear_targets()
                .targets([
                    Target::new(TargetKind::Webview),
                    Target::new(TargetKind::Stdout),
                ])
                .level(log::LevelFilter::Info)
                .build(),
        )
        .setup(move |app| {
            let handle = app.handle();
            let menu = get_maximized_menu(handle);
            let _tray = tauri::tray::TrayIconBuilder::with_id("tray_1")
                .icon(tauri::Icon::File("icons/icon_none.png".into()))
                .menu(&menu)
                .on_menu_event(|app, event| {
                    match event.id().as_ref() {
                        "show" => {
                            app.get_window("main")
                                .unwrap()
                                .show()
                                .expect("To show the window");
                            app.tray()
                                .unwrap()
                                .set_menu(Some(get_maximized_menu(app)))
                                .unwrap();
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![command, start_events])
        .build(tauri::generate_context!())
        .expect("To build tauri app");
    app.hide_menu().unwrap();
    app.run(|app, event| {
        if let tauri::RunEvent::WindowEvent {
            event: tauri::WindowEvent::CloseRequested { api, .. },
            ..
        } = event
        {
            app.tray()
                .unwrap()
                .set_menu(Some(get_minimized_menu(app)))
                .unwrap();
            api.prevent_close();
            app.get_window("main")
                .unwrap()
                .hide()
                .expect("To hide the window");
        }
    });
}

