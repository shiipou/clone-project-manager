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
    group: String,
    name: String,
    debug: bool,
) {
    // Read the JSON file
    let json_data = fs::read_to_string(&target_path).unwrap_or("[]".to_string());
    let mut entries: Vec<Entry> =
        serde_json::from_str(&json_data).expect("JSON was not well-formed");
    let mut code_workspace = workspace.clone();
    if workspace.extension().is_none() {
        let mut workspace_path = workspace.clone();
        workspace_path.set_extension("code-workspace");
        if workspace_path.exists() {
            code_workspace = workspace_path;
        }
    }
    let must_add = !entries.iter().any(|e| e.root_path == code_workspace);
    if debug && !must_add {
        println!("Entry already exist {:?}.", code_workspace);
    }
    // Create a new entry from the provided parameters
    if must_add {
        let new_entry = Entry {
            name,
            root_path: code_workspace,
            tags: [group].to_vec(),
            ..Entry::default()
        };

        // Append the new entry to the array in the JSON data
        entries.push(new_entry.clone());
        // Write the updated JSON data back to the file
        let updated_data =
            serde_json::to_string_pretty(&entries).expect("Failed to serialize JSON");
        let _ = fs::create_dir_all(target_path.parent().unwrap());
        fs::write(&target_path, updated_data).expect("Unable to write file");
        if debug {
            println!("Entry added {:#?}", new_entry);
        }
    }
}
