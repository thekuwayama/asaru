use std::io::{stdin, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, screen};

pub fn run() {
    let mut stdin = stdin().keys();
    let mut screen = screen::AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(screen, "{}", clear::All).unwrap();
    screen.flush().unwrap();

    'root: loop {
        let input = stdin.next();

        for c in input {
            match c.unwrap() {
                Key::Ctrl('c') => break 'root,
                _ => return,
            }
        }
    }
}
