use std::io;

use crossterm::{cursor, event, style, terminal, ExecutableCommand};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

struct Pos {
    x: usize,
    y: usize,
}

pub struct Board {
    pos: Pos,
    cells: [[bool; WIDTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            pos: Pos { x: 5, y: 5 },
            cells: Default::default(),
        }
    }

    pub fn draw(&self) -> crossterm::Result<()> {
        io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;

        for y in 0..HEIGHT + 2 {
            for x in 0..WIDTH + 2 {
                if y == 0 || y == HEIGHT + 1 || x == 0 || x == WIDTH + 1 {
                    draw_char(x, y, '+')?;
                }
            }
        }

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.cells[y][x] {
                    draw_char(x + 1, y + 1, '@')?;
                }
            }
        }

        draw_char(self.pos.x + 1, self.pos.y + 1, '#')?;

        Ok(())
    }

    pub fn input(&mut self, code: event::KeyCode) {
        match code {
            event::KeyCode::Up => self.mv(Dir::Up),
            event::KeyCode::Down => self.mv(Dir::Down),
            event::KeyCode::Left => self.mv(Dir::Left),
            event::KeyCode::Right => self.mv(Dir::Right),
            _ => {}
        }
    }

    pub fn step(&mut self) {
        self.mv(Dir::Down);
    }

    fn mv(&mut self, dir: Dir) {
        match dir {
            Dir::Up => {
                if self.pos.y > 0 {
                    self.pos.y -= 1
                }
            }
            Dir::Down => {
                if self.pos.y >= HEIGHT - 1 || self.cells[self.pos.y + 1][self.pos.x] {
                    self.cells[self.pos.y][self.pos.x] = true;
                    self.pos.y = 0;
                } else {
                    self.pos.y += 1;
                }
            }
            Dir::Left => {
                if self.pos.x > 0 {
                    self.pos.x -= 1
                }
            }
            Dir::Right => {
                if self.pos.x < WIDTH - 1 {
                    self.pos.x += 1
                }
            }
        }
    }
}

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn draw_char(x: usize, y: usize, c: char) -> Result<(), io::Error> {
    io::stdout()
        .execute(cursor::MoveTo(x as u16, y as u16))?
        .execute(style::Print(c))?;
    Ok(())
}
