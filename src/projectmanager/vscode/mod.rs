use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Repo {
  name: String,
  root_path: String,
  paths: Vec<PathBuf>,
  tags: Vec<String>,
  enabled: bool
}
