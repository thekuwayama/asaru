# Asaru

[![CI](https://github.com/thekuwayama/asaru/workflows/CI/badge.svg)](https://github.com/thekuwayama/asaru/actions?workflow=CI)
[![Apache-2.0 licensed](https://img.shields.io/badge/license-Apache2-blue.svg)](https://raw.githubusercontent.com/thekuwayama/asaru/main/LICENSE-APACHE)

Asaru (`漁る` - look for) is CLI to search Asana Tasks, by which you can do the interactive-search.

<img src="https://github.com/thekuwayama/asaru/blob/1e4a6a7a8be8860ebe7ca18f53bccb5daf56daf1/screenshots/sample.gif" width="50%">


## Install

You can install `asaru` with the following:

```sh-session
$ cargo install --git https://github.com/thekuwayama/asaru.git --branch main
```


## Usage

```sh-session
$ asaru --help
asaru 0.1.0
Asana Tasks Search CLI

USAGE:
    asaru <workspace_gid> <pats> [file]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <workspace_gid>    Globally unique identifier for the workspace or organization
    <pats>             Personal Access Tokens (PATs)
    <file>             Output file
```


## Settings

You can read descriptions about Workspaces:

- https://asana.com/ja/guide/help/workspaces/basics

You can get all your accessible workspace IDs:

- https://app.asana.com/api/1.0/workspaces

You can issue your PATs:

- https://app.asana.com/0/my-apps

```sh-session
$ mkdir $HOME/.asaru

$ echo -n $WORKSPACE_GID > $HOME/.asaru/workspace_gid

$ echo -n $PATS > $HOME/.asaru/pats

$ echo "alias asaru='asaru \$(cat \$HOME/.asaru/workspace_gid) \$(cat \$HOME/.asaru/pats) \$HOME/.asaru/tmp && cat \$HOME/.asaru/tmp | xargs open && rm -f \$HOME/.asaru/tmp'" >> $HOME/.bashrc

$ source $HOME/.bashrc
```


## License

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
