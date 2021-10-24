use sdl2::{rect::Rect, render::Canvas, video::Window};



pub enum BlockType {
    Square,
    L,
    I,
    Z,
    T,
}

impl BlockType {
    pub fn rand() -> Self {
        let x: u16 = rand::random();
        match x % 5 {
            0 => BlockType::Square,
            1 => BlockType::L,
            2 => BlockType::I,
            3 => BlockType::Z,
            4 => BlockType::T,
            _ => unreachable!(),
        }
    }
}

pub const BLOCK_SIZE: usize = 4;

#[derive(Clone, Copy)]
pub struct Block {
    pub cells: [[bool; BLOCK_SIZE ]; BLOCK_SIZE ],
}

#[allow(dead_code)]
pub enum RotDir {
    Clockwise,
    CounterClockwise,
}

impl Block {
    pub fn new(variant: BlockType) -> Self {
        Self {
            cells: match variant {
                BlockType::Square => [
                    [false, false, false, false],
                    [false, true, true, false],
                    [false, true, true, false],
                    [false, false, false, false],
                ],
                BlockType::L => [
                    [false, false, false, false],
                    [false, true, false, false],
                    [false, true, false, false],
                    [false, true, true, false],
                ],
                BlockType::I => [
                    [false, true, false, false],
                    [false, true, false, false],
                    [false, true, false, false],
                    [false, true, false, false],
                ],
                BlockType::Z => [
                    [false, false, false, false],
                    [true, true, false, false],
                    [false, true, true, false],
                    [false, false, false, false],
                ],
                BlockType::T => [
                    [false, false, false, false],
                    [true, true, true, false],
                    [false, true, false, false],
                    [false, false, false, false],
                ],
            },
        }
    }

    pub fn rotate(&self, dir: RotDir) -> Self {
        let mut rotated = [[false; 4]; 4];

        for y in 0..4 {
            #[allow(clippy::needless_range_loop)]
            for x in 0..4 {
                match dir {
                    RotDir::Clockwise => {
                        rotated[x][y] = self.cells[3 - y][x];
                    }
                    RotDir::CounterClockwise => {
                        rotated[x][y] = self.cells[y][3 - x];
                    }
                }
            }
        }

        Self {
            cells: rotated,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, rect: Rect) -> Result<(), String> {
        let cell_w = rect.w / BLOCK_SIZE as i32;
        let cell_h = rect.h / BLOCK_SIZE as i32;
        for y in 0..BLOCK_SIZE {
            for x in 0..BLOCK_SIZE {
                if self.cells[y][x] {
                    canvas.set_draw_color(sdl2::pixels::Color::RGBA(200, 0, 0, 255));
                      canvas.fill_rect(Rect::new(
                        rect.x + x as i32 * cell_w + 2,
                        rect.y + y as i32 * cell_h + 2,
                        cell_w as u32 - 4,
                        cell_h as u32 - 4,
                      ))?;
                }
            }
        }
        Ok(())
    }
}

#[test]
fn rotate_block() {
    let mut block = Block::new(BlockType::L);
    block = block.rotate(RotDir::Clockwise);
    assert_eq!(
        block.cells,
        [
            [false, false, false, false],
            [true, true, true, false],
            [true, false, false, false],
            [false, false, false, false],
        ]
    )
}
