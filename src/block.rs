use crate::draw::draw_text;

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

const WIDTH: usize = 4;
const HEIGHT: usize = 4;

#[derive(Clone, Copy)]
pub struct Block {
    pub pos: Pos,
    pub cells: [[bool; WIDTH]; HEIGHT],
}

#[derive(Clone, Copy)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

#[allow(dead_code)]
pub enum RotDir {
    Clockwise,
    CounterClockwise,
}

impl Block {
    pub fn new(variant: BlockType, x: isize, y: isize) -> Self {
        Self {
            pos: Pos { x, y },
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
            pos: self.pos,
        }
    }

    pub fn draw(&self, pos_x: isize, pos_y: isize) -> crossterm::Result<()> {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.cells[y][x] {
                    draw_text(
                        self.pos.x + x as isize + pos_x as isize,
                        self.pos.y + y as isize + pos_y as isize,
                        "#",
                    )?;
                }
            }
        }

        Ok(())
    }
}

#[test]
fn rotate_block() {
    let mut block = Block::new(BlockType::L, 0, 0);
    block = block.rotate(RotDir::Clockwise);
    assert_eq!(
        block.cells,
        [
            [false, false, false, false],
            [true, true, true, true],
            [true, false, false, false],
            [false, false, false, false],
        ]
    )
}
