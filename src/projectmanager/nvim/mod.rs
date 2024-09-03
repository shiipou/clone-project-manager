use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Entry {
  command_exit: i32,
  command_start: String,
  path: PathBuf,
  name: String,
  current: bool,
}
