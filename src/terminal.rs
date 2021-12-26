use std::io::{stdin, stdout, Write};

use anyhow::Result;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, screen};

use crate::controller;

pub fn run(workspace_gid: &str, pats: &str) -> Result<()> {
    let mut stdin = stdin().keys();
    let mut screen = screen::AlternateScreen::from(stdout().into_raw_mode()?);
    write!(screen, "{}", clear::All)?;
    screen.flush()?;

    let mut state = controller::State::new(workspace_gid, pats);
    'root: loop {
        show(&mut screen, &state)?;
        let input = stdin.next();

        for c in input {
            match c? {
                Key::Ctrl('c') => break 'root,
                Key::Char('\n') => {
                    state.search()?;
                    show(&mut screen, &state)?;
                }
                Key::Char(c) => {
                    state.text.push(c);
                    show(&mut screen, &state)?;
                    screen.flush()?;
                }
                _ => continue,
            }
        }
    }

    Ok(())
}

fn show<W: Write>(screen: &mut W, state: &controller::State) -> Result<()> {
    write!(screen, "{}{}", clear::All, cursor::Goto(1, 1))?;

    write!(screen, "{}", state.text)?;
    state.get_titles().iter().enumerate().for_each(|(i, s)| {
        write!(screen, "{}{}", cursor::Goto(1, (i + 2) as u16), s); // FIXME
    });
    screen.flush()?;

    Ok(())
}
