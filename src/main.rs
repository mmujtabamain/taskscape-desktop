use iced::widget::{button, column, row, text, text_input};
use iced::{Element, Sandbox, Settings};

fn main() -> iced::Result {
    // Run the application with default settings
    Taskscape::run(Settings::default())
}

// Our application state
struct Taskscape {
    input_value: String,
    tasks: Vec<String>,
}

// Messages that can trigger state changes
#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    AddTask,
    ClearTasks,
}

impl Sandbox for Taskscape {
    type Message = Message;

    // Initialize with empty state
    fn new() -> Self {
        Self {
            input_value: String::new(),
            tasks: Vec::new(),
        }
    }

    // Set the window title
    fn title(&self) -> String {
        String::from("Taskscape")
    }

    // Handle messages and modify state
    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::AddTask => {
                if !self.input_value.trim().is_empty() {
                    self.tasks.push(self.input_value.clone());
                    self.input_value.clear();
                }
            }
            Message::ClearTasks => {
                self.tasks.clear();
            }
        }
    }

    // Render the UI based on current states
    fn view(&self) -> Element<'_, Message> {
        // Create the input row
        let input_row: text_input::TextInput<'_, Message> =
            text_input("Add a task...", &self.input_value)
                .on_input(Message::InputChanged)
                .padding(10)
                .size(20);

        let add_button = button("Add").on_press(Message::AddTask).padding(10);
        let clear_button = button("Clear").on_press(Message::ClearTasks).padding(10);

        // Build the tasks list
        let mut tasks_row = column![].spacing(10).padding(20);

        for (i, task) in self.tasks.iter().enumerate() {
            let task_text = text(format!("{}. {}", i + 1, task)).size(18);
            tasks_row = tasks_row.push(task_text);
        }

        // Combine everything
        column![
            text("My Todo List").size(30),
            row![input_row, add_button, clear_button].spacing(10),
            tasks_row,
        ]
        .spacing(20)
        .padding(20)
        .into()
    }
}
