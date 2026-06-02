//! Auto-launches the background tray service so the user only runs one app.
//!
//! The tray service binds the IPC Unix socket, so its presence is detected by
//! attempting to connect to that socket. If nothing is listening, we launch the
//! tray — from the nested `.app` bundle in a packaged build, or from the sibling
//! `taskscape-tray` binary when running via `cargo run`.

use std::time::Duration;

/// Ensures the tray service is running, launching it if not. Best-effort: any
/// failure is logged and ignored (the main app still works standalone).
pub fn ensure_tray_running() {
    if tray_is_running() {
        return;
    }
    if let Err(error) = launch_tray() {
        eprintln!("Could not launch tray service: {error}");
    }
}

/// True if something is already listening on the IPC socket (i.e. the tray is up).
fn tray_is_running() -> bool {
    use std::os::unix::net::UnixStream;
    UnixStream::connect(common::ipc::socket_path()).is_ok()
}

#[cfg(target_os = "macos")]
fn launch_tray() -> Result<(), String> {
    use std::process::Command;

    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe_dir = exe
        .parent()
        .ok_or_else(|| String::from("no parent dir for current exe"))?;

    // Packaged layout:
    //   Taskscape.app/Contents/MacOS/taskscape  (this exe)
    //   Taskscape.app/Contents/Library/LoginItems/Taskscape Tray.app
    // exe_dir = .../Contents/MacOS  →  .../Contents
    if let Some(contents) = exe_dir.parent() {
        let tray_bundle = contents
            .join("Library/LoginItems/Taskscape Tray.app");
        if tray_bundle.exists() {
            Command::new("open")
                .arg(&tray_bundle)
                .spawn()
                .map_err(|e| format!("open {}: {e}", tray_bundle.display()))?;
            return Ok(());
        }
    }

    // Dev fallback: sibling `taskscape-tray` binary next to this one.
    let sibling = exe_dir.join("taskscape-tray");
    if sibling.exists() {
        Command::new(&sibling)
            .spawn()
            .map_err(|e| format!("spawn {}: {e}", sibling.display()))?;
        // Give the server a moment to bind before the app's IPC client races to
        // connect (the client retries anyway, so this is just to be tidy).
        std::thread::sleep(Duration::from_millis(150));
        return Ok(());
    }

    Err(String::from(
        "tray service not found (no bundled app or sibling binary)",
    ))
}

#[cfg(not(target_os = "macos"))]
fn launch_tray() -> Result<(), String> {
    // The tray service is only implemented on macOS for now.
    Ok(())
}
