use rand::distributions::Standard;
use rand::prelude::*;
use crate::game_color::GameColor;


pub type PieceMatrix = [[Presence; 4]; 4];
pub type GameMap = [Vec<Presence>];


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Presence {
    No,
    Yes(GameColor),
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

impl Distribution<PieceType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
        use self::PieceType::*;
        rng.choose(&[L, J, S, Z, T, I, O]).unwrap().clone()
    }
}
