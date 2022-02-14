use std::env;
use std::fs::OpenOptions;
use std::io::{stdout, Write};

use clap::{crate_description, crate_name, crate_version, App, Arg};

mod asana;
mod controller;
mod terminal;

#[tokio::main]
async fn main() {
    let cli = App::new(crate_name!())
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
        .arg(Arg::new("file").help("Output file").required(false));
    let matches = cli.get_matches();
    let workspace_gid = matches
        .value_of("workspace_gid")
        .expect("Error: Failed to specify workspace_gid");
    let pats = matches
        .value_of("pats")
        .expect("Error: Failed to specify pats");
    let file = matches.value_of("file");
    match asana::get_workspace(workspace_gid, pats).await {
        Ok(false) | Err(_) => {
            panic!("Error: Failed to access workspace({})", workspace_gid);
        }
        _ => {}
    };

    let (mut stdout_write, mut file_write);
    let w: &mut dyn Write = match file {
        Some(name) => {
            file_write = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&name)
                .unwrap_or_else(|_| panic!("Error: Failed to open \"{}\"", name));
            &mut file_write
        }
        _ => {
            stdout_write = stdout();
            &mut stdout_write
        }
    };
    terminal::run(workspace_gid, pats)
        .await
        .map(|res| {
            res.iter().for_each(|url| {
                w.write_all(format!("{}\n", url).as_bytes())
                    .expect("Error: Failed to print");
            })
        })
        .unwrap_or_else(|err| panic!("Error: {}", err));
}
