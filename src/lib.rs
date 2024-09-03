use auth_git2::GitAuthenticator;
use dirs::config_dir;
use git2::{Error, Repository};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub mod projectmanager;
//use crate::projectmanager::{nvim, vscode};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub workspaces_dir: String,
    pub nvim_projectmanager_path: String,
    pub vscode_projectmanager_path: String,
}

impl AppConfig {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if config_path.exists() {
            let config_data = fs::read_to_string(config_path).expect("Unable to read config file");
            toml::from_str(&config_data).expect("Invalid config file")
        } else {
            // Default configuration
            match detect_current_os() {
                "wsl" => {
                    if let Some(username) = get_wsl_user_name() {
                        let win_user_home_path = format!("/mnt/c/Users/{}", username);
                        let wsl_user_home_path = std::env::var("HOME")
                            .expect("'HOME' environment variable must be set.");

                        AppConfig {
                            workspaces_dir: format!("{}/workspaces", wsl_user_home_path),
                            nvim_projectmanager_path: format!("{}/AppData/Roaming/nvim/", wsl_user_home_path),
                            vscode_projectmanager_path: format!("{}/AppData/Roaming/Code/User/globalStorage/alefragnani.project-manager/projects.json", win_user_home_path)
                        }
                    } else {
                        panic!("Cannot get WSL username in a WSL environment, that must never happen, that mean powershell.exe didn't exist or didn't provide the environment variable $env:USERNAME.");
                    }
                }
                "macos" => {
                    let user_home_path =
                        std::env::var("HOME").expect("'HOME' environment variable must be set.");
                    AppConfig {
                        workspaces_dir: format!("{}/workspaces", user_home_path),
                        nvim_projectmanager_path: format!("{}/.local/share/nvim/lazy/projectmgr.nvim/projects.json", user_home_path),
                        vscode_projectmanager_path: format!("{}/Library/Application Support/Code/User/globalStorage/alefragnani.project-manager/projects.json", user_home_path)
                    }
                }
                "windows" => {
                    let user_home_path = std::env::var("USERPROFILE")
                        .expect("'USERPROFILE' environment variable must be set.");
                    AppConfig {
                        workspaces_dir: format!("{}/workspaces", user_home_path),
                        nvim_projectmanager_path: format!("{}/AppData/Roaming/nvim/", user_home_path),
                        vscode_projectmanager_path: format!("{}/AppData/Roaming/Code/User/globalStorage/alefragnani.project-manager/projects.json", user_home_path)
                    }
                }
                _linux => {
                    let user_home_path =
                        std::env::var("HOME").expect("'HOME' environment variable must be set.");
                    AppConfig {
                        workspaces_dir: format!("{}/workspaces", user_home_path),
                        nvim_projectmanager_path: format!("{}/.local/share/nvim/lazy/projectmgr.nvim/projects.json", user_home_path),
                        vscode_projectmanager_path:format!("{}/.config/Code/User/globalStorage/alefragnani.project-manager/projects.json", user_home_path)
                    }
                }
            }
        }
    }

    pub fn save(&self) {
        let config_path = Self::config_path();
        println!("Saving configfile to: {:?}", config_path);
        let config_data = toml::to_string(self).expect("Unable to serialize config");
        fs::create_dir_all(config_path.parent().unwrap())
            .expect("Failed to create config directory");
        fs::write(config_path, config_data).expect("Unable to write config file");
    }

    fn config_path() -> PathBuf {
        config_dir().unwrap().join("clone").join("config.toml")
    }
}

pub fn parse_repo_url(url: &str) -> PathBuf {
    let url = url.trim_end_matches(".git");
    let url = url
        .replace("https://", "")
        .replace("git@", "")
        .replace(":", "/");

    PathBuf::from(url)
}

pub fn clone_repo(url: &str, repo_path: &Path) -> Result<Repository, Error> {
    if repo_path.exists() {
        Err(Error::new(
            git2::ErrorCode::Exists,
            git2::ErrorClass::Filesystem,
            format!("repository already exist at {:?}", repo_path),
        ))
    } else {
        //fs::create_dir_all(&repo_path).expect("Failed to create directories");
        let auth = GitAuthenticator::default();
        match auth.clone_repo(url, &repo_path) {
            Ok(repo) => {
                println!("Cloned into {:?}", repo_path);
                Ok(repo)
            }
            Err(e) => Err(e),
        }
    }
}

pub fn detect_current_os() -> &'static str {
    match std::env::consts::OS {
        "linux" => match detect_wsl_with_envs() {
            true => "wsl",
            false => "linux",
        },
        os => os,
    }
}

fn _detect_wsl_with_powershell() -> bool {
    if let Some(_output) = get_wsl_user_name() {
        true
    } else {
        false
    }
}

fn get_wsl_user_name() -> Option<String> {
    if let Ok(output_utf8) = std::process::Command::new("powershell.exe")
        .arg("-c")
        .arg("echo $env:USERNAME")
        .output()
    {
        if let Ok(output) = String::from_utf8(output_utf8.stdout) {
            Some(output)
        } else {
            None
        }
    } else {
        None
    }
}

fn detect_wsl_with_envs() -> bool {
    match std::env::var("WSL_DISTRO_NAME") {
        Ok(_) => true,
        _ => false,
    }
}

pub fn add_project_to_nvim() {}
