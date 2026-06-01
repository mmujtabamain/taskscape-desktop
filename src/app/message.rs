use crate::app::native_menu::NativeMenuCommand;
use crate::app::tray::TrayCommand;
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
    WindowCloseRequested(window::Id),
    NativeMenuEvent(NativeMenuCommand),
    NativeMenuInstalled(Result<(), String>),
    TrayEvent(TrayCommand),
    TrayInstalled(Result<(), String>),
    KeyboardEvent(keyboard::Event),
}
