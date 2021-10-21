pub enum BlockType {
    Square,
    L,
    I,
    Z,
    T,
}

pub struct Block {
    cells: [[bool; 4]; 4],
}

pub enum Dir {
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

    pub fn draw(&self) {
        for y in 0..4 {
            for x in 0..4 {
                if self.cells[y][x] == true {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

#[test]
fn rotate_block() {
    let mut block = Block::new(BlockType::L);
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
