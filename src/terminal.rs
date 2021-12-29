use std::io::{stdin, stdout, Write};

use anyhow::{anyhow, Result};
use termion::cursor::{self, DetectCursorPos};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, screen, terminal_size};

use crate::controller;

const BOL: u16 = 1;
const BOP: u16 = 3;
const FIRST_LINE: u16 = 1;
const PROMPT_LINE: u16 = 3;
const RESULTS_LINE: u16 = 5;
const MENU_BAR: &str = "Asaru | Ctrl-c: Exit | Ctrl-s: Search | TAB: Select | Enter: Execute";
const CRLF: &str = "\r\n";

enum Mode {
    Prompt,
    Results,
}

pub fn run(workspace_gid: &str, pats: &str) -> Result<Vec<String>> {
    let mut stdin = stdin().keys();
    let mut screen = screen::AlternateScreen::from(stdout().into_raw_mode()?);
    write!(screen, "{}{}", clear::All, color::Fg(color::LightWhite))?;

    let mut state = controller::State::new(workspace_gid, pats);
    show_state(&mut screen, &state, None)?;
    show_cursor(&mut screen, BOP, PROMPT_LINE)?;
    let mut mode = Mode::Prompt;
    let mut result = Ok(Vec::new());
    'root: loop {
        let input = stdin.next();

        if let Some(c) = input {
            match mode {
                Mode::Prompt => match c? {
                    Key::Ctrl('c') => break 'root,
                    Key::Char('\n') => {
                        state.search()?;
                        if !state.tasks.is_empty() {
                            state.index = 0;
                            state.checked.clear();
                            show_state(&mut screen, &state, Some(state.index))?;
                            hide_cursor(&mut screen)?;
                            mode = Mode::Results;
                        } else {
                            let (x, _) = screen.cursor_pos()?;
                            state.checked.clear();
                            show_state(&mut screen, &state, Some(state.index))?;
                            show_cursor(&mut screen, x, PROMPT_LINE)?;
                        }
                    }
                    Key::Left | Key::Ctrl('b') => {
                        let (x, _) = screen.cursor_pos()?;
                        if x > 1 {
                            show_cursor(&mut screen, x - 1, PROMPT_LINE)?;
                        }
                    }
                    Key::Right | Key::Ctrl('f') => {
                        let (x, _) = screen.cursor_pos()?;
                        if x < state.text.len() as u16 + 1 {
                            show_cursor(&mut screen, x + 1, PROMPT_LINE)?;
                        }
                    }
                    Key::Char(c) => {
                        let (x, _) = screen.cursor_pos()?;
                        state.text.insert((x - BOP) as usize, c);
                        show_state(&mut screen, &state, None)?;
                        show_cursor(&mut screen, x + 1, PROMPT_LINE)?;
                    }
                    Key::Backspace | Key::Ctrl('h') => {
                        let (x, _) = screen.cursor_pos()?;
                        if x > 1 && !state.text.is_empty() {
                            state.text.remove((x - BOP - 1) as usize);
                            show_state(&mut screen, &state, None)?;
                            show_cursor(&mut screen, x - 1, PROMPT_LINE)?;
                        }
                    }
                    Key::Ctrl('a') => {
                        show_cursor(&mut screen, BOP, PROMPT_LINE)?;
                    }
                    Key::Ctrl('e') => {
                        show_cursor(&mut screen, state.text.len() as u16 + BOP, PROMPT_LINE)?;
                    }
                    Key::Ctrl('k') => {
                        let (x, _) = screen.cursor_pos()?;
                        state.text.truncate((x - BOP) as usize);
                        show_state(&mut screen, &state, None)?;
                        show_cursor(&mut screen, state.text.len() as u16 + BOP, PROMPT_LINE)?;
                    }
                    Key::Down | Key::Ctrl('n') => {
                        if !state.tasks.is_empty() {
                            state.index = 0;
                            show_state(&mut screen, &state, Some(state.index))?;
                            hide_cursor(&mut screen)?;
                            mode = Mode::Results;
                        }
                    }
                    _ => continue,
                },
                Mode::Results => match c? {
                    Key::Ctrl('c') => break 'root,
                    Key::Ctrl('s') => {
                        show_state(&mut screen, &state, None)?;
                        show_cursor(&mut screen, state.text.len() as u16 + BOP, PROMPT_LINE)?;
                        mode = Mode::Prompt;
                    }
                    Key::Up | Key::Ctrl('p') => {
                        if state.index > 0 {
                            state.index -= 1;
                            show_state(&mut screen, &state, Some(state.index))?;
                        } else {
                            show_state(&mut screen, &state, None)?;
                            show_cursor(&mut screen, state.text.len() as u16 + BOP, PROMPT_LINE)?;
                            mode = Mode::Prompt;
                        }
                    }
                    Key::Down | Key::Ctrl('n') => {
                        let (_, h) = terminal_size()?;
                        if state.index + 1 < state.tasks.len()
                            && state.index as u16 + RESULTS_LINE < h
                        {
                            state.index += 1;
                            show_state(&mut screen, &state, Some(state.index))?;
                        }
                    }
                    Key::Char('\n') => {
                        if state.checked.is_empty() {
                            result = state
                                .get_permalink_url()
                                .map(|s| vec![s])
                                .ok_or(anyhow!("Failed to extract permalink_url"));
                        } else {
                            let urls = state.get_checked_permalink_urls();
                            if state.checked.len() == urls.len() {
                                result = Ok(urls);
                            } else {
                                result = Err(anyhow!("Failed to extract permalink_url"));
                            }
                        }
                        break 'root;
                    }
                    Key::Char('\t') => {
                        if state.is_checked(&state.index) {
                            state.uncheck();
                        } else {
                            state.check();
                        }
                        show_state(&mut screen, &state, Some(state.index))?;
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

fn show_state<W: Write>(
    screen: &mut W,
    state: &controller::State,
    opt: Option<usize>,
) -> Result<()> {
    let (w, h) = terminal_size()?;
    write!(screen, "{}{}", clear::All, cursor::Goto(BOL, FIRST_LINE))?;

    write!(
        screen,
        "{}{}{}{}",
        color::Bg(color::LightMagenta),
        format!("{:<width$}", MENU_BAR, width = w as usize),
        color::Bg(color::Reset),
        CRLF,
    )?;

    write!(screen, "{}$ {}{}", CRLF, state.text, CRLF)?;
    let mut titles = state.get_titles();
    titles.truncate((h - PROMPT_LINE - 1) as usize);
    titles.iter().enumerate().try_for_each(|(i, s)| match opt {
        Some(index) if i == index && state.is_checked(&i) => {
            write!(
                screen,
                "{}> {}{}{}",
                CRLF,
                color::Bg(color::Magenta),
                s,
                color::Bg(color::Reset),
            )
        }
        Some(index) if i == index => write!(screen, "{}> {}", CRLF, s),
        Some(_) if state.is_checked(&i) => {
            write!(
                screen,
                "{}  {}{}{}",
                CRLF,
                color::Bg(color::Magenta),
                s,
                color::Bg(color::Reset),
            )
        }
        None if state.is_checked(&i) => {
            write!(
                screen,
                "{}  {}{}{}",
                CRLF,
                color::Bg(color::Magenta),
                s,
                color::Bg(color::Reset),
            )
        }
        _ => write!(screen, "{}  {}", CRLF, s),
    })?;
    screen.flush()?;

    Ok(())
}

fn show_cursor<W: Write>(screen: &mut W, x: u16, y: u16) -> Result<()> {
    write!(screen, "{}{}", cursor::Goto(x, y), cursor::Show)?;
    screen.flush()?;

    Ok(())
}

fn hide_cursor<W: Write>(screen: &mut W) -> Result<()> {
    write!(screen, "{}", cursor::Hide)?;
    screen.flush()?;

    Ok(())
}
