use std::io;

use crossterm::{ExecutableCommand, cursor, style};

pub fn draw_char(x: isize, y: isize, c: char) -> Result<(), io::Error> {
    io::stdout()
        .execute(cursor::MoveTo(x as u16, y as u16))?
        .execute(style::Print(c))?;
    Ok(())
}
