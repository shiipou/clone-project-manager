# 🧬 Clone Project Manager

**Clone Project Manager** is a simple yet powerful tool to keep your workspace clean and organized. Whether you're using VSCode or NeoVim, this utility helps streamline project management with ease! 🧹✨

## 🚀 Installation Guide

### 📋 Requirements

This tool has minimal dependencies, but to maximize its potential, we recommend the following tools for an optimized development workflow:

- **VSCode** with the [Project Manager Extension](https://marketplace.visualstudio.com/items?itemName=alefragnani.project-manager)
- **NeoVim** with the [ProjectMgr Plugin](https://github.com/charludo/projectmgr.nvim) (for NeoVim users, [NVChad](https://nvchad.com) is recommended as a configuration base)

### 🆕 Quick Installation

#### Option 1: Direct Download

Download the latest release of the `clone` binary from the release section and place it in one of your PATH directories (e.g., `~/bin/clone`).

Install it in one step with `curl`:

```sh
curl -fSLo /usr/local/bin/clone https://github.com/shiipou/clone-project-manager/releases/latest/download/clone-$(uname -s)-$(uname -m)
```

Ensure that `/usr/local/bin` is part of your system's `$PATH`.

#### Option 2: Cargo Installation

Alternatively, if you use [Cargo](https://doc.rust-lang.org/cargo/), you can install it directly from crates.io:

```sh
cargo install clone-project-manager
```

> *Tip*: You can specify the version by adding the `--version` argument if needed.

### ⚙️ Configuration

After installing, ensure the binary is executable by running:

```sh
chmod +x /usr/local/bin/clone
```

## 🛠️ How to Use Clone Project Manager

Using **Clone Project Manager** is as simple as running `git clone`, but with added organization! Here's how to use it:

### Basic Usage

```sh
clone https://github.com/shiipou/clone-project-manager
```

This will:

1. Clone the repository to an organized folder structure based on the remote server and groups.
2. Automatically add the cloned project to your **VSCode** or **NeoVim** project manager (if installed).

### Customizing Settings

You can pass various arguments to customize the behavior of the tool. To see all available options, use the `--help` flag:

```sh
clone --help
```

#### Persistent Customization

You can set default options for future runs by using the `--save` flag. For example:

```sh
clone https://github.com/shiipou/clone-project-manager --save
```

This will save the current configuration as default for future usage.

## 🤔 Why Use Clone Project Manager?

This tool offers several advantages over standard `git clone`:

1. **Workspace Organization**: Automatically organizes your cloned repositories into structured directories based on the git remote server and other groupings.
2. **Integration with Project Managers**: Instantly adds cloned projects to your favorite project manager tools like **VSCode** and **NeoVim**, making project switching a breeze.
3. **Automation**: Reduces manual steps and keeps your development environment clean and efficient.

## 📸 Visual Representation

Here's an example of how Clone Project Manager helps organize your projects in VSCode and NeoVim:

### VSCode Project Organization Example

![VSCode Project organization example](https://github.com/shiipou/clone-project-manager/assets/38187238/331cca5a-9a36-4a17-bb61-133f06db9e5d)

### NeoVim Project Organization Example

![NVim Project organization example](https://github.com/user-attachments/assets/3355edc5-3ac8-4d89-872f-ec11698ff876)

## 🤝 Contributions

We welcome contributions from the community! If you encounter any bugs or have suggestions for new features, feel free to open an issue or a pull request.

## 🔗 Useful Links

- [VSCode Project Manager Extension](https://marketplace.visualstudio.com/items?itemName=alefragnani.project-manager)
- [NVim Project Manager Plugin](https://github.com/charludo/projectmgr.nvim)
- [NVChad Configuration](https://nvchad.com)
- [Cargo Installation Guide](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Give **Clone Project Manager** a try and enjoy a cleaner, more organized development environment! 🎉
