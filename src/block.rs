use crate::draw::draw_char;

pub enum BlockType {
    Square,
    L,
    I,
    Z,
    T,
}

const WIDTH: usize = 4;
const HEIGHT: usize = 4;

pub struct Block {
    pub pos: Pos,
    pub cells: [[bool; WIDTH]; HEIGHT],
}

pub struct Pos {
    pub x: isize,
    pub y: isize,
}

pub enum Dir {
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
                    [false, true, false, false],
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

    pub fn rotate(&mut self, dir: Dir) {
        let mut rotated = [[false; 4]; 4];

        for y in 0..4 {
            for x in 0..4 {
                match dir {
                    Dir::Clockwise => {
                        rotated[x][y] = self.cells[3 - y][x];
                    }
                    Dir::CounterClockwise => {
                        rotated[x][y] = self.cells[y][3 - x];
                    }
                }
            }
        }

        self.cells = rotated;
    }

    pub fn draw(&self) -> crossterm::Result<()> {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.cells[y][x] {
                    draw_char(self.pos.x + x as isize + 1, self.pos.y + y as isize + 1, '#')?;
                }
            }
        }

        Ok(())
    }
}

#[test]
fn rotate_block() {
    let mut block = Block::new(BlockType::L, 0, 0);
    block.rotate(Dir::Clockwise);
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
