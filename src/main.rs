mod block;

use block::{Block, BlockType, Dir};

fn main() {
    let mut block = Block::new(BlockType::L);
    block.rotate(Dir::Clockwise);
    block.draw();
}
