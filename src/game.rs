use crate::piece::Piece;
use crate::constants::*;
use crate::others::{Presence,PieceType};
use rand::random;

pub type GameMap = Vec<Vec<Presence>>;

pub struct Game {
    pub current_piece: Option<Piece>,
    pub score: usize,
    pub lines_cleared: usize,
    pub current_level: usize,
    pub map: Vec<Vec<Presence>>,
    pub quit: bool
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_piece: Some(Piece::from(random::<PieceType>())),
            score: 0,
            lines_cleared: 0,
            current_level: 0,
            map: vec![vec![Presence::No; NUM_BLOCKS_X]; NUM_BLOCKS_Y],
            quit: false
        }
    }

    pub fn check_lines(&mut self) {
        let mut to_clear = vec![];

        for y in 0..NUM_BLOCKS_Y {
            if self.map[y].iter().all(|&x| x != Presence::No) {
                to_clear.push(y);
            }
        }

        self.increase_lines(to_clear.len());

        for index in to_clear.into_iter() {
            self.map.remove(index);
            self.map.insert(0, vec![Presence::No; NUM_BLOCKS_X]);
        }
    }

    pub fn finalize_move(&mut self, piece: &mut Piece) {
        if piece.y < 0 {
            self.quit = true;
        } else {
            piece.freeze(&mut self.map);
            self.check_lines();
            *piece = Piece::random();
        }
    }

    fn increase_lines(&mut self, delta: usize) {
        self.lines_cleared += delta;
        if self.lines_cleared > LEVEL_LINES[self.current_level] {
            self.current_level = usize::max(self.current_level + 1, NUM_LEVELS - 1)
        }
    }

    pub fn get_shadow_piece(&self) -> Piece {
        let mut p = self.current_piece.unwrap().clone();
        while p.move_position(&self.map, p.x, p.y + 1) {}
        p
    }
}