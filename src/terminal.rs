use std::io::{stdin, stdout, Write};

use anyhow::Result;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, screen};

use crate::controller;

pub fn run(workspace_gid: &str, pats: &str) -> Result<()> {
    let mut stdin = stdin().keys();
    let mut screen = screen::AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(screen, "{}", clear::All).unwrap();
    screen.flush().unwrap();

    let mut state = controller::State::new(workspace_gid, pats);
    'root: loop {
        show(&state);
        let input = stdin.next();

        for c in input {
            match c.unwrap() {
                Key::Ctrl('c') => break 'root,
                Key::Char('\n') => {
                    state.search()?;
                    show(&state);
                    screen.flush().unwrap();
                }
                Key::Char(c) => {
                    state.text.push(c);
                    show(&state);
                    screen.flush().unwrap();
                }
                _ => continue,
            }
        }
    }

    Ok(())
}

fn show(state: &controller::State) {
    print!("{}{}", clear::All, cursor::Goto(1, 1));

    println!("{}", state.text);
    state.get_titles().iter().for_each(|s| println!("{}", s));
}
