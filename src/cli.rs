use clap::{crate_description, crate_name, crate_version, Arg, Command};

pub const WORKSPACE_GID: &str = "workspace_gid";
pub const PATS: &str = "pats";
pub const FILE: &str = "file";

pub fn build() -> Command<'static> {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new(WORKSPACE_GID)
                .help("Globally unique identifier for the workspace or organization")
                .required(true),
        )
        .arg(
            Arg::new(PATS)
                .help("Personal Access Tokens (PATs)")
                .required(true),
        )
        .arg(Arg::new(FILE).help("Output file").required(false))
}
