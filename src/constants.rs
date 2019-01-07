pub const TITLE: &'static str = "Tetris in Rust";

pub const NUM_BLOCKS_X: usize = 10;
pub const NUM_BLOCKS_Y: usize = 18;

pub const BORDER_WIDTH: u32 = 1;
pub const TEXTURE_SIZE: u32 = 32;
pub const TEXTURE_SIZE_INNER: u32 = TEXTURE_SIZE - BORDER_WIDTH * 2;

pub const WIDTH: u32 = NUM_BLOCKS_X as u32 * TEXTURE_SIZE; // 480;
pub const HEIGHT: u32 = NUM_BLOCKS_Y as u32 * TEXTURE_SIZE; // 860;