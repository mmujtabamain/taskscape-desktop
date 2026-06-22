//! Shared code for the Taskscape apps.
//!
//! This crate is **not executable** — it holds the pieces both the main-window
//! app (`main_src`) and the tray service (`tray_src`) build on: the task
//! interfaces/models, the IPC link protocol and transport, the theme manager,
//! the generic widget toolkit, and shared utilities.

#![warn(unreachable_pub)]

pub mod attachments;
pub mod hotkey;
pub mod ipc;
pub mod models;
pub mod storage;
pub mod tasklist;
pub mod ui;
pub mod utils;

pub use models::Task;
pub use tasklist::TaskList;
