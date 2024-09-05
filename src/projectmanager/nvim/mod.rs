use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Entry {
    command_exit: String,
    command_start: String,
    path: PathBuf,
    name: String,
    current: bool,
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            name: "".to_string(),
            path: PathBuf::from(""),
            current: false,
            command_start: "NvimTreeToggle".to_string(),
            command_exit: "".to_string(),
        }
    }
}

pub fn add_project(
    target_path: PathBuf,
    workspace: PathBuf,
    host: String,
    group: String,
    name: String,
) {
    // Read the JSON file
    let json_data = fs::read_to_string(&target_path).expect("Unable to read file");
    let mut data: Vec<Entry> = serde_json::from_str(&json_data).expect("JSON was not well-formed");

    // Create a new entry from the provided parameters
    let new_entry = Entry {
        name: format!("{}/{}/{}", host, group, name),
        path: workspace,
        ..Entry::default()
    };

    // Append the new entry to the array in the JSON data
    if let Some(entries) = data.get_mut("projects") {
        entries.push(serde_json::to_value(&new_entry).expect("Failed to convert Entry to Value"));
    } else {
        // If "projects" field does not exist, create it as an array and add the new entry
        let mut projects = serde_json::Value::default();
        projects
            .as_array_mut()
            .unwrap()
            .push(serde_json::to_value(&new_entry).expect("Failed to convert Entry to Value"));
        data["projects"] = projects;
    }

    // Write the updated JSON data back to the file
    let updated_data = serde_json::to_string_pretty(&data).expect("Failed to serialize JSON");
    fs::write(&target_path, updated_data).expect("Unable to write file");
}
