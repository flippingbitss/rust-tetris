use crate::piece::Piece;
use crate::constants::*;
use crate::others::{Presence,PieceType};
use rand::random;

pub type GameMap = Vec<Vec<Presence>>;

pub struct Game {
    pub current_piece: Option<Piece>,
    pub score: usize,
    pub lines_cleared: usize,
    pub map: Vec<Vec<Presence>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_piece: Some(Piece::from(random::<PieceType>())),
            score: 0,
            lines_cleared: 0,
            map: vec![vec![Presence::No; NUM_BLOCKS_X]; NUM_BLOCKS_Y],
        }
    }

    pub fn check_lines(&mut self) {
        let mut to_clear = vec![];

        for y in 0..NUM_BLOCKS_Y {
            if self.map[y].iter().all(|&x| x != Presence::No) {
                to_clear.push(y);
            }
        }

        self.lines_cleared += to_clear.len();
        println!("cleared {}", self.lines_cleared);

        for index in to_clear.into_iter() {
            self.map.remove(index);
            self.map.insert(0, vec![Presence::No; NUM_BLOCKS_X]);
        }
    }

    pub fn finalize_move(&mut self, piece: &mut Piece) {
        piece.freeze(&mut self.map);
        self.check_lines();
        *piece = Piece::random();
    }
}