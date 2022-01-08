# Asaru

[![crates.io](https://img.shields.io/crates/v/asaru.svg)](https://crates.io/crates/asaru)
[![CI](https://github.com/thekuwayama/asaru/workflows/CI/badge.svg)](https://github.com/thekuwayama/asaru/actions?workflow=CI)
[![license](https://img.shields.io/crates/l/asaru.svg)](https://raw.githubusercontent.com/thekuwayama/asaru/main/LICENSE-APACHE)

Asaru (`漁る` - look for) is CLI to search Asana Tasks, by which you can do the interactive-search.

<img src="/screenshots/sample.gif" width="50%">


## Install

You can install `asaru` with the following:

```sh-session
$ cargo install asaru
```


## Usage

```sh-session
$ asaru --help
asaru 0.3.2
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


## Key Manual

| Key                | Explanation                                                                                                 |
|--------------------|-------------------------------------------------------------------------------------------------------------|
| Ctrl-c             | Exit `asaru` command.                                                                                       |
| Ctrl-s             | Move to Search Mode.                                                                                        |
| TAB                | Select(check/uncheck) search results.                                                                       |
| Enter              | Search tasks if the cursor is at the prompt. Get task URLs if the cursor is at search results.              |
| Backspace / Ctrl-h | Delete the character to the left of the cursor if the cursor is at the prompt.                              |
| ←  / Ctrl-b        | Move the cursor left.                                                                                       |
| →  / Ctrl-f        | Move the cursor right.                                                                                      |
| ↓  / Ctrl-n        | Move the cursor down.                                                                                       |
| ↑  / Ctrl-p        | Move the cursor up.                                                                                         |
| PageDown / Alt-v   | Move the cursor to the bottom.                                                                              |
| PageUp / Ctrl-v    | Move the cursor to the top.                                                                                 |
| Ctrl-a             | Move the cursor to the beginning of the text line.                                                          |
| Ctrl-e             | Move the cursor to the end of the text line.                                                                |
| Ctrl-k             | Delete all the text from the current cursor position to the end of the line if the cursor is at the prompt. |


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
