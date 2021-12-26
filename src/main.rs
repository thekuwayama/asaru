#[macro_use]
extern crate clap;

use anyhow::Result;
use clap::{App, Arg};

mod asana;
mod controller;
mod terminal;

fn main() -> Result<()> {
    let cli = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("workspace_gid")
                .help("Globally unique identifier for the workspace or organization")
                .required(true),
        )
        .arg(
            Arg::with_name("pats")
                .help("Personal Access Tokens (PATs)")
                .required(true),
        );
    let matches = cli.get_matches();
    let workspace_gid = matches
        .value_of("workspace_gid")
        .expect("Failed to specify workspace_gid");
    let pats = matches.value_of("pats").expect("Failed to specify pats");

    let _ = terminal::run(workspace_gid, pats); // TODO

    Ok(())
}
