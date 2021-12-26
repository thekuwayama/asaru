#[macro_use]
extern crate clap;

use anyhow::Result;
use clap::{App, Arg};

mod asana;
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
            Arg::with_name("text")
                .help("Performs full-text search on both task name and description")
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
    let text = matches.value_of("text").expect("Failed to specify text");
    let pats = matches.value_of("pats").expect("Failed to specify pats");
    asana::search_tasks(workspace_gid, text, pats)?
        .data
        .iter()
        .map(|t| t.get_permalink_url(pats))
        .flat_map(|r| r.ok())
        .for_each(|s| println!("{}", s));

    Ok(())
}
