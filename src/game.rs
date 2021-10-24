use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::board::Board;
use crate::draw::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub enum Game {
    Start,
    Play(Box<Board>),
    Over { score: usize },
}

impl Game {
    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        match self {
            Self::Start => {
                let msg = "Press SPACE to start";
                canvas.set_draw_color(sdl2::pixels::Color::RGBA(200, 0, 0, 255));
                canvas.fill_rect(Rect::new(
                    0,
                    0,
                    SCREEN_HEIGHT as u32 / 20,
                    SCREEN_HEIGHT as u32 / 20,
                ))?;
            }
            Self::Play(board) => {
                board.draw(
                    canvas,
                    Rect::new(10, 10, SCREEN_WIDTH / 2 - 20, SCREEN_HEIGHT - 20),
                )?;
            }
            Self::Over { score } => {
                let msg = "GAME OVER";
                let score_msg = format!("Score: {}", score);
            }
        }
        Ok(())
    }

    pub fn input(&mut self, code: Keycode) {
        match self {
            Self::Start => {
                if let Keycode::Space = code {
                    *self = Self::Play(Box::new(Board::new(10, 20)));
                }
            }
            Self::Play(board) => {
                board.input(code);
            }
            Self::Over { .. } => {
                if let Keycode::Space = code {
                    *self = Self::Start;
                }
            }
        }
    }

    pub fn step(&mut self) {
        match self {
            Self::Start => {}
            Self::Play(board) => {
                if board.step() {
                    *self = Self::Over { score: board.score };
                }
            }
            Self::Over { .. } => {}
        }
    }
}
