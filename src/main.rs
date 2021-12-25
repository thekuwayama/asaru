#[macro_use]
extern crate clap;

use anyhow::{anyhow, Result};
use clap::{App, Arg};
use reqwest::blocking::Client;
use reqwest::StatusCode;

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
    println!("{}", search_ticket(workspace_gid, text, pats)?);

    Ok(())
}

fn search_ticket(workspace_gid: &str, text: &str, pats: &str) -> Result<String> {
    let url = format!(
        "https://app.asana.com/api/1.0/workspaces/{}/tasks/search?text={}",
        workspace_gid, text
    );
    let cli = Client::new();
    let res = cli.get(url).bearer_auth(pats).send()?;
    if res.status() == StatusCode::OK {
        return Ok(res.text()?);
    }

    Err(anyhow!(
        "Failed to search tasks in a workspace app.asana.com"
    ))
}
