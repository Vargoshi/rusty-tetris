use std::io;

use crossterm::{event, terminal, ExecutableCommand};

use crate::{
    block::{Block, RotDir},
    draw::draw_char,
};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

pub struct Board {
    block: Block,
    cells: [[bool; WIDTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            block: Block::new(crate::block::BlockType::L, WIDTH as isize / 2, 0),
            cells: Default::default(),
        }
    }

    pub fn draw(&self) -> crossterm::Result<()> {
        io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;

        for y in 0..HEIGHT + 2 {
            for x in 0..WIDTH + 2 {
                if y == 0 || y == HEIGHT + 1 || x == 0 || x == WIDTH + 1 {
                    draw_char(x as isize, y as isize, '+')?;
                }
            }
        }

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.cells[y][x] {
                    draw_char(x as isize + 1, y as isize + 1, '@')?;
                }
            }
        }

        self.block.draw()?;

        Ok(())
    }

    pub fn input(&mut self, code: event::KeyCode) {
        match code {
            event::KeyCode::Up => self.mv(Dir::Up),
            event::KeyCode::Down => self.mv(Dir::Down),
            event::KeyCode::Left => self.mv(Dir::Left),
            event::KeyCode::Right => self.mv(Dir::Right),
            event::KeyCode::Char(' ') => self.rotate(),
            _ => {}
        }
    }

    fn rotate(&mut self) {
        let mut rotated = self.block.rotate(RotDir::Clockwise);
        if !self.is_block_collision(&rotated) {
            self.block = rotated;
            return;
        }
        rotated.pos.x += 1;
        if !self.is_block_collision(&rotated) {
            self.block = rotated;
            return;
        }
        rotated.pos.x -= 2;
        if !self.is_block_collision(&rotated) {
            self.block = rotated;
            return;
        }
    }

    pub fn step(&mut self) {
        self.mv(Dir::Down);
    }

    fn mv(&mut self, dir: Dir) {
        match dir {
            Dir::Up => {
                // if self.block.pos.y > 0 {
                //     self.block.pos.y -= 1
                // }
            }
            Dir::Down => {
                if self.is_collision(0, 1) {
                    self.drop_block();
                    self.try_clear();
                    self.block.pos.y = 0;
                } else {
                    self.block.pos.y += 1;
                }
            }

            Dir::Left => {
                if !self.is_collision(-1, 0) {
                    self.block.pos.x -= 1
                }
            }

            Dir::Right => {
                if !self.is_collision(1, 0) {
                    self.block.pos.x += 1
                }
            }
        }
    }

    fn is_collision(&self, dx: isize, dy: isize) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if self.block.cells[y][x] {
                    let abs_x = self.block.pos.x + x as isize + dx;
                    let abs_y = self.block.pos.y + y as isize + dy;

                    if abs_y >= HEIGHT as isize
                        || abs_y < 0
                        || abs_x >= WIDTH as isize
                        || abs_x < 0
                        || self.cells[abs_y as usize][abs_x as usize]
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn is_block_collision(&self, block: &Block) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if block.cells[y][x] {
                    let abs_x = block.pos.x + x as isize;
                    let abs_y = block.pos.y + y as isize;

                    if abs_y >= HEIGHT as isize
                        || abs_y < 0
                        || abs_x >= WIDTH as isize
                        || abs_x < 0
                        || self.cells[abs_y as usize][abs_x as usize]
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Adds cells of the currently falling block to the board cells.
    fn drop_block(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                if self.block.cells[y][x] {
                    let abs_x = (self.block.pos.y + y as isize) as usize;
                    let abs_y = (self.block.pos.x + x as isize) as usize;
                    self.cells[abs_x][abs_y] = true;
                }
            }
        }
    }

    fn try_clear(&mut self) {
        for y in 0..HEIGHT {
            if self.should_clear(y) {
                self.clear(y);
            }
        }
    }

    fn should_clear(&mut self, row: usize) -> bool {
        for x in 0..WIDTH {
            if self.cells[row][x] == false {
                return false;
            }
        }
        true
    }

    fn clear(&mut self, row: usize) {
        for y in (0..row).rev() {
            for x in 0..WIDTH {
                self.cells[y + 1][x] = self.cells[y][x];
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
