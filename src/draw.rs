use std::io;

use crossterm::{cursor, style, terminal, ExecutableCommand, Result};

pub fn draw_text(x: isize, y: isize, s: &str) -> Result<()> {
    io::stdout()
        .execute(cursor::MoveTo(x as u16, y as u16))?
        .execute(style::Print(s))?;
    Ok(())
}

pub fn clear() -> Result<()> {
    std::io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}