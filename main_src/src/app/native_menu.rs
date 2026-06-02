use iced::Subscription;
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use muda::{Menu, MenuEvent, MenuItem, Submenu};
use std::cell::{Cell, RefCell};

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

#[derive(Debug, Clone, Copy)]
pub enum NativeMenuCommand {
    FileNew,
    FileSave,
    FileLoad,
    EditUndo,
    EditRedo,
    ToggleTheme,
}

#[cfg(target_os = "macos")]
fn install_menu() -> Result<(), String> {
    if is_installed() {
        return Ok(());
    }

    let menu = build_menu().map_err(|e| e.to_string())?;
    menu.init_for_nsapp();

    // Keep the menu alive for the app lifetime.
    MENU_HANDLE.with(|slot| *slot.borrow_mut() = Some(menu));
    MENU_INSTALLED.with(|flag| flag.set(true));
    Ok(())
}

#[cfg(target_os = "windows")]
fn install_menu(window: &dyn HasWindowHandle) -> Result<(), String> {
    if is_installed() {
        return Ok(());
    }

    let menu = build_menu().map_err(|e| e.to_string())?;

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

#[cfg(target_os = "macos")]
pub fn install_for_window(
    _window: &dyn iced::window::raw_window_handle::HasWindowHandle,
) -> Result<(), String> {
    install_menu()
}

#[cfg(not(target_os = "windows"))]
#[cfg(not(target_os = "macos"))]
pub fn install_for_window(
    _window: &dyn iced::window::raw_window_handle::HasWindowHandle,
) -> Result<(), String> {
    Err(String::from(
        "Native menu is not supported on this platform in iced/winit yet. Keyboard shortcuts remain available.",
    ))
}

/// Returns a subscription that delivers native menu events as [`NativeMenuCommand`]s.
#[cfg(any(target_os = "macos", target_os = "windows"))]
pub fn subscription() -> Subscription<NativeMenuCommand> {
    Subscription::run(menu_event_stream)
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn subscription() -> Subscription<NativeMenuCommand> {
    Subscription::none()
}

/// Builds a stream of menu commands without blocking the async executor.
/// A dedicated forwarding thread is tied to the stream receiver lifetime.
fn menu_event_stream() -> impl iced::futures::Stream<Item = NativeMenuCommand> {
    let (tx, rx) = mpsc::channel::<NativeMenuCommand>(64);

    if let Err(error) = std::thread::Builder::new()
        .name("taskscape-menu-events".into())
        .spawn(move || {
            let mut tx = tx;
            let receiver = MenuEvent::receiver();

            while let Ok(event) = receiver.recv() {
                if let Some(cmd) = map_event_to_command(&event) {
                    // Block this forwarding thread when needed so we do not drop events.
                    if iced::futures::executor::block_on(tx.send(cmd)).is_err() {
                        // The subscription stream was dropped.
                        break;
                    }
                }
            }
        })
    {
        eprintln!("Failed to spawn native menu event thread: {error}");
    }

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
    let menu: Menu = Menu::new();

    #[cfg(target_os = "macos")]
    {
        use muda::PredefinedMenuItem;

        let app_menu = Submenu::with_items(
            "",
            true,
            &[
                &PredefinedMenuItem::about(None, None),
                &PredefinedMenuItem::separator(),
                &PredefinedMenuItem::services(None),
                &PredefinedMenuItem::separator(),
                &PredefinedMenuItem::hide(None),
                &PredefinedMenuItem::hide_others(None),
                &PredefinedMenuItem::show_all(None),
                &PredefinedMenuItem::separator(),
                &PredefinedMenuItem::quit(None),
            ],
        )?;

        menu.append(&app_menu)?;
    }

    let file_new = MenuItem::with_id(ID_FILE_NEW, "New List", true, None);
    let file_save = MenuItem::with_id(ID_FILE_SAVE, "Export List…", true, None);
    let file_load = MenuItem::with_id(ID_FILE_LOAD, "Import List…", true, None);

    let edit_undo = MenuItem::with_id(ID_EDIT_UNDO, "Undo", true, None);
    let edit_redo = MenuItem::with_id(ID_EDIT_REDO, "Redo", true, None);

    let view_theme = MenuItem::with_id(ID_VIEW_TOGGLE_THEME, "Toggle Theme", true, None);

    let file_menu = Submenu::with_items("File", true, &[&file_new, &file_save, &file_load])?;
    let edit_menu = Submenu::with_items("Edit", true, &[&edit_undo, &edit_redo])?;
    let view_menu = Submenu::with_items("View", true, &[&view_theme])?;

    menu.append(&file_menu)?;
    menu.append(&edit_menu)?;
    menu.append(&view_menu)?;

    Ok(menu)
}
