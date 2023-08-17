#!/bin/bash

script_url=https://raw.githubusercontent.com/shiipou/clone-project-manager/main/clone

# if $1 equals install, then install the script
if [ "$1" = "install" ]; then
    echo "Installing..."
    # Get absolute path of the script
    script_path=$2
    if [ -z "$script_path" ]; then
        script_path=$HOME/.local/bin/clone
    fi
    # Extract path of the script
    script_dir=$(dirname "$script_path")
    # check if script_dir is in PATH
    if [[ ":$PATH:" != *":$script_dir:"* ]]; then
        if [ -n "$BASH_VERSION" ]; then
            # Bash shell
            # Linux (bashrc)
            shell_rc_file=~/.bashrc
        elif [ -n "$ZSH_VERSION" ]; then
            # Zsh shell (zshrc)
            shell_rc_file=~/.zshrc
        else
            echo "Unsupported shell"
            exit 1
        fi

        echo "Adding $script_dir to PATH"
        # Add script_dir to PATH
        echo "export PATH=\"\$PATH:$script_dir\"" >> $shell_rc_file
        # Reload shell rc file
        source $shell_rc_file
    fi
    # Create the directory if it doesn't exist
    if [ ! -d "$script_dir" ]; then
        mkdir -p "$script_dir"
    fi
    # Download the script
    tmp=$(mktemp)
    curl -fsSLo "$tmp" "$script_url"
    chmod +x "$tmp"
    mv "$tmp" "$script_path"
    echo "Done"
    exit
elif [ "$1" = "uninstall" ]; then
  script_path=$(which clone)
  echo "Uninstalling $script_path ..."
  rm "$script_path"
else
  echo "Usage: install.sh [install|uninstall]"
  echo -e "\t- install: install clone script"
  echo -e "\t- uninstall: uninstall clone script"
  exit
fi
