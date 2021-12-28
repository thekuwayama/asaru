# Asaru

Asaru (`漁る` - look for) is CLI to search Asana Tasks, by which you can do the interactive-search.


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
    asaru <workspace_gid> <pats>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <workspace_gid>    Globally unique identifier for the workspace or organization
    <pats>             Personal Access Tokens (PATs)
```


## Settings

```sh-session
$ mkdir $HOME/.asaru

$ echo -n $WORKSPACE_GID > $HOME/.asaru/workspace_gid

$ echo -n $PATS > $HOME/.asaru/pats

$ echo "alias asaru='asaru \$(cat \$HOME/.asaru/workspace_gid) \$(cat \$HOME/.asaru/pats)'" >> $HOME/.bashrc

$ source $HOME/.bashrc
```


## License

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
