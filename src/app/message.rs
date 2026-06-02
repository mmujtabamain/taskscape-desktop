use crate::app::hotkey::HotkeyCommand;
use crate::app::native_menu::NativeMenuCommand;
use crate::app::tray::TrayCommand;
use crate::ipc::IpcInbound;
use iced::{keyboard, window};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded,
    ToggleTheme,
    TitleChanged(String),
    FileNameChanged(String),
    ToggleTitleEdit,
    ToggleTitleEditCancel,
    CancelAllEditing,
    DueDateChanged(String),
    ToggleTaskCompleted(usize, bool),
    RemoveTask(usize),
    AddTask,
    ClearCompleted,
    ClearAll,
    FileNew,
    FileSave,
    FileLoad,
    FileSaveResult(Option<PathBuf>),
    FileLoadResult(Option<PathBuf>),
    EditUndo,
    EditRedo,
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    WindowCloseRequested(window::Id),
    NativeMenuEvent(NativeMenuCommand),
    NativeMenuInstalled(Result<(), String>),
    TrayEvent(TrayCommand),
    TrayInstalled(Result<(), String>),
    HotkeyEvent(HotkeyCommand),
    HotkeyInstalled(Result<(), String>),
    KeyboardEvent(keyboard::Event),
    /// A link event from the peer process (main app ⇄ tray service). Carries the
    /// inbound IPC event; the `update` loop interprets it based on which role
    /// this process plays (see `app::role`).
    IpcEvent(IpcInbound),
}
