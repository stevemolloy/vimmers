use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    event::Event::Key,
    event::KeyCode::{Char, Esc},
    event::{read, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
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
        stdout
            .execute(Clear(ClearType::All))?
            .execute(MoveTo(0, 0))?;

        match ui_mode {
            UiMode::Command => {
                println!("In COMMAND mode");
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
                println!("In EDIT mode");
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
    Ok(())
}
