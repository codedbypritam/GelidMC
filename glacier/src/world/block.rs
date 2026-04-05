pub type BlockState = u16;

struct Block {
    pub state: BlockState
}

impl Block {
    fn new(state: BlockState) -> Self {
        Self { state }
    }

    fn id(&self) -> u8 {
        (self.state >> 4) as u8
    }

    fn meta(&self) -> u8 {
        (self.state & 0xF) as u8
    }
}

#[inline(always)]
pub const fn make_block_state(id: u8, meta: u8) -> BlockState {
    ((id as u16) << 4) | (meta as u16 & 0xF)
}

// BLOCK CONSTANTS

pub const AIR: BlockState = make_block_state(0, 0);
pub const BEDROCK: BlockState = make_block_state(1, 0);
pub const DIRT: BlockState = make_block_state(2, 0);
pub const GRASS_BLOCK: BlockState = make_block_state(3, 0);