use crate::models::Task;
use std::path::Path;

pub const TODO_FILE: &str = "taskscape-todos.csv";

pub fn save_todos_to_path(tasks: &[Task], path: &Path) -> Result<String, String> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(path)
        .map_err(|error| format!("Unable to create todo CSV: {error}"))?;

    for task in tasks {
        writer
            .serialize(task)
            .map_err(|error| format!("Unable to write todo row: {error}"))?;
    }

    writer
        .flush()
        .map_err(|error| format!("Unable to flush todo CSV: {error}"))?;

    Ok(format!("Todos saved to {}.", path.display()))
}

pub fn load_todos_from_path(path: &Path) -> Result<Vec<Task>, String> {

    if !path.exists() {
        return Err(format!("Todo file {} does not exist.", path.display()));
    }

    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(path)
        .map_err(|error| format!("Unable to open todo CSV: {error}"))?;

    let mut tasks = Vec::new();

    for row in reader.deserialize() {
        let task: Task = row.map_err(|error| format!("Unable to parse todo CSV: {error}"))?;
        tasks.push(task);
    }

    Ok(tasks)
}
