pub mod actions;
pub mod application;
pub mod hotkey;
pub mod message;
pub mod native_menu;
pub mod queries;
pub mod snapshot;
pub mod tray;
pub mod update;
pub mod view;

pub use application::{run, Taskscape};
pub use message::Message;

pub type AppElement<'a> = iced::Element<'a, Message>;
pub type AppTask = iced::Task<Message>;
