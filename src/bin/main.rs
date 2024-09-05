use clap::Parser;
use clone_lib::{
    add_project_to_nvim, add_project_to_vscode, clone_repo, parse_repo_url, AppConfig,
};
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

    /// Location of the vscode ProjectManager .json file that contain the projects's locations
    #[clap(long)]
    vscode: Option<PathBuf>,

    /// This option allow to skip the clone operation, to add the project if it already exist.
    #[clap(long, action)]
    skip_clone: bool,

    /// This save the config to the configfile.
    #[clap(short = 'S', long, action)]
    save: bool,

    /// To use a custom regex to define what's the host, the group, and the name of the repository.
    /// [default: "^(?:https://|git@)([^/:]+)[/:]([^/]+)/([^\.]+(?:\.git)?)$"]
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
        config.vscode_projectmanager_path = path
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
        .expect(format!("{:?} didn't match the regex {:?}", &repo_url, &regex_str).as_str());

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
                .join(host.clone())
                .join(group.clone())
                .join(name.clone())
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

    if debug {
        println!("Writing to {:?}", config.nvim_projectmanager_path);
    }
    add_project_to_nvim(
        config.nvim_projectmanager_path.clone(),
        clone_location.clone(),
        host.clone(),
        group.clone(),
        name.clone(),
        debug.clone(),
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
        None => clone_location,
    };
    add_project_to_vscode(
        config.vscode_projectmanager_path,
        path_with_prefix,
        host,
        group,
        name,
        debug,
    );
}
