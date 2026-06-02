pub mod actions;
pub mod application;
pub mod hotkey;
pub mod main_app;
pub mod message;
pub mod native_menu;
pub mod queries;
pub mod snapshot;
pub mod sync;
pub mod tray;
pub mod tray_app;
pub mod update;
pub mod view;

pub use application::{AppRole, Taskscape};
pub use message::Message;

pub type AppElement<'a> = iced::Element<'a, Message>;
pub type AppTask = iced::Task<Message>;
