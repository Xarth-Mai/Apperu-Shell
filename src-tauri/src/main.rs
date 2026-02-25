mod mpris;
mod state;

use state::{IncomingPlayerState, PlayerState};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};
use tokio::sync::RwLock;

use tauri_plugin_notification::NotificationExt;

const MAIN_WINDOW_LABEL: &str = "main";
const MAIN_TRAY_ID: &str = "main-tray";

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

fn has_gstreamer_element(element: &str) -> bool {
    use std::{
        env,
        path::PathBuf,
        process::{Command, Stdio},
    };

    if let Ok(status) = Command::new("gst-inspect-1.0")
        .arg(element)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        if status.success() {
            return true;
        }
    }

    let mut dirs: Vec<PathBuf> = Vec::new();

    for var in ["GST_PLUGIN_SYSTEM_PATH", "GST_PLUGIN_PATH"] {
        if let Ok(value) = env::var(var) {
            dirs.extend(env::split_paths(&value));
        }
    }

    dirs.extend(
        [
            "/usr/lib/gstreamer-1.0",
            "/usr/lib64/gstreamer-1.0",
            "/usr/local/lib/gstreamer-1.0",
            "/usr/lib/x86_64-linux-gnu/gstreamer-1.0",
            "/usr/lib/aarch64-linux-gnu/gstreamer-1.0",
            "/app/lib/gstreamer-1.0",
            "/app/lib64/gstreamer-1.0",
        ]
        .into_iter()
        .map(PathBuf::from),
    );

    dirs.sort();
    dirs.dedup();

    match element {
        "autoaudiosink" => dirs
            .into_iter()
            .any(|dir| dir.join("libgstautodetect.so").exists()),
        _ => false,
    }
}

fn warn_if_missing_gstreamer_plugins<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    if has_gstreamer_element("autoaudiosink") {
        return;
    }

    let title = "Missing GStreamer plugins";
    let body = "Audio playback may fail (autoaudiosink not found). Install GStreamer plugins (plugins-good / autodetect).";

    eprintln!("[Apperu Shell] {title}: {body}");
    let _ = app.notification().builder().title(title).body(body).show();
}

fn has_widevine_cdm() -> bool {
    use std::{env, path::PathBuf};

    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Ok(path) = env::var("WIDEVINE_CDM_PATH") {
        candidates.push(PathBuf::from(path));
    }

    candidates.extend(
        [
            "/usr/lib/chromium/libwidevinecdm.so",
            "/usr/lib64/chromium/libwidevinecdm.so",
            "/usr/lib/chromium-browser/libwidevinecdm.so",
            "/opt/google/chrome/WidevineCdm/_platform_specific/linux_x64/libwidevinecdm.so",
            "/opt/google/chrome-beta/WidevineCdm/_platform_specific/linux_x64/libwidevinecdm.so",
            "/opt/google/chrome-unstable/WidevineCdm/_platform_specific/linux_x64/libwidevinecdm.so",
            "/usr/lib/mozilla/plugins/libwidevinecdm.so",
        ]
        .into_iter()
        .map(PathBuf::from),
    );

    candidates.into_iter().any(|path| path.exists())
}

fn warn_if_missing_widevine<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    if has_widevine_cdm() {
        return;
    }

    let title = "Widevine CDM not found";
    let body = "Apple Music login may succeed, but playback can be limited to previews. Install Widevine CDM and restart the app.";

    eprintln!("[Apperu Shell] {title}: {body}");
    let _ = app.notification().builder().title(title).body(body).show();
}

fn request_app_exit<R: tauri::Runtime>(app: &tauri::AppHandle<R>, is_quitting: &Arc<AtomicBool>) {
    if is_quitting.swap(true, Ordering::SeqCst) {
        return;
    }

    let _ = app.remove_tray_by_id(MAIN_TRAY_ID);
    if let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        let _ = window.close();
    } else {
        app.exit(0);
    }
}

fn main() {
    let shared = AppShared {
        player: Arc::new(RwLock::new(PlayerState::default())),
    };
    let js_on_load = include_str!("inject.js").to_string();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .on_page_load(move |webview, _payload| {
            let _ = webview.eval(&js_on_load);
        })
        .manage(shared.clone())
        .invoke_handler(tauri::generate_handler![update_player_state, control])
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let main_window = app
                .get_webview_window(MAIN_WINDOW_LABEL)
                .expect("main window");
            let is_quitting = Arc::new(AtomicBool::new(false));
            let product_name = app
                .config()
                .product_name
                .clone()
                .unwrap_or_else(|| "Apperu Shell".to_string());

            main_window.with_webview(|webview| {
                use webkit2gtk::glib::prelude::Cast;
                use webkit2gtk::{
                    MediaKeySystemPermissionRequest, PermissionRequestExt, SettingsExt, WebViewExt,
                };

                let native_webview = webview.inner();
                if let Some(settings) = native_webview.settings() {
                    settings.set_enable_encrypted_media(true);
                    settings.set_enable_mediasource(true);
                    settings.set_enable_media_capabilities(true);
                } else {
                    eprintln!(
                        "[Apperu Shell] WebKit settings unavailable; EME settings were not applied."
                    );
                }

                native_webview.connect_permission_request(|_, request| {
                    if request
                        .dynamic_cast_ref::<MediaKeySystemPermissionRequest>()
                        .is_some()
                    {
                        request.allow();
                        return true;
                    }
                    false
                });
            })?;

            if let Err(err) = main_window.reload() {
                eprintln!("[Apperu Shell] initial page reload failed: {err}");
            }

            warn_if_missing_gstreamer_plugins(app.handle());
            warn_if_missing_widevine(app.handle());

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
            let is_quitting_for_menu = is_quitting.clone();
            TrayIconBuilder::with_id(MAIN_TRAY_ID)
                .icon(tauri::include_image!("./icons/icon.png"))
                .title(&product_name)
                .tooltip(&product_name)
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
                        if let Some(win) = app.get_webview_window(MAIN_WINDOW_LABEL) {
                            let _ = if win.is_visible().unwrap_or(false) {
                                win.hide()
                            } else {
                                win.show()
                            };
                        }
                    }
                    "playpause" | "next" | "prev" => {
                        if let Some(win) = app.get_webview_window(MAIN_WINDOW_LABEL) {
                            let action = event.id().0.to_string();
                            let _ = win.eval(&format!(
                                "window.apperuPlayer && window.apperuPlayer.control && window.apperuPlayer.control('{}');",
                                action
                            ));
                        }
                    }
                    "quit" => {
                        request_app_exit(app, &is_quitting_for_menu);
                    }
                    _ => {}
                })
                .build(app)?;

            let is_quitting_for_close = is_quitting.clone();
            main_window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    if is_quitting_for_close.load(Ordering::Relaxed) {
                        return;
                    }
                    api.prevent_close();
                    let _ = app_handle
                        .get_webview_window(MAIN_WINDOW_LABEL)
                        .expect("main")
                        .hide();
                }
            });

            main_window.show()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
