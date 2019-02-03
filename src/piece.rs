use crate::game_color::GameColor;
use std::default::Default;
use crate::others::{GameMap, PieceMatrix, Presence, PieceType};
use crate::constants::{NUM_BLOCKS_Y,NUM_BLOCKS_X};
use rand::{random, thread_rng, Rng};

#[derive(Default, Copy, Clone)]
pub struct Piece {
    pub states: [u16; 4],
    pub color: GameColor,
    pub x: isize,
    pub y: isize,
    pub current_state: usize,
}

impl Piece {
    pub fn random() -> Self {
        let mut p = Piece::from(random::<PieceType>());
        p.x  = thread_rng().gen_range(0, NUM_BLOCKS_X - 2) as isize;
        p.y = -1;
        p
    }

    pub fn rotate(&mut self, game_map: &GameMap) {
        let temp_state = (self.current_state + 1) % 4;
        let x_pos_matches = [0, -1, 1, -2, 2, -3];
        for x in x_pos_matches.iter() {
            if self.test_position(&game_map, temp_state, self.x + *x as isize, self.y) {
                self.current_state = temp_state;
                self.x += *x;
                break;
            }
        }
    }

    pub fn move_position(&mut self, game_map: &GameMap, new_x: isize, new_y: isize) -> bool {
        if self.test_position(&game_map, self.current_state, new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
            return true;
        }
        false
    }

    pub fn get_block_matrix(&self, state: usize) -> PieceMatrix {
        use self::Presence::*;
        let num = self.states[state];
        let mut res = [[No; 4]; 4];

        for i in 0..16 {
            if num & 1u16 << 15 - i > 0 {
                res[i / 4][i % 4] = Yes(self.color)
            }
        }
        res
    }

    pub fn test_position(&self, game_map: &[Vec<Presence>], state: usize, x: isize, y: isize) -> bool {
        let state_m = self.get_block_matrix(state);

        for mx in 0..4isize {
            for my in 0..4isize {
                if state_m[my as usize][mx as usize] != Presence::No {
                    if x + mx < 0 || y + my < 0 {
                        return y < 0;
                    }
                    if x + mx >= NUM_BLOCKS_X as isize ||
                        y + my >= NUM_BLOCKS_Y as isize ||
                        game_map[(y + my) as usize][(x + mx) as usize] != Presence::No {
                            return false;
                    }
                }
            }
        }
        true
    }

    pub fn freeze(&self, game_map: &mut [Vec<Presence>]) {
        let state = self.get_block_matrix(self.current_state);

        for dx in 0..4 {
            for dy in 0..4 {
                let cell = state[dy][dx];
                if cell != Presence::No {
                    let x = self.x + dx as isize;
                    let y = self.y + dy as isize;
                    game_map[y as usize][x as usize] = cell;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_filled_region(&self, matrix: PieceMatrix) -> (usize, usize, usize, usize) {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (4,4,0,0);

        for dx in 0..4 {
            for dy in 0..4 {
                let cell = matrix[dy][dx];
                if cell != Presence::No {
                    if dx < min_x { min_x = dx }
                    if dy < min_y { min_y = dy }
                    if dx > max_x { max_x = dx }
                    if dy > max_y { max_y = dy }
                }
            }
        }
        (min_x, max_x + 1, min_y, max_y + 1)
    }

}

impl From<PieceType> for Piece {
    fn from(piece_type: PieceType) -> Piece {
        use self::PieceType::*;

        let def = Piece::default();

        match piece_type {
            L => Piece {
                states: [17504, 1856, 1570, 736],
                color: GameColor::Orange,
                ..def
            },
            J => Piece {
                states: [8800, 1136, 1604, 3616],
                color: GameColor::Blue,
                ..def
            },
            S => Piece {
                states: [17952, 1728, 17952, 1728],
                color: GameColor::Green,
                ..def
            },
            Z => Piece {
                states: [9792, 3168, 9792, 3168],
                color: GameColor::Red,
                ..def
            },
            T => Piece {
                states: [17984, 3648, 19520, 19968],
                color: GameColor::Purple,
                ..def
            },
            I => Piece {
                states: [17476, 3840, 17476, 3840],
                color: GameColor::Cyan,
                ..def
            },
            O => Piece {
                states: [1632, 1632, 1632, 1632],
                color: GameColor::Yellow,
                ..def
            },
        }
    }
}
