use rand::distributions::Standard;
use rand::prelude::*;
use sdl2::pixels::Color;
use std::default::Default;

#[derive(Debug)]
pub enum GameColor {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Orange,
    Purple,
    Gray,
}

impl Default for GameColor {
    fn default() -> Self {
        GameColor::Red
    }
}

impl From<GameColor> for Color {
    fn from(color: GameColor) -> Self {
        match color {
            GameColor::Red => Color::RGB(255, 0, 0),
            GameColor::Green => Color::RGB(0, 255, 0),
            GameColor::Blue => Color::RGB(0, 0, 255),
            GameColor::Yellow => Color::RGB(255, 255, 0),
            GameColor::Cyan => Color::RGB(0, 255, 255),
            GameColor::Orange => Color::RGB(255, 165, 0),
            GameColor::Purple => Color::RGB(128, 0, 128),
            GameColor::Gray => Color::RGB(25, 25, 25),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PieceType {
    J,
    L,
    S,
    Z,
    T,
    I,
    O,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Presence {
    No,
    Yes,
}

type PieceMatrix = [[Presence; 4]; 4];
type GameMap = [Vec<Presence>];

#[derive(Default)]
pub struct Piece {
    pub states: [u16; 4],
    pub color: GameColor,
    pub x: isize,
    pub y: usize,
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

    pub fn move_position(&mut self, game_map: &GameMap, new_x: isize, new_y: usize) -> bool {
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
                res[i / 4][i % 4] = Yes
            }
        }
        res
    }

    fn test_position(&self, game_map: &[Vec<Presence>], state: usize, x: isize, y: usize) -> bool {
        let state_m = self.get_block_matrix(state);

        for mx in 0..4 {
            for my in 0..4 {
                if state_m[my][mx] == Presence::Yes {
                    if x as usize + mx >= game_map[y].len()
                        || y + my >= game_map.len()
                        || game_map[y + my][x as usize + mx] == Presence::Yes
                    {
                        return false;
                    }
                }
            }
        }
        true
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

impl Distribution<PieceType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
        use self::PieceType::*;
        rng.choose(&[L, J, S, Z, T, I, O]).unwrap().clone()
    }
}
