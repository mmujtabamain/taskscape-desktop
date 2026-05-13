use crate::app::native_menu::NativeMenuCommand;
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
    NativeMenuEvent(NativeMenuCommand),
    NativeMenuInstalled(Result<(), String>),
    KeyboardEvent(keyboard::Event),
}
