use sdl2::pixels::Color;
use std::default::Default;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameColor {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Orange,
    Purple,
    Gray,
    Pink
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
            GameColor::Pink => Color::RGB(255, 105, 180),
        }
    }
}