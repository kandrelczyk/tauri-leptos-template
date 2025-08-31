#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    AppHandle, Manager,
};
use tauri_plugin_updater::UpdaterExt;

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

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        // alternatively we could also call update.download() and update.install() separately
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    }

    Ok(())
}

fn main() {
    let app = app_lib::AppBuilder::new()
        .setup(move |app| {
            let handle = app.handle();
            let update_handle = handle.clone();
            tauri::async_runtime::spawn(async move {
                update(update_handle).await.unwrap();
            });
            let menu = get_maximized_menu(handle);
            let _tray = tauri::tray::TrayIconBuilder::with_id("tray_1")
                .icon(tauri::image::Image::from_bytes(include_bytes!(
                    "../icons/32x32.png"
                ))?)
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        app.get_window("main")
                            .unwrap()
                            .show()
                            .expect("To show the window");
                        app.tray_by_id("tray_1")
                            .unwrap()
                            .set_menu(Some(get_maximized_menu(app)))
                            .unwrap();
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .build_app();
    app.hide_menu().unwrap();
    app.run(|app, event| {
        if let tauri::RunEvent::WindowEvent {
            event: tauri::WindowEvent::CloseRequested { api, .. },
            ..
        } = event
        {
            api.prevent_close();
            app.tray_by_id("tray_1")
                .unwrap()
                .set_menu(Some(get_minimized_menu(app)))
                .unwrap();
            app.get_window("main")
                .unwrap()
                .hide()
                .expect("To hide the window");
        }
    });
}
