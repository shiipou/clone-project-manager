use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Entry {
    name: String,
    root_path: PathBuf,
    paths: Vec<PathBuf>,
    tags: Vec<String>,
    enabled: Option<bool>,
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            name: "".to_string(),
            root_path: "".into(),
            paths: [].to_vec(),
            tags: [].to_vec(),
            enabled: Some(true),
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
    let must_add = !entries.iter().any(|e| e.root_path == workspace);
    if debug && !must_add {
        println!("Entry already exist {:?}.", workspace);
    }
    // Create a new entry from the provided parameters
    if must_add {
        let new_entry = Entry {
            name,
            root_path: workspace,
            tags: [host, group].to_vec(),
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
