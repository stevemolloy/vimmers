use std::io::{stdout, Stdout};

use crossterm::{
    cursor::{MoveDown, MoveTo, MoveToColumn},
    event::Event::Key,
    event::KeyCode::{Char, Enter, Esc},
    event::{read, KeyEvent},
    style::{style, Attribute, Print, PrintStyledContent, ResetColor, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    ExecutableCommand, Result,
};

#[derive(Debug)]
struct TextBuffer {
    text: Vec<String>,
    row: usize,
    col: usize,
}

impl TextBuffer {
    fn new() -> Self {
        Self {
            text: vec![],
            row: 0,
            col: 0,
        }
    }

    fn from(s: &str) -> Self {
        let mut buf = Self::new();
        for i in s.to_string().lines() {
            println!("{}", i);
            buf.text.push(i.to_string());
        }
        return buf;
    }

    fn print(&self, s: &mut Stdout) -> Result<()> {
        s.execute(MoveTo(0, 0))?;
        for line in &self.text {
            s.execute(Print(line))?
                .execute(MoveDown(1))?
                .execute(MoveToColumn(1))?;
        }

        Ok(())
    }

    fn edit(&mut self, c: char) {
        match c {
            '\n' => {
                self.text.push("".to_string());
                self.row += 1;
                self.col = 0;
            }
            a => {
                self.text[self.row].push(a);
                self.col += 1;
            }
        }
    }
}

enum UiMode {
    Command,
    Edit,
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;

    let mut ui_mode = UiMode::Command;
    let mut text_buffer: TextBuffer = TextBuffer::from("\n");

    loop {
        stdout.execute(Clear(ClearType::All))?;

        text_buffer.print(&mut stdout)?;

        let (_, rows) = size().expect("Couldn't communicate with the terminal");

        match ui_mode {
            UiMode::Command => {
                stdout
                    .execute(MoveTo(0, rows))?
                    .execute(PrintStyledContent(
                        style(format!("In COMMAND mode.")).attribute(Attribute::Bold),
                    ))?
                    .execute(MoveTo(text_buffer.col as u16, text_buffer.row as u16))?;
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
                stdout
                    .execute(MoveTo(0, rows))?
                    .execute(PrintStyledContent(
                        style(format!("In EDIT mode.")).attribute(Attribute::Bold),
                    ))?
                    .execute(MoveTo(text_buffer.col as u16, text_buffer.row as u16))?;
                match read().unwrap() {
                    Key(KeyEvent { code: Esc, .. }) => ui_mode = UiMode::Command,
                    Key(KeyEvent { code: Char(a), .. }) => {
                        text_buffer.edit(a);
                    }
                    Key(KeyEvent { code: Enter, .. }) => {
                        text_buffer.edit('\n');
                    }
                    _ => (),
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout
        .execute(ResetColor)?
        .execute(Clear(ClearType::All))?
        .execute(MoveTo(0, 0))?;

    println!("{:#?}", text_buffer);

    Ok(())
}
