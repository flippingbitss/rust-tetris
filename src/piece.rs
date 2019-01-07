use crate::game_color::GameColor;
use std::default::Default;
use crate::others::{GameMap, PieceMatrix, Presence, PieceType};

#[derive(Default, Copy, Clone)]
pub struct Piece {
    pub states: [u16; 4],
    pub color: GameColor,
    pub x: isize,
    pub y: isize,
    pub current_state: usize,
}

impl Piece {
    pub fn rotate(&mut self, game_map: &GameMap) {
        let temp_state = (self.current_state + 1) % 4;
        let x_pos_matches = [0, -1, 1, -2, 2, -3];

        for x in x_pos_matches.iter() {
            if self.test_position(&game_map, temp_state, *x as isize, self.y) {
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

    fn test_position(&self, game_map: &[Vec<Presence>], state: usize, x: isize, y: isize) -> bool {
        let state_m = self.get_block_matrix(state);

        for mx in 0..4 {
            for my in 0..4 {
                if state_m[my][mx] != Presence::No {
                    if x as usize + mx >= game_map[y as usize].len()
                        || y as usize + my >= game_map.len()
                        || game_map[y as usize + my][x as usize + mx] != Presence::No
                        {
                            return false;
                        }
                }
            }
        }
        true
    }

    fn freeze(&self, game_map: &mut [Vec<Presence>]) {
        let state = self.get_block_matrix(self.current_state);

        for dx in 0..4 {
            for dy in 0..4 {
                let cell = state[dy][dx];
                if cell != Presence::No {
                    game_map[self.y as usize + dy][self.x as usize + dx] = cell;
                }
            }
        }
    }
}

impl From<PieceType> for Piece {
    fn from(piece_type: PieceType) -> Piece {
        use self::PieceType::*;

        let def = Piece::default();

        match piece_type {
            L => Piece {
                states: [17504, 736, 1570, 1856],
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
                states: [17984, 3648, 17984, 3648],
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
