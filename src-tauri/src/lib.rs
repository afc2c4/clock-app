use std::sync::Mutex;

// ─── Keep-awake state ────────────────────────────────────────────────────────

pub struct KeepAwakeState(pub Mutex<Option<std::process::Child>>);

/// Spawns a background OS process that holds a sleep/idle inhibitor lock.
/// On Windows this is a no-op (the Screen Wake Lock API handles it in JS).
#[tauri::command]
fn start_keep_awake(state: tauri::State<KeepAwakeState>) -> Result<(), String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    if guard.is_some() {
        return Ok(()); // already active
    }

    #[cfg(target_os = "linux")]
    let spawn_result = std::process::Command::new("systemd-inhibit")
        .args([
            "--what=sleep:idle",
            "--who=Despertador",
            "--why=Alarme ativo",
            "--mode=block",
            "sleep",
            "infinity",
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();

    #[cfg(target_os = "macos")]
    let spawn_result = std::process::Command::new("caffeinate")
        .arg("-i")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();

    // Windows: handled entirely by the Screen Wake Lock API in the frontend.
    #[cfg(target_os = "windows")]
    let spawn_result: Result<std::process::Child, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "no-op",
    ));

    if let Ok(child) = spawn_result {
        *guard = Some(child);
    }
    Ok(())
}

/// Releases the inhibitor lock and lets the OS manage power normally.
#[tauri::command]
fn stop_keep_awake(state: tauri::State<KeepAwakeState>) -> Result<(), String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    if let Some(mut child) = guard.take() {
        let _ = child.kill();
        let _ = child.wait();
    }
    Ok(())
}

// ─── Original greet command (kept for reference) ─────────────────────────────

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// ─── App entry point ──────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(KeepAwakeState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            greet,
            start_keep_awake,
            stop_keep_awake
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
