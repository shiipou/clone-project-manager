use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Entry {
    commandexit: Option<String>,
    commandstart: Option<String>,
    path: PathBuf,
    name: String,
    current: u8,
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            name: "".to_string(),
            path: PathBuf::from(""),
            current: 0,
            commandstart: Some("NvimTreeToggle".to_string()),
            commandexit: Some("".to_string()),
        }
    }
}

pub fn add_project(
    target_path: PathBuf,
    workspace: PathBuf,
    host: String,
    group: String,
    name: String,
    debug: bool,
) {
    // Read the JSON file
    let json_data = fs::read_to_string(&target_path).unwrap_or("[]".to_string());

    let mut entries: Vec<Entry> =
        serde_json::from_str(&json_data).expect("JSON was not well-formed");
    let must_add = !entries.iter().any(|e| e.path == workspace);

    if debug && !must_add {
        println!("Entry already exist {:?}.", workspace);
    }
    // Create a new entry from the provided parameters
    if must_add {
        // Create a new entry from the provided parameters
        let new_entry = Entry {
            name: format!("{}/{}/{}", host, group, name),
            path: workspace,
            ..Entry::default()
        };

        // Append the new entry to the array in the JSON data
        entries.push(new_entry.clone());

        // Write the updated JSON data back to the file
        let updated_data =
            serde_json::to_string_pretty(&entries).expect("Failed to serialize JSON");
        fs::write(&target_path, updated_data).expect("Unable to write file");
        if debug {
            println!("Entry added {:#?}", new_entry);
        }
    }
}
