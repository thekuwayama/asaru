use clap::{crate_description, crate_name, crate_version, App, Arg};

pub fn build() -> App<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("workspace_gid")
                .help("Globally unique identifier for the workspace or organization")
                .required(true),
        )
        .arg(
            Arg::new("pats")
                .help("Personal Access Tokens (PATs)")
                .required(true),
        )
        .arg(Arg::new("file").help("Output file").required(false))
}
