use clap::Parser;
use clone_lib::{add_project_to_nvim, clone_repo, parse_repo_url, AppConfig};
use shellexpand;
use std::path::{Path, PathBuf};

/// Automatically Clone GitHub repositories into a structured workspace. And use it in your VSCode or NVim's project manager.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The git repository to clone.
    repo_url: String,
    /// A directory where all your project goes by default. Used only if --target is not used. Default value based on current OS.
    #[clap(short, long)]
    workspaces_path: Option<PathBuf>,

    /// Force to clone the repository at the specified location.
    #[clap(short, long)]
    target: Option<PathBuf>,

    /// Location of the neovim ProjectMgr .json file that contain the projects's locations
    #[clap(short, long)]
    nvim: Option<PathBuf>,

    /// Location of the vscode ProjectManager .json file that contain the projects's locations
    #[clap(short, long)]
    vscode: Option<PathBuf>,

    /// This option allow to skip the clone operation, to add the project if it already exist.
    #[clap(short, long, action)]
    skip_clone: bool,

    /// This save the config to the configfile.
    #[clap(long, action)]
    save: bool,
}

fn main() {
    let args = Args::parse();

    let repo_url = args.repo_url;
    let workspaces_path = args.workspaces_path;
    let target_path = args.target;
    let nvim_path = args.nvim;
    let vscode_path = args.vscode;
    let skip_clone = args.skip_clone;
    let save = args.save;

    let mut config = AppConfig::load();

    if let Some(path) = workspaces_path {
        config.workspaces_dir = path
            .into_os_string()
            .into_string()
            .expect("'workspaces_path' arg parsing error.");
    }
    if let Some(path) = nvim_path {
        config.nvim_projectmanager_path = path
            .into_os_string()
            .into_string()
            .expect("'nvim' arg parsing error");
    }
    if let Some(path) = vscode_path {
        config.vscode_projectmanager_path = path
            .into_os_string()
            .into_string()
            .expect("'vscode' arg parsing error");
    }

    if save {
        config.save();
    }

    let clone_location = match target_path {
        Some(path) => path,
        None => {
            let workspaces_dir = shellexpand::tilde(&config.workspaces_dir).to_string();
            let workspaces_dir = Path::new(&workspaces_dir);
            workspaces_dir.join(parse_repo_url(&repo_url)).to_path_buf()
        }
    };

    println!("Project destination: {:?}", clone_location);

    if !skip_clone {
        let _ = match clone_repo(&repo_url, &clone_location) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error: {}", e.message());
                Err(())
            }
        };
    }

    add_project_to_nvim()
}
