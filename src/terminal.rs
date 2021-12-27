use std::io::{stdin, stdout, Write};

use anyhow::Result;
use termion::cursor::{self, DetectCursorPos};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, screen};

use crate::controller;

const PROMPT_LINE: u16 = 1;
const CRLF: &str = "\r\n";

enum Mode {
    Prompt,
    Results,
}

pub fn run(workspace_gid: &str, pats: &str) -> Result<Vec<String>> {
    let mut stdin = stdin().keys();
    let mut screen = screen::AlternateScreen::from(stdout().into_raw_mode()?);
    write!(screen, "{}", clear::All)?;

    let mut state = controller::State::new(workspace_gid, pats);
    show(&mut screen, &state, None)?;

    write!(screen, "{}{}", cursor::Goto(1, PROMPT_LINE), cursor::Show)?;
    screen.flush()?;
    let mut mode = Mode::Prompt;
    let mut result = Ok(Vec::new());
    'root: loop {
        let input = stdin.next();

        for c in input {
            match mode {
                Mode::Prompt => match c? {
                    Key::Ctrl('c') => break 'root,
                    Key::Char('\n') => {
                        state.search()?;
                        if !state.tasks.is_empty() {
                            state.index = 0;
                            show(&mut screen, &state, Some(state.index))?;

                            write!(screen, "{}", cursor::Hide)?;
                            screen.flush()?;
                            mode = Mode::Results;
                        }
                    }
                    Key::Left | Key::Ctrl('b') => {
                        let (x, _) = screen.cursor_pos()?;
                        if x > 1 {
                            write!(screen, "{}", cursor::Goto(x - 1, PROMPT_LINE))?;
                            screen.flush()?;
                        }
                    }
                    Key::Right | Key::Ctrl('f') => {
                        let (x, _) = screen.cursor_pos()?;
                        if x < state.text.len() as u16 + 1 {
                            write!(screen, "{}", cursor::Goto(x + 1, PROMPT_LINE))?;
                            screen.flush()?;
                        }
                    }
                    Key::Char(c) => {
                        let (x, _) = screen.cursor_pos()?;
                        state.text.insert((x - 1) as usize, c);
                        show(&mut screen, &state, None)?;

                        write!(screen, "{}", cursor::Goto(x + 1, PROMPT_LINE))?;
                        screen.flush()?;
                    }
                    Key::Backspace | Key::Ctrl('h') => {
                        let (x, _) = screen.cursor_pos()?;
                        if x > 1 && !state.text.is_empty() {
                            state.text.remove((x - 2) as usize);
                            show(&mut screen, &state, None)?;

                            write!(screen, "{}", cursor::Goto(x - 1, PROMPT_LINE))?;
                            screen.flush()?;
                        }
                    }
                    Key::Ctrl('a') => {
                        write!(screen, "{}", cursor::Goto(1, PROMPT_LINE))?;
                        screen.flush()?;
                    }
                    Key::Ctrl('e') => {
                        write!(
                            screen,
                            "{}",
                            cursor::Goto(state.text.len() as u16 + 1, PROMPT_LINE)
                        )?;
                        screen.flush()?;
                    }
                    Key::Ctrl('k') => {
                        let (x, _) = screen.cursor_pos()?;
                        state.text.truncate((x - 1) as usize);
                        show(&mut screen, &state, None)?;

                        write!(
                            screen,
                            "{}",
                            cursor::Goto(state.text.len() as u16 + 1, PROMPT_LINE)
                        )?;
                        screen.flush()?;
                    }
                    Key::Down | Key::Ctrl('n') if !state.tasks.is_empty() => {
                        state.index = 0;
                        show(&mut screen, &state, Some(state.index))?;

                        write!(screen, "{}", cursor::Hide)?;
                        screen.flush()?;
                        mode = Mode::Results;
                    }
                    _ => continue,
                },
                Mode::Results => match c? {
                    Key::Ctrl('c') => break 'root,
                    Key::Ctrl('s') => {
                        show(&mut screen, &state, None)?;

                        write!(
                            screen,
                            "{}{}",
                            cursor::Goto(state.text.len() as u16 + 1, PROMPT_LINE),
                            cursor::Show
                        )?;
                        screen.flush()?;
                        mode = Mode::Prompt;
                    }
                    Key::Up | Key::Ctrl('p') if state.index <= 0 => {
                        show(&mut screen, &state, None)?;

                        write!(
                            screen,
                            "{}{}",
                            cursor::Goto(state.text.len() as u16 + 1, PROMPT_LINE),
                            cursor::Show
                        )?;
                        screen.flush()?;
                        mode = Mode::Prompt;
                    }
                    Key::Up | Key::Ctrl('p') => {
                        if state.index > 0 {
                            state.index -= 1;
                        }
                        show(&mut screen, &state, Some(state.index))?;
                    }
                    Key::Down | Key::Ctrl('n') => {
                        if state.index + 1 < state.tasks.len() {
                            state.index += 1;
                        }
                        show(&mut screen, &state, Some(state.index))?;
                    }
                    Key::Char('\n') if state.checked.is_empty() => {
                        result = Ok(state.get_permalink_urls(&[state.index]));
                        break 'root;
                    }
                    Key::Char('\n') => {
                        result = Ok(state.get_checked_permalink_urls());
                        break 'root;
                    }
                    Key::Char('\t') => {
                        state.check();
                        show(&mut screen, &state, Some(state.index))?;
                    }
                    _ => continue,
                },
            }
        }
    }
    write!(screen, "{}", cursor::Show)?;
    screen.flush()?;

    result
}

fn show<W: Write>(screen: &mut W, state: &controller::State, opt: Option<usize>) -> Result<()> {
    write!(screen, "{}{}", clear::All, cursor::Goto(1, PROMPT_LINE))?;

    write!(screen, "{}{}{}", state.text, CRLF, CRLF)?;
    state
        .get_titles()
        .iter()
        .enumerate()
        .try_for_each(|(i, s)| match opt {
            Some(index) if i == index && state.is_checked(&i) => {
                write!(
                    screen,
                    "> {}{}{}{}",
                    color::Bg(color::LightMagenta),
                    s,
                    CRLF,
                    color::Bg(color::Reset),
                )
            }
            Some(index) if i == index => write!(screen, "> {}{}", s, CRLF),
            Some(_) if state.is_checked(&i) => {
                write!(
                    screen,
                    "  {}{}{}{}",
                    color::Bg(color::LightMagenta),
                    s,
                    CRLF,
                    color::Bg(color::Reset),
                )
            }
            None if state.is_checked(&i) => {
                write!(
                    screen,
                    "  {}{}{}{}",
                    color::Bg(color::LightMagenta),
                    s,
                    CRLF,
                    color::Bg(color::Reset),
                )
            }
            _ => write!(screen, "  {}{}", s, CRLF),
        })?;
    screen.flush()?;

    Ok(())
}
