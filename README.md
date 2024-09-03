# clone-project-manager
A simple tool that help to keep your workspace clean.

## Installation

### Requirements

The new version of the tool didn't have any requirements to use. But to be able to take adventage of all the power of this tool, I recommand you to use VSCode with `ProjectManager` extension and/or NeoVim with `ProjectMgr` plugin.

I personnaly use NVChad as neovim config.
  
### New installation

Just download the `clone` file in the release section and place it in one of your path's directory.

I personnaly put it in `~/bin/clone`

Quick curl script to do it with one of the following step :
```sh
curl -fSLo /usr/local/bin/clone https://raw.githubusercontent.com/shiipou/clone-project-manager/main/clone
```

or with cargo :
```sh
# You can specify the version by adding the --version argument.
cargo install -F cli clone-project-manager
```

## How to use it ?

Just clone the project like you'll do it with `git clone`.

```sh
clone https://github.com/shiipou/clone-project-manager
```

If you encounter any problems, You can customize the settings by using the arguments listed in `--help` and using the `--save` to keep them by default for next run.

## Why using this ?

This help to keep your home directory clean. To sort your workspaces by git remote server and groups, and will automatically add it to your [vscode project managment](https://marketplace.visualstudio.com/items?itemName=alefragnani.project-manager)

![image](https://github.com/shiipou/clone-project-manager/assets/38187238/331cca5a-9a36-4a17-bb61-133f06db9e5d)


