use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    event::Event::Key,
    event::KeyCode::{Char, Esc},
    event::{read, KeyEvent},
    style::{style, Attribute, PrintStyledContent, ResetColor, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    ExecutableCommand, Result,
};

enum UiMode {
    Command,
    Edit,
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;

    let mut ui_mode = UiMode::Command;

    loop {
        stdout.execute(Clear(ClearType::All))?;

        let (_, rows) = size().expect("Couldn't communicate with the terminal");
        stdout.execute(MoveTo(0, rows))?;
        match ui_mode {
            UiMode::Command => {
                stdout.execute(PrintStyledContent(
                    style(format!("In COMMAND mode.")).attribute(Attribute::Bold),
                ))?;
                match read().unwrap() {
                    Key(KeyEvent {
                        code: Char('q'), ..
                    }) => {
                        break;
                    }
                    Key(KeyEvent {
                        code: Char('i'), ..
                    }) => ui_mode = UiMode::Edit,
                    _ => (),
                }
            }
            UiMode::Edit => {
                stdout.execute(PrintStyledContent(
                    style(format!("In EDIT mode.")).attribute(Attribute::Bold),
                ))?;
                match read().unwrap() {
                    Key(KeyEvent {
                        code: Char('q'), ..
                    }) => {
                        break;
                    }
                    Key(KeyEvent { code: Esc, .. }) => ui_mode = UiMode::Command,
                    _ => (),
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout.execute(ResetColor)?;

    Ok(())
}
