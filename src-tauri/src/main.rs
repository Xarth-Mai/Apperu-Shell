#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mpris;
mod state;

use state::{IncomingPlayerState, PlayerState};
use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};
use tokio::sync::RwLock;

#[derive(Clone)]
struct AppShared {
    player: Arc<RwLock<PlayerState>>,
}

#[tauri::command]
async fn update_player_state(
    state: IncomingPlayerState,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let shared = app.state::<AppShared>().player.clone();
    let mut guard = shared.write().await;
    let old = guard.clone();
    *guard = state.into();

    if guard.title != old.title || guard.artist != old.artist {
        app.emit("track_changed", guard.clone())
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
async fn control(action: String, window: tauri::WebviewWindow) -> Result<(), String> {
    let script = format!(
        "window.apperuPlayer && window.apperuPlayer.control && window.apperuPlayer.control('{}');",
        action
    );
    window.eval(&script).map_err(|e| e.to_string())
}

fn main() {
    let shared = AppShared {
        player: Arc::new(RwLock::new(PlayerState::default())),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(shared.clone())
        .invoke_handler(tauri::generate_handler![update_player_state, control])
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let main_window = app.get_webview_window("main").expect("main window");

            let player_state = shared.player.clone();
            tauri::async_runtime::spawn(async move {
                let _ = mpris::init_mpris(player_state).await;
            });

            let show = MenuItem::with_id(app, "show", "显示/隐藏", true, None::<&str>)?;
            let play_pause = MenuItem::with_id(app, "playpause", "播放/暂停", true, None::<&str>)?;
            let next = MenuItem::with_id(app, "next", "下一首", true, None::<&str>)?;
            let prev = MenuItem::with_id(app, "prev", "上一首", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &play_pause, &next, &prev, &quit])?;

            let win_for_tray = main_window.clone();
            TrayIconBuilder::new()
                .menu(&menu)
                .on_tray_icon_event(move |_tray, event| {
                    if let TrayIconEvent::Click { .. } = event {
                        let _ = if win_for_tray.is_visible().unwrap_or(false) {
                            win_for_tray.hide()
                        } else {
                            win_for_tray.show()
                        };
                    }
                })
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = if win.is_visible().unwrap_or(false) {
                                win.hide()
                            } else {
                                win.show()
                            };
                        }
                    }
                    "playpause" | "next" | "prev" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let action = event.id().0.to_string();
                            let _ = win.eval(&format!(
                                "window.apperuPlayer && window.apperuPlayer.control && window.apperuPlayer.control('{}');",
                                action
                            ));
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            main_window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = app_handle
                        .get_webview_window("main")
                        .expect("main")
                        .hide();
                }
            });

            let js = include_str!("inject.js");
            main_window.eval(js)?;
            main_window.show()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
