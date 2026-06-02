//! On-disk storage for named task lists.
//!
//! Each list is its own JSON file under the OS app-data directory
//! (`~/Library/Application Support/Taskscape/lists/` on macOS). The JSON shape
//! ([`TaskListFile`]) doubles as the import/export format, so importing a list
//! is just validating and copying a file in, and exporting is copying one out.
//!
//! A small [`Config`] (`config.json`) remembers the last-opened list so the app
//! can reopen it on the next launch.

use crate::models::Task;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The JSON on-disk shape of a task list. Also the import/export format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListFile {
    pub name: String,
    #[serde(default)]
    pub tasks: Vec<Task>,
}

/// A list as surfaced to the browser: its display name and backing file.
#[derive(Debug, Clone)]
pub struct ListEntry {
    pub name: String,
    pub path: PathBuf,
    pub task_count: usize,
}

/// Persisted app configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// Display name of the list to reopen on launch, if any.
    #[serde(default)]
    pub last_open: Option<String>,
}

/// The app-data root, created if missing. Falls back to the current directory
/// only if `$HOME` is somehow unset (should not happen on macOS).
pub fn app_data_dir() -> PathBuf {
    let base = std::env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join("Library/Application Support"))
        .unwrap_or_else(|| std::env::temp_dir());
    let dir = base.join("Taskscape");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Directory holding one JSON file per list, created if missing.
pub fn lists_dir() -> PathBuf {
    let dir = app_data_dir().join("lists");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Path to the app config file.
pub fn config_path() -> PathBuf {
    app_data_dir().join("config.json")
}

/// Turns a display name into a filesystem-safe file stem.
fn slugify(name: &str) -> String {
    let mut slug: String = name
        .trim()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();
    // Collapse runs of '-' and trim them from the ends.
    while slug.contains("--") {
        slug = slug.replace("--", "-");
    }
    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        String::from("list")
    } else {
        slug
    }
}

/// Picks a not-yet-used `lists/<slug>.json` path for a new list name, suffixing
/// `-2`, `-3`, … on collision.
fn unique_list_path(name: &str) -> PathBuf {
    let dir = lists_dir();
    let base = slugify(name);
    let mut candidate = dir.join(format!("{base}.json"));
    let mut n = 2;
    while candidate.exists() {
        candidate = dir.join(format!("{base}-{n}.json"));
        n += 1;
    }
    candidate
}

/// The file path for a list with the given display name. If a file for this name
/// already exists (matched by its stored `name`), returns that path so saving
/// overwrites it; otherwise returns a fresh unique path.
pub fn path_for_name(name: &str) -> PathBuf {
    if let Some(entry) = list_all().into_iter().find(|e| e.name == name) {
        entry.path
    } else {
        unique_list_path(name)
    }
}

/// Scans the lists directory and returns every readable list, sorted by name.
pub fn list_all() -> Vec<ListEntry> {
    let mut entries = Vec::new();

    let Ok(read_dir) = std::fs::read_dir(lists_dir()) else {
        return entries;
    };

    for dir_entry in read_dir.flatten() {
        let path = dir_entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        if let Ok(file) = read_file(&path) {
            entries.push(ListEntry {
                name: file.name,
                task_count: file.tasks.len(),
                path,
            });
        }
    }

    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    entries
}

/// Reads and parses a list file.
pub fn read_file(path: &Path) -> Result<TaskListFile, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("Could not read {}: {e}", path.display()))?;
    serde_json::from_slice::<TaskListFile>(&bytes)
        .map_err(|e| format!("Could not parse {}: {e}", path.display()))
}

/// Loads the tasks of a list by display name.
pub fn load(name: &str) -> Result<TaskListFile, String> {
    let path = path_for_name(name);
    read_file(&path)
}

/// Writes a list (name + tasks) to its file, creating or overwriting it. Returns
/// the path written.
pub fn save(name: &str, tasks: &[Task]) -> Result<PathBuf, String> {
    let path = path_for_name(name);
    write_file(
        &path,
        &TaskListFile {
            name: name.to_owned(),
            tasks: tasks.to_vec(),
        },
    )?;
    Ok(path)
}

/// Serializes a list to a specific path (pretty JSON).
fn write_file(path: &Path, file: &TaskListFile) -> Result<(), String> {
    let json = serde_json::to_string_pretty(file)
        .map_err(|e| format!("Could not encode list: {e}"))?;
    std::fs::write(path, json).map_err(|e| format!("Could not write {}: {e}", path.display()))
}

/// Deletes a list file by path.
pub fn delete(path: &Path) -> Result<(), String> {
    std::fs::remove_file(path).map_err(|e| format!("Could not delete {}: {e}", path.display()))
}

/// Renames a list (changes its stored display `name`, keeping its tasks). The
/// backing filename is left as-is — the display name lives inside the JSON, so
/// the file stem is incidental. Returns the resulting entry. No-ops to an error
/// if `new_name` is blank or already used by another list.
pub fn rename(old_name: &str, new_name: &str) -> Result<ListEntry, String> {
    let new_name = new_name.trim();
    if new_name.is_empty() {
        return Err(String::from("List name cannot be empty."));
    }
    if new_name == old_name {
        // Nothing to do; return the current entry.
        let path = path_for_name(old_name);
        let file = read_file(&path)?;
        return Ok(ListEntry { name: file.name, task_count: file.tasks.len(), path });
    }
    if list_all().iter().any(|e| e.name == new_name) {
        return Err(format!("A list named \"{new_name}\" already exists."));
    }

    let path = path_for_name(old_name);
    let mut file = read_file(&path)?;
    file.name = new_name.to_owned();
    write_file(&path, &file)?;
    Ok(ListEntry {
        name: file.name,
        task_count: file.tasks.len(),
        path,
    })
}

/// Imports a list from an external JSON file into the lists directory. The
/// imported name is taken from the file; on a name collision a unique file is
/// created (the existing list is not overwritten). Returns the new entry.
pub fn import_from(src: &Path) -> Result<ListEntry, String> {
    let file = read_file(src)?;
    let dest = unique_list_path(&file.name);
    write_file(&dest, &file)?;
    Ok(ListEntry {
        name: file.name,
        task_count: file.tasks.len(),
        path: dest,
    })
}

/// Exports a list (by display name) to an external JSON file.
pub fn export_to(name: &str, dest: &Path) -> Result<(), String> {
    let file = load(name)?;
    write_file(dest, &file)
}

/// Loads the persisted config (defaults if missing/unreadable).
pub fn load_config() -> Config {
    match std::fs::read(config_path()) {
        Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

/// Persists the config (best-effort).
pub fn save_config(config: &Config) {
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = std::fs::write(config_path(), json);
    }
}
