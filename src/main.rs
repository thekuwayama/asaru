use std::fs::OpenOptions;
use std::io::{stdout, Write};
use std::process;

mod asana;
mod cli;
mod controller;
mod terminal;

#[tokio::main]
async fn main() {
    let matches = cli::build().get_matches();
    let workspace_gid = matches
        .value_of(cli::WORKSPACE_GID)
        .expect("Error: Failed to specify workspace_gid");
    let pats = matches
        .value_of(cli::PATS)
        .expect("Error: Failed to specify pats");
    let file = matches.value_of(cli::FILE);
    match asana::get_workspace(workspace_gid, pats).await {
        Ok(false) | Err(_) => {
            eprintln!("Error: Failed to access workspace({})", workspace_gid);
            process::exit(1);
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
                .unwrap_or_else(|_| {
                    eprintln!("Error: Failed to open \"{}\"", name);
                    process::exit(1);
                });
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
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            process::exit(1);
        });
}
