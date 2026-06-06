//! Launches the main window app when the user asks to show it (e.g. by clicking
//! the mini window's list title) while it is closed.
//!
//! Mirrors `main_src`'s launcher in reverse: resolve the main `.app` bundle
//! relative to this (nested) tray executable, or fall back to the sibling
//! `taskscape` binary when running via `cargo run`.

/// Launches the main app. Best-effort: failures are logged and ignored.
pub fn launch_main() {
    if let Err(error) = try_launch_main() {
        eprintln!("Could not launch main app: {error}");
    }
}

#[cfg(target_os = "macos")]
fn try_launch_main() -> Result<(), String> {
    use std::process::Command;

    let exe = std::env::current_exe().map_err(|e| e.to_string())?;

    // Packaged layout (this exe is the nested tray binary):
    //   Taskscape.app/Contents/MacOS/taskscape                       (main)
    //   Taskscape.app/Contents/Library/LoginItems/Taskscape Tray.app/Contents/MacOS/taskscape-tray
    // Walk up from the tray exe to the outer `Taskscape.app` and `open` it.
    // .../Taskscape Tray.app/Contents/MacOS/taskscape-tray
    //  └─ ancestors: 1:MacOS → 2:Contents → 3:Taskscape Tray.app → 4:LoginItems
    //     → 5:Library → 6:Contents → 7:Taskscape.app
    if let Some(main_bundle) = exe.ancestors().nth(7) {
        if main_bundle.extension().and_then(|e| e.to_str()) == Some("app")
            && main_bundle.exists()
        {
            Command::new("open")
                .arg(main_bundle)
                .spawn()
                .map_err(|e| format!("open {}: {e}", main_bundle.display()))?;
            return Ok(());
        }
    }

    // Dev fallback: sibling `taskscape` binary next to this one.
    if let Some(dir) = exe.parent() {
        let sibling = dir.join("taskscape");
        if sibling.exists() {
            Command::new(&sibling)
                .spawn()
                .map_err(|e| format!("spawn {}: {e}", sibling.display()))?;
            return Ok(());
        }
    }

    Err(String::from(
        "main app not found (no bundled app or sibling binary)",
    ))
}

#[cfg(not(target_os = "macos"))]
fn try_launch_main() -> Result<(), String> {
    Ok(())
}
