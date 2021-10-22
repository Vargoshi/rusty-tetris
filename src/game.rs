use crate::board::Board;

pub enum Game {
    Start,
    Play(Board),
    Over,
}

impl Game {
    pub(crate) fn draw(&self) -> crossterm::Result<()> {
        use crossterm::{cursor, style, terminal, ExecutableCommand};
        match self {
            Self::Start => {
                let msg = "Press SPACE to start";
                let (w, h) = terminal::size()?;
                std::io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;
                std::io::stdout()
                    .execute(cursor::MoveTo(w / 2 - msg.len() as u16 / 2, h / 2))?
                    .execute(style::Print(msg))?;
            }
            Self::Play(board) => board.draw()?,
            Self::Over => {
                let msg = "GAME OVER";
                let (w, h) = terminal::size()?;
                std::io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;
                std::io::stdout()
                    .execute(cursor::MoveTo(w / 2 - msg.len() as u16 / 2, h / 2))?
                    .execute(style::Print(msg))?;
            }
        }
        Ok(())
    }

    pub(crate) fn input(&mut self, code: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode;
        match self {
            Self::Start => {
                if let KeyCode::Char(' ') = code {
                    *self = Self::Play(Board::new());
                }
            }
            Self::Play(board) => board.input(code),
            Self::Over => {
                if let KeyCode::Char(' ') = code {
                    *self = Self::Start;
                }
            }
        }
    }

    pub(crate) fn step(&mut self) {
        match self {
            Self::Start => {}
            Self::Play(board) => {
                if board.step() {
                    *self = Self::Over;
                }
            },
            Self::Over => {}
        }
    }
}
