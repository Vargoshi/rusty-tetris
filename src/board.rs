use std::io;

use crossterm::{event, terminal, ExecutableCommand};

use crate::{
    block::{Block, BlockType, RotDir},
    draw::draw_text,
};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

pub struct Board {
    block: Block,
    cells: [[bool; WIDTH]; HEIGHT],
    score: usize,
}

impl Board {
    pub fn new() -> Self {
        Self {
            block: Block::new(BlockType::rand(), WIDTH as isize / 2, 0),
            cells: Default::default(),
            score: 0,
        }
    }

    pub fn draw(&self, pos_x: isize, pos_y: isize) -> crossterm::Result<()> {
        io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;
        let abs_x = pos_x - (WIDTH as isize + 2) / 2;
        let abs_y = pos_y - (HEIGHT as isize + 2) / 2;

        for y in 0..HEIGHT + 2 {
            for x in 0..WIDTH + 2 {
                if y == 0 || y == HEIGHT + 1 || x == 0 || x == WIDTH + 1 {
                    draw_text(abs_x + x as isize, abs_y + y as isize, "+")?;
                }
            }
        }

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.cells[y][x] {
                    draw_text(abs_x + 1 + x as isize, abs_y + 1 + y as isize, "@")?;
                }
            }
        }

        self.block.draw(abs_x + 1, abs_y + 1)?;

        let msg = format!("Score: {}", self.score);
        draw_text(abs_x, abs_y + HEIGHT as isize + 3, &msg)?;

        Ok(())
    }

    pub fn input(&mut self, code: event::KeyCode) {
        match code {
            event::KeyCode::Up => {
                self.mv(Dir::Up);
            }
            event::KeyCode::Down => {
                self.mv(Dir::Down);
            }
            event::KeyCode::Left => {
                self.mv(Dir::Left);
            }
            event::KeyCode::Right => {
                self.mv(Dir::Right);
            }
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

    pub fn step(&mut self) -> bool {
        self.mv(Dir::Down)
    }

    fn mv(&mut self, dir: Dir) -> bool {
        match dir {
            Dir::Up => false,
            Dir::Down => {
                if self.is_collision(0, 1) {
                    self.drop_block();
                    self.try_clear();
                    self.block = Block::new(BlockType::rand(), WIDTH as isize / 2, 0);
                    if self.is_collision(0, 0) {
                        return true;
                    }
                } else {
                    self.block.pos.y += 1;
                }
                false
            }
            Dir::Left => {
                if !self.is_collision(-1, 0) {
                    self.block.pos.x -= 1
                }
                false
            }
            Dir::Right => {
                if !self.is_collision(1, 0) {
                    self.block.pos.x += 1
                }
                false
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
        let mut lines = Vec::new();

        for y in 0..HEIGHT {
            if self.should_clear(y) {
                self.clear(y);
                lines.push(HEIGHT - y);
            }
        }

        for line in &lines {
            self.score += get_score(*line, lines.len());
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

/// Calculate score for removed line.
/// `y` - Line number counting from bottom (1 is first).
/// `lines` - number of lines removed in total.
pub fn get_score(y: usize, lines: usize) -> usize {
    match lines {
        0 => 0,
        1 => 40 * y,
        2 => 100 * y,
        3 => 300 * y,
        4 => 1200 * y,
        _ => unreachable!(),
    }
}

enum Dir {
    Left,
    Right,
    Up,
    Down,
}
