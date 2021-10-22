use crate::board::Board;
use crate::draw::{clear, draw_text};

pub enum Game {
    Start,
    Play(Board),
    Over,
}

impl Game {
    pub(crate) fn draw(&self) -> crossterm::Result<()> {
        let (w, h) = crossterm::terminal::size()?;
        match self {
            Self::Start => {
                let msg = "Press SPACE to start";
                clear()?;
                draw_text(
                    w as isize / 2 - msg.len() as isize / 2,
                    h as isize / 2,
                    &msg,
                )?;
            }
            Self::Play(board) => board.draw(w as isize / 2, h as isize / 2)?,
            Self::Over => {
                let msg = "GAME OVER";
                clear()?;
                draw_text(
                    w as isize / 2 - msg.len() as isize / 2,
                    h as isize / 2,
                    &msg,
                )?;
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
            }
            Self::Over => {}
        }
    }
}
