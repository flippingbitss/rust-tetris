use crate::piece::Piece;
use crate::constants::*;
use crate::others::{Presence,PieceType};
use rand::random;

pub struct Game {
    pub current_piece: Option<Piece>,
    pub score: usize,
    pub lines_cleared: usize,
    pub game_map: Vec<Vec<Presence>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_piece: Some(Piece::from(random::<PieceType>())),
            score: 0,
            lines_cleared: 0,
            game_map: vec![vec![Presence::No; NUM_BLOCKS_X]; NUM_BLOCKS_Y],
        }
    }
}