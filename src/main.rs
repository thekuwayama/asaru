#[macro_use]
extern crate clap;

use std::env;
use std::fs::OpenOptions;
use std::io::{stdout, Write};
use std::process;

use clap::{App, Arg};

mod asana;
mod controller;
mod terminal;

fn main() {
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
        )
        .arg(Arg::with_name("file").help("Output file").required(false));
    let matches = cli.get_matches();
    let workspace_gid = matches
        .value_of("workspace_gid")
        .expect("Error: Failed to specify workspace_gid");
    let pats = matches
        .value_of("pats")
        .expect("Error: Failed to specify pats");
    let file = matches.value_of("file");

    let mut w: Box<dyn Write> = match file {
        Some(name) => Box::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(&name)
                .expect(format!("Error: Failed to open \"{}\"", name).as_str()),
        ),
        _ => Box::new(stdout()),
    };
    match terminal::run(workspace_gid, pats) {
        Ok(res) => {
            res.iter().for_each(|url| {
                w.write_all(format!("{}\n", url).as_bytes())
                    .expect("Error: Failed to print");
            });
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };
}
