use crate::app::{AppTask, Message, Taskscape};
use iced::{Task, keyboard, window};

impl Taskscape {
    pub(crate) fn update(&mut self, message: Message) -> AppTask {
        match message {
            Message::FontLoaded => {} // Fonts loaded, trigger redraw via Task::none()
            Message::ToggleTheme => {
                self.push_history();
                self.theme_mode = self.theme_mode.toggled();
                self.status_message = format!("Switched to {}.", self.theme_mode.label());
            }
            Message::TitleChanged(value) => self.title_input = value,
            Message::DueDateChanged(value) => self.due_date_input = value,
            Message::ComposerPriorityChanged(value) => self.composer_priority = value,
            Message::ToggleTaskCompleted(index, completed) => {
                self.toggle_task_completed(index, completed)
            }
            Message::AddTask => self.add_task(),
            Message::ClearCompleted => {
                self.push_history();
                self.tasks.retain(|task| !task.is_completed());
                self.status_message = String::from("Completed tasks removed.");
            }
            Message::ClearAll => {
                self.push_history();
                self.tasks.clear();
                self.status_message = String::from("All tasks removed.");
            }
            Message::FileNew => self.new_list("Started a new list."),
            Message::FileSave => return Self::launch_save_dialog(),
            Message::FileLoad => return Self::launch_load_dialog(),
            Message::FileSaveResult(Some(path)) => self.complete_save(path),
            Message::FileSaveResult(None) => {
                self.status_message = String::from("Save cancelled.");
            }
            Message::FileLoadResult(Some(path)) => self.complete_load(path),
            Message::FileLoadResult(None) => {
                self.status_message = String::from("Load cancelled.");
            }
            Message::EditUndo => self.undo(),
            Message::EditRedo => self.redo(),
            Message::WindowOpened(window_id) => {
                return window::run(window_id, |window| {
                    crate::app::native_menu::install_for_window(window)
                })
                .map(Message::NativeMenuInstalled);
            }
            Message::NativeMenuEvent(command) => {
                use crate::app::native_menu::NativeMenuCommand;
                match command {
                    NativeMenuCommand::FileNew => self.new_list("Started a new list."),
                    NativeMenuCommand::FileSave => return Self::launch_save_dialog(),
                    NativeMenuCommand::FileLoad => return Self::launch_load_dialog(),
                    NativeMenuCommand::EditUndo => self.undo(),
                    NativeMenuCommand::EditRedo => self.redo(),
                    NativeMenuCommand::ToggleTheme => {
                        self.push_history();
                        self.theme_mode = self.theme_mode.toggled();
                        self.status_message =
                            format!("Switched to {}.", self.theme_mode.label());
                    }
                }
            }
            Message::NativeMenuInstalled(result) => {
                if let Err(error) = result {
                    self.status_message = format!("Native menu: {error}");
                }
            }
            Message::KeyboardEvent(event) => {
                if let keyboard::Event::KeyPressed { key, modifiers, .. } = event {
                    use iced::keyboard::Key;

                    let command = modifiers.command();

                    match key.as_ref() {
                        Key::Character("z") if command && modifiers.shift() => self.redo(),
                        Key::Character("z") if command => self.undo(),
                        Key::Character("s") if command => return Self::launch_save_dialog(),
                        Key::Character("o") if command => return Self::launch_load_dialog(),
                        Key::Character("n") if command => self.new_list("Started a new list."),
                        Key::Character("t") if command => {
                            self.push_history();
                            self.theme_mode = self.theme_mode.toggled();
                            self.status_message =
                                format!("Switched to {}.", self.theme_mode.label());
                        }
                        _ => {}
                    }
                }
            }
        }

        Task::none()
    }
}

