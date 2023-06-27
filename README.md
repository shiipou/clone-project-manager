# clone-project-manager
A simple script to help me to keep my projects clean when I clone it

## Installation

### New installation

Just download the `clone` file and place it in a directory that was in your path.

I personnaly put it in `~/bin/clone`

Quick curl script to do it with one of the following step :
```sh
curl -fSLo ~/bin/clone https://raw.githubusercontent.com/shiipou/clone-project-manager/stable/clone
```
or
```sh
curl -fsSL https://raw.githubusercontent.com/shiipou/clone-project-manager/stable/install.sh | sh -s -- install
```

### Upgrading

Just run the upgrade script

```sh
clone upgrade
```


## How to use it ?

Just clone the project like you'll do it with `git clone`.

```sh
clone https://github.com/shiipou/clone-project-manager
```

## Why using this ?

This help to keep your home directory clean. To sort your workspaces by git remote server and groups, and will automatically add it to your [vscode project managment](https://marketplace.visualstudio.com/items?itemName=alefragnani.project-manager)

![image](https://github.com/shiipou/clone-project-manager/assets/38187238/331cca5a-9a36-4a17-bb61-133f06db9e5d)
