fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "unknown".to_string());

    if target_os != "linux" {
        panic!(
            "Apperu Shell only supports Linux targets. \
             Use a Linux target triple (for example x86_64-unknown-linux-gnu). \
             Current target_os: {target_os}"
        );
    }

    tauri_build::build()
}
