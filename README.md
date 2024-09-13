# 🧬 clone-project-manager

A simple tool to help keep your workspace clean and organized! 🧹✨

## 🚀 Installation

### 📋 Requirements

The latest version of this tool doesn't have any specific requirements. However, to take full advantage of its power, we recommend using:

- VSCode with the `ProjectManager` extension
- and/or NeoVim with the `ProjectMgr` plugin

I personally use NVChad as my Neovim configuration.

### 🆕 New Installation

Simply download the `clone` file from the release section and place it in one of your PATH directories. I personally put it in `~/bin/clone`.

Quick curl script to install it in one step:

```sh
curl -fSLo /usr/local/bin/clone https://github.com/shiipou/clone-project-manager/releases/latest/download/clone-$(uname -s)-$(uname -m)
```

Alternatively, you can install it using Cargo:

```sh
# You can specify the version by adding the --version argument.
cargo install clone-project-manager
```

## 🛠️ How to Use

Just clone the project as you would with `git clone`:

```sh
clone https://github.com/shiipou/clone-project-manager
```

If you encounter any issues, you can customize the settings using the arguments listed in `--help`. Use the `--save` option to keep them as default for future runs.

## 🤔 Why Use This?

This tool helps keep your home directory clean by:

1. Sorting your workspaces by git remote server and groups
2. Automatically adding projects to your [VSCode Project Management](https://marketplace.visualstudio.com/items?itemName=alefragnani.project-manager)

Here's a visual representation of how it organizes your projects:

![Project organization example](https://github.com/shiipou/clone-project-manager/assets/38187238/331cca5a-9a36-4a17-bb61-133f06db9e5d)

Give it a try and experience a more organized development environment! 🎉
