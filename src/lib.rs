use sdl2::pixels::Color;

pub enum GameColor {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Orange,
    Purple,
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
        }
    }
}

pub enum PieceType {
    J,
    L,
    S,
    Z,
    T,
    I,
    O,
}

pub struct Piece {
    build: [u16; 4],
    color: GameColor,
    x: usize,
    y: usize,
    current_build: usize
}

impl From<PieceType> for Piece {
    fn from(piece_type: PieceType) -> Self {
        match piece_type {
            PieceType::L => Piece {
                build: [17504, 736, 1570, 1856],
                color: GameColor::Orange,
                x: 0,
                y: 0,
                current_build: 0
            },
            PieceType::J => Piece {
                build: [8800, 1136, 1604, 3616],
                color: GameColor::Blue,
                x: 0,
                y: 0,
                current_build: 0
            },
            PieceType::S => Piece {
                build: [17952, 1728, 17952, 1728],
                color: GameColor::Green,
                x: 0,
                y: 0,
                current_build: 0
            },
            PieceType::Z => Piece {
                build: [9792, 3168, 9792, 3168],
                color: GameColor::Red,
                x: 0,
                y: 0,
                current_build: 0
            },
            PieceType::T => Piece {
                build: [17984, 3648, 17984, 3648],
                color: GameColor::Purple,
                x: 0,
                y: 0,
                current_build: 0
            },
            PieceType::I => Piece {
                build: [17476, 3840, 17476, 3840],
                color: GameColor::Cyan,
                x: 0,
                y: 0,
                current_build: 0
            },
            PieceType::O => Piece {
                build: [1632, 1632, 1632, 1632],
                color: GameColor::Yellow,
                x: 0,
                y: 0,
                current_build: 0
            },
        }
    }
}

