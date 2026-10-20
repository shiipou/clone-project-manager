use clap::Parser;
use clone_lib::{
    add_project_to_alefragnani_project_manager, add_project_to_nvim,
    add_project_to_wirecore_vscode_project_manager, clone_repo, parse_repo_url, AppConfig,
};
use serde_json::Value;
use std::path::{Path, PathBuf};

/// Automatically Clone GitHub repositories into a structured workspace. And use it in your VSCode or NVim's project manager.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The git repository to clone.
    repo_url: Option<String>,
    /// A directory where all your project goes by default. Used only if --target is not used. Default value based on current OS.
    #[clap(short = 'p', long)]
    workspaces_path: Option<PathBuf>,

    /// Force to clone the repository at the specified location.
    #[clap(short, long)]
    target: Option<PathBuf>,

    /// Location of the neovim ProjectMgr .json file that contain the projects's locations
    #[clap(long)]
    nvim: Option<PathBuf>,

    /// Location of either VS Code project manager's projects.json; detects the manager from its path or schema
    #[clap(long)]
    vscode: Option<PathBuf>,

    /// Explicit location of the Alefragnani Project Manager projects.json file
    #[clap(long)]
    vscode_alefragnani: Option<PathBuf>,

    /// Explicit location of the WireCore VS Code Project Manager projects.json file
    #[clap(long)]
    vscode_wirecore: Option<PathBuf>,

    /// This option allow to skip the clone operation, to add the project if it already exist.
    #[clap(long, action)]
    skip_clone: bool,

    /// This save the config to the configfile.
    #[clap(short = 'S', long, action)]
    save: bool,

    /// To use a custom regex to define what's the host, the group, and the name of the repository.
    /// [default: "^(?:(?:ssh|https?)://)?(?:git@)?([^/:]+)(?::\d+)?[/:](.+)/([^/]+?)(?:\.git)?/?$"]
    #[clap(short, long)]
    regex: Option<String>,

    /// The owner of the project. Allow to sort the project in the correct directory. If not specified, it'll use the regex argument to obtain it. Example: shiipou
    #[clap(short, long)]
    owner: Option<String>,

    /// The source of the project. Allow to sort the project in the correct directory. If not specified, it'll use the regex argument to obtain it. Example:
    /// github.com
    #[clap(short, long)]
    source: Option<String>,

    /// The name of the project. Allow to sort the project in the correct directory. If not specified, it'll use the regex argument to obtain it. Example:
    /// clone-project-manager
    #[clap(short, long)]
    name: Option<String>,

    /// Enable debug mode to print more info into the terminal.
    #[clap(short = 'D', long, action)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let debug = args.debug;

    if debug {
        println!("Runtime values are {:#?}", &args);
    }

    let workspaces_path = args.workspaces_path;
    let target_path = args.target;
    let nvim_path = args.nvim;
    let vscode_path = args.vscode;
    let alefragnani_vscode_path = args.vscode_alefragnani;
    let vscode_wirecore_path = args.vscode_wirecore;
    let skip_clone = args.skip_clone;
    let save = args.save;
    let regex_input = args.regex;

    let mut config = AppConfig::load();

    if let Some(path) = workspaces_path {
        config.workspaces_dir = path
    }
    if let Some(path) = nvim_path {
        config.nvim_projectmanager_path = path
    }
    if let Some(path) = vscode_path {
        match detect_vscode_manager(&path) {
            Ok(VscodeManager::Alefragnani) => config.vscode_projectmanager_path = path,
            Ok(VscodeManager::WireCore) => config.wirecore_vscode_projectmanager_path = path,
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(2);
            }
        }
    }
    if let Some(path) = alefragnani_vscode_path {
        config.vscode_projectmanager_path = path
    }
    if let Some(path) = vscode_wirecore_path {
        config.wirecore_vscode_projectmanager_path = path
    }
    let regex_str = match regex_input {
        Some(regex) => regex,
        None => config.regex.to_owned(),
    };

    if debug {
        println!("Current config is {:#?}", config);
    }

    if save {
        config.save();
    }

    let repo_url = match args.repo_url {
        Some(url) => url,
        None => {
            eprintln!("Error: The repository URL is required to go futher.");
            std::process::exit(1);
        }
    };

    let (host, group, name) = {
        let (from_repo_host, from_repo_group, from_repo_name) = parse_repo_url(
            &repo_url, &regex_str,
        )
        .unwrap_or_else(|e| panic!("Failed to parse repo URL: {}", e));

        let host = match args.source {
            Some(source) => source,
            None => from_repo_host,
        };
        let group = match args.owner {
            Some(owner) => owner,
            None => from_repo_group,
        };
        let project_name = match args.name {
            Some(name) => name,
            None => from_repo_name,
        };

        (host, group, project_name)
    };

    let clone_location = match target_path {
        Some(path) => path,
        None => {
            let workspaces_dir = &config.workspaces_dir;

            workspaces_dir
                .join(&host)
                .join(&group)
                .join(&name)
        }
    };

    println!("Project destination: {:?}", clone_location);

    if !skip_clone {
        if let Err(e) = clone_repo(&repo_url, &clone_location) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    if debug {
        println!("Writing to {:?}", config.nvim_projectmanager_path);
    }
    add_project_to_nvim(
        config.nvim_projectmanager_path.clone(),
        clone_location.clone(),
        host.clone(),
        group.clone(),
        name.clone(),
        debug,
    );

    if debug {
        println!("Writing to {:?}", config.vscode_projectmanager_path);
    }

    let path_with_prefix = match &config.vscode_path_prefix {
        Some(prefix) => {
            let prefixed_path = prefix.to_owned()
                + clone_location
                    .to_str()
                    .expect("Cannot parse clone_location as a string");
            Path::new(&prefixed_path).to_path_buf()
        }
        None => clone_location.clone(),
    };
    add_project_to_alefragnani_project_manager(
        config.vscode_projectmanager_path,
        path_with_prefix,
        group.clone(),
        name.clone(),
        debug,
    );

    if debug {
        println!(
            "Writing to {:?}",
            config.wirecore_vscode_projectmanager_path
        );
    }
    add_project_to_wirecore_vscode_project_manager(
        config.wirecore_vscode_projectmanager_path,
        clone_location,
        host,
        group,
        name,
        debug,
    );
}

#[derive(Debug, PartialEq, Eq)]
enum VscodeManager {
    Alefragnani,
    WireCore,
}

fn detect_vscode_manager(path: &Path) -> Result<VscodeManager, String> {
    let path_text = path.to_string_lossy().to_ascii_lowercase();
    if path_text.contains("alefragnani.project-manager") {
        return Ok(VscodeManager::Alefragnani);
    }
    if path_text.contains("wirecore.vscode-project-manager") {
        return Ok(VscodeManager::WireCore);
    }

    let json_data = std::fs::read_to_string(path).map_err(|error| {
        format!(
            "Cannot detect the VS Code project manager from {:?}: {}. Use --vscode-wirecore for WireCore or --vscode with an extension-specific path.",
            path, error
        )
    })?;
    detect_vscode_manager_from_json(&json_data).map_err(|_| {
        format!(
            "Cannot detect the VS Code project manager from {:?}. Use --vscode-wirecore for WireCore or pass a projects.json with a recognizable schema.",
            path
        )
    })
}

fn detect_vscode_manager_from_json(json_data: &str) -> Result<VscodeManager, ()> {
    let entries: Vec<Value> = serde_json::from_str(json_data).map_err(|_| ())?;
    let has_wirecore_schema = entries.iter().any(|entry| {
        entry.get("_path").is_some()
            || entry.get("_categoryId").is_some()
            || entry.get("_id").is_some()
    });
    let has_alefragnani_schema = entries.iter().any(|entry| entry.get("rootPath").is_some());

    match (has_alefragnani_schema, has_wirecore_schema) {
        (true, false) => Ok(VscodeManager::Alefragnani),
        (false, true) => Ok(VscodeManager::WireCore),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_managers_from_extension_paths() {
        assert_eq!(
            detect_vscode_manager(Path::new(
                "/Code/User/globalStorage/alefragnani.project-manager/projects.json"
            )),
            Ok(VscodeManager::Alefragnani)
        );
        assert_eq!(
            detect_vscode_manager(Path::new(
                "/Code/User/globalStorage/wirecore.vscode-project-manager/projects.json"
            )),
            Ok(VscodeManager::WireCore)
        );
    }

    #[test]
    fn detects_managers_from_projects_json_schema() {
        assert_eq!(
            detect_vscode_manager_from_json(r#"[{"rootPath":"/work","name":"work"}]"#),
            Ok(VscodeManager::Alefragnani)
        );
        assert_eq!(
            detect_vscode_manager_from_json(r#"[{"_path":"/work","_categoryId":0}]"#),
            Ok(VscodeManager::WireCore)
        );
        assert!(detect_vscode_manager_from_json("[]").is_err());
    }
}
