pub mod actions;
pub mod application;
pub mod message;
pub mod queries;
pub mod update;
pub mod view;

pub use application::{run, Taskscape};
pub use message::Message;

pub type AppElement<'a> = iced::Element<'a, Message>;
