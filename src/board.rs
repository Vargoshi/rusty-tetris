use sdl2::{keyboard::Keycode, rect::{Point, Rect}, render::Canvas, ttf::Font, video::Window};

use crate::{block::{Block, BlockType, RotDir, BLOCK_SIZE}, draw::draw_text};

pub struct Board {
    size: Point,
    block: Block,
    block_pos: Point,
    cells: Vec<Vec<bool>>,
    pub score: usize,
}

impl Board {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            size: Point::new(w, h),
            block: Block::new(BlockType::rand()),
            block_pos: Point::new(w as i32 / 2, 0),
            cells: vec![vec![false; w as usize]; h as usize],
            score: 0,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, rect: Rect, font: &Font) -> Result<(), String> {
        // Border
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(200, 200, 200, 255));
        canvas.draw_rect(Rect::new(
            rect.x - 2,
            rect.y - 2,
            rect.w as u32 + 4,
            rect.h as u32 + 4,
        ))?;
        let cell_w = rect.w / self.size.x;
        let cell_h = rect.h / self.size.y;

        // Fallen blocks
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                if self.cells[y as usize][x as usize] {
                    canvas.set_draw_color(sdl2::pixels::Color::RGBA(200, 200, 200, 255));
                    canvas.fill_rect(Rect::new(
                        rect.x + x as i32 * cell_w + 2,
                        rect.y + y as i32 * cell_h + 2,
                        cell_w as u32 - 4,
                        cell_h as u32 - 4,
                    ))?;
                }
            }
        }
        self.block.draw(
            canvas,
            Rect::new(
                (rect.x + self.block_pos.x * cell_w) as i32,
                (rect.y + self.block_pos.y * cell_h) as i32,
                BLOCK_SIZE as u32 * cell_w as u32,
                BLOCK_SIZE as u32 * cell_h as u32,
            ),
        )?;

        let msg = format!("Score: {}", self.score);
        draw_text(canvas, font, Point::new(400, 100), &msg, 0.2)?;
        // draw_text(abs_x, abs_y + HEIGHT as isize + 3, &msg)?;

        Ok(())
    }

    pub fn input(&mut self, code: Keycode) {
        match code {
            Keycode::Up => {
                self.mv(Dir::Up);
            }
            Keycode::Down => {
                self.mv(Dir::Down);
            }
            Keycode::Left => {
                self.mv(Dir::Left);
            }
            Keycode::Right => {
                self.mv(Dir::Right);
            }
            Keycode::Space => {
                self.rotate();
            }
            _ => {}
        }
    }

    fn rotate(&mut self) {
        let rotated_block = self.block.rotate(RotDir::Clockwise);
        let pos = self.block_pos;
        if !self.check_collision(&rotated_block, pos) {
            self.block = rotated_block;
            return;
        }
        let pos = self.block_pos.offset(1, 0);
        if !self.check_collision(&rotated_block, pos) {
            self.block = rotated_block;
            self.block_pos = pos;
            return;
        }
        let pos = self.block_pos.offset(-1, 0);
        if !self.check_collision(&rotated_block, pos) {
            self.block = rotated_block;
            self.block_pos = pos;
        }
    }

    pub fn step(&mut self) -> bool {
        self.mv(Dir::Down)
    }

    fn mv(&mut self, dir: Dir) -> bool {
        match dir {
            Dir::Up => false,
            Dir::Down => {
                if self.check_collision(&self.block, self.block_pos.offset(0, 1)) {
                    self.drop_block();
                    self.try_clear();
                    self.block = Block::new(BlockType::rand());
                    self.block_pos = Point::new(self.size.x / 2, 0);
                    if self.check_collision(&self.block, self.block_pos) {
                        return true;
                    }
                } else {
                    self.block_pos.y += 1;
                }
                false
            }
            Dir::Left => {
                if !self.check_collision(&self.block, self.block_pos.offset(-1, 0)) {
                    self.block_pos.x -= 1;
                }
                false
            }
            Dir::Right => {
                if !self.check_collision(&self.block, self.block_pos.offset(1, 0)) {
                    self.block_pos.x += 1;
                }
                false
            }
        }
    }

    fn check_collision(&self, block: &Block, pos: Point) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if block.cells[y][x] {
                    let abs_x = pos.x + x as i32;
                    let abs_y = pos.y + y as i32;

                    if abs_y >= self.size.y
                        || abs_y < 0
                        || abs_x >= self.size.x
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
        for y in 0..BLOCK_SIZE {
            for x in 0..BLOCK_SIZE {
                if self.block.cells[y][x] {
                    let abs_x = (self.block_pos.y + y as i32) as usize;
                    let abs_y = (self.block_pos.x + x as i32) as usize;
                    self.cells[abs_x][abs_y] = true;
                }
            }
        }
    }

    fn try_clear(&mut self) {
        let mut lines = Vec::new();

        for y in 0..self.size.y {
            if self.should_clear(y as usize) {
                self.clear(y as usize);
                lines.push(self.size.y - y);
            }
        }

        for line in &lines {
            self.score += get_score(*line as usize, lines.len());
        }
    }

    fn should_clear(&mut self, row: usize) -> bool {
        for x in 0..self.size.x {
            if !self.cells[row][x as usize] {
                return false;
            }
        }
        true
    }

    fn clear(&mut self, row: usize) {
        for y in (0..row).rev() {
            for x in 0..self.size.x {
                self.cells[y + 1][x as usize] = self.cells[y][x as usize];
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
