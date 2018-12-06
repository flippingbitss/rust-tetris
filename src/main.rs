extern crate sdl2;

mod lib;

use rand::prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

use std::thread::sleep;
use std::time::Duration;

use crate::lib::{GameColor, Piece, PieceType, Presence};

// constants
const TITLE: &'static str = "Tetris in Rust";

const NUM_BLOCKS_X: u32 = 10;
const NUM_BLOCKS_Y: u32 = 18;

const BORDER_WIDTH: u32 = 1;
const TEXTURE_SIZE: u32 = 32;
const TEXTURE_SIZE_INNER: u32 = TEXTURE_SIZE - BORDER_WIDTH * 2;

const WIDTH: u32 = NUM_BLOCKS_X * TEXTURE_SIZE; // 480;
const HEIGHT: u32 = NUM_BLOCKS_Y * TEXTURE_SIZE; //860;

struct Game {
    current_piece: Piece,
    score: usize,
    lines_cleared: usize,
    game_map: Vec<Vec<Presence>>
}

impl Game {
    fn new() -> Game {
        Game {
            current_piece: Piece::from(random::<PieceType>()),
            score: 0,
            lines_cleared: 0,
            game_map: vec![vec![Presence::No; NUM_BLOCKS_X as usize]; NUM_BLOCKS_Y as usize]
        }
    }
}

// initialize sdl context and canvas
fn main() {
    let sdl_context = sdl2::init().expect("sdl initialization failed");

    let video_subsystem = sdl_context
        .video()
        .expect("failed to get sdl video subsystem");

    let window = video_subsystem
        .window(TITLE, WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Failed to build canvas");

    let texture_creator = canvas.texture_creator();

    macro_rules! texture {
        ($color: expr) => {
            create_texture_rect(&mut canvas, &texture_creator, $color)
                .expect("Failed to create texture");
        };
    }

    // set canvas background and clear it
    canvas.set_draw_color(GameColor::Blue);
    canvas.clear();

    let textures = [texture!(GameColor::Red), texture!(GameColor::Gray)];



    let mut game = Game::new();

    draw_tetris_piece(&mut canvas, &textures, &game);

    //    println!("{:?}", format!("{:b}",curr).split(""));
    //    println!("{:?}", mat);
    canvas.present();

    start_render_loop(&sdl_context, &mut game)
}

fn draw_tetris_piece(
    canvas: &mut Canvas<Window>,
    textures: &[Texture; 2],
    game: &Game
) {
    let piece = &game.current_piece;
    let mat = piece.get_block_matrix(piece.current_state);
    let [block_tex, border_tex] = textures;
    let x = piece.x;
    let y = piece.y;

    for row in 0..4 {
        for col in 0..4 {
            if mat[row][col] == Presence::Yes {
                let y_offset = (y + row * TEXTURE_SIZE as usize) as i32;
                let x_offset = (x as usize + col * TEXTURE_SIZE as usize) as i32;

                canvas.copy(
                    &border_tex,
                    None,
                    Rect::new(y_offset, x_offset, TEXTURE_SIZE, TEXTURE_SIZE),
                );

                canvas.copy(
                    &block_tex,
                    None,
                    Rect::new(
                        y_offset + BORDER_WIDTH as i32,
                        x_offset + BORDER_WIDTH as i32,
                        TEXTURE_SIZE_INNER,
                        TEXTURE_SIZE_INNER,
                    ),
                );
            }
        }
    }
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    texture_color: GameColor,
) -> Option<Texture<'a>> {
    let result = texture_creator.create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE);

    if let Ok(mut square_texture) = result {
        canvas
            .with_texture_canvas(&mut square_texture, |texture| {
                texture.set_draw_color(texture_color);
                texture.clear();
            })
            .expect("Failed texture drawing");

        Some(square_texture)
    } else {
        None
    }
}

fn start_render_loop(sdl_context: &Sdl, game: &mut Game) {
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get sdl context event pump");

    // loop till we receive exit signal QUIT/ESCAPE key
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'main;
                }
                _ => {}
            }
        }

        sleep(Duration::new(0, 1_000_000_000u32 / 60)); // for 60 fps TODO: use better time sync
    }
}
