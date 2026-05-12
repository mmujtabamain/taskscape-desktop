use iced::Subscription;
use iced::futures::channel::mpsc;
use muda::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use std::cell::{Cell, RefCell};
use std::sync::Once;

#[cfg(target_os = "windows")]
use iced::window::raw_window_handle::{HasWindowHandle, RawWindowHandle};

const ID_FILE_NEW: &str = "taskscape.file.new";
const ID_FILE_SAVE: &str = "taskscape.file.save";
const ID_FILE_LOAD: &str = "taskscape.file.load";
const ID_EDIT_UNDO: &str = "taskscape.edit.undo";
const ID_EDIT_REDO: &str = "taskscape.edit.redo";
const ID_VIEW_TOGGLE_THEME: &str = "taskscape.view.toggle-theme";

thread_local! {
    static MENU_HANDLE: RefCell<Option<Menu>> = const { RefCell::new(None) };
    static MENU_INSTALLED: Cell<bool> = const { Cell::new(false) };
}

// Ensures the event stream thread is only spawned once across subscription re-runs.
static STREAM_STARTED: Once = Once::new();

#[derive(Debug, Clone, Copy)]
pub enum NativeMenuCommand {
    FileNew,
    FileSave,
    FileLoad,
    EditUndo,
    EditRedo,
    ToggleTheme,
}

#[cfg(target_os = "windows")]
fn install_menu(window: &dyn HasWindowHandle) -> Result<(), String> {
    if is_installed() {
        return Ok(());
    }

    let menu = build_menu().map_err(|e| e.to_string())?;

    #[cfg(target_os = "macos")]
    menu.init_for_nsapp();

    #[cfg(target_os = "windows")]
    {
        let handle = window
            .window_handle()
            .map_err(|e| format!("window handle error: {e}"))?;
        match handle.as_raw() {
            RawWindowHandle::Win32(h) => unsafe {
                menu.init_for_hwnd(h.hwnd.get())
                    .map_err(|e| format!("init_for_hwnd failed: {e}"))?;
            },
            other => return Err(format!("unsupported handle type: {other:?}")),
        }
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    return Err(String::from(
        "Native top-menu on Linux requires a GTK window handle not exposed by iced/winit.",
    ));

    MENU_HANDLE.with(|slot| *slot.borrow_mut() = Some(menu));
    MENU_INSTALLED.with(|flag| flag.set(true));
    Ok(())
}

/// On Windows the HWND is needed; on macOS the window handle is unused.
#[cfg(target_os = "windows")]
pub fn install_for_window(window: &dyn HasWindowHandle) -> Result<(), String> {
    install_menu(window)
}

#[cfg(not(target_os = "windows"))]
pub fn install_for_window(
    _window: &dyn iced::window::raw_window_handle::HasWindowHandle,
) -> Result<(), String> {
    // No-op on macOS — menu is installed in subscription() before the stream starts.
    Ok(())
}

/// Returns a subscription that delivers native menu events as [`NativeMenuCommand`]s.
/// On macOS the menu is installed immediately (no window handle needed).
pub fn subscription() -> Subscription<NativeMenuCommand> {
    // macOS: install menu as soon as subscriptions are evaluated.
    #[cfg(target_os = "macos")]
    {
        if !is_installed() {
            println!("Installing macOS menu...");
            if let Ok(menu) = build_menu() {
                menu.init_for_nsapp();
                println!("macOS menu installed successfully");
                MENU_INSTALLED.with(|flag| flag.set(true));
            } else {
                eprintln!("Failed to build menu!");
            }
        }
    }

    Subscription::run(menu_event_stream)
}

/// Builds a stream of menu commands without blocking the async executor.
/// Spawns a single OS thread that performs the blocking recv and forwards
/// events via a non-blocking try_send.
fn menu_event_stream() -> impl iced::futures::Stream<Item = NativeMenuCommand> {
    let (mut tx, rx) = mpsc::channel::<NativeMenuCommand>(64);

    STREAM_STARTED.call_once(|| {
        std::thread::Builder::new()
            .name("taskscape-menu-events".into())
            .spawn(move || {
                let receiver = MenuEvent::receiver();
                loop {
                    match receiver.recv() {
                        Ok(event) => {
                            if let Some(cmd) = map_event_to_command(&event) {
                                // try_send is non-blocking; drops the event if buffer is full.
                                let _ = tx.try_send(cmd);
                            }
                        }
                        Err(_) => break,
                    }
                }
            })
            .expect("failed to spawn menu event thread");
    });

    rx
}

fn map_event_to_command(event: &MenuEvent) -> Option<NativeMenuCommand> {
    match event.id.as_ref() {
        ID_FILE_NEW => Some(NativeMenuCommand::FileNew),
        ID_FILE_SAVE => Some(NativeMenuCommand::FileSave),
        ID_FILE_LOAD => Some(NativeMenuCommand::FileLoad),
        ID_EDIT_UNDO => Some(NativeMenuCommand::EditUndo),
        ID_EDIT_REDO => Some(NativeMenuCommand::EditRedo),
        ID_VIEW_TOGGLE_THEME => Some(NativeMenuCommand::ToggleTheme),
        _ => None,
    }
}

fn is_installed() -> bool {
    MENU_INSTALLED.with(Cell::get)
}

fn build_menu() -> muda::Result<Menu> {
    let file_new = MenuItem::with_id(ID_FILE_NEW, "New List", true, None);
    let file_save = MenuItem::with_id(ID_FILE_SAVE, "Save CSV…", true, None);
    let file_load = MenuItem::with_id(ID_FILE_LOAD, "Load CSV…", true, None);
    let sep = PredefinedMenuItem::separator();

    let edit_undo = MenuItem::with_id(ID_EDIT_UNDO, "Undo", true, None);
    let edit_redo = MenuItem::with_id(ID_EDIT_REDO, "Redo", true, None);

    let view_theme = MenuItem::with_id(ID_VIEW_TOGGLE_THEME, "Toggle Theme", true, None);

    let file_menu = Submenu::with_items("File", true, &[&file_new, &file_save, &file_load, &sep])?;
    let edit_menu = Submenu::with_items("Edit", true, &[&edit_undo, &edit_redo])?;
    let view_menu = Submenu::with_items("View", true, &[&view_theme])?;

    Menu::with_items(&[&file_menu, &edit_menu, &view_menu])
}
