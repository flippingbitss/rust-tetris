extern crate sdl2;

mod constants;
mod others;
mod game;
mod game_color;
mod renderer;
mod piece;

use rand::prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture};
use sdl2::video::{Window};
use sdl2::Sdl;

use std::thread::sleep;
use std::time::Duration;

use crate::others::{PieceType, Presence};
use crate::game_color::GameColor;
use crate::piece::Piece;
use crate::game::Game;
use crate::constants::*;
use crate::renderer::{create_window, draw_tetris_piece, create_texture_rect};

// initialize sdl context and canvas
fn main() {
    let (sdl_ctx, mut canvas) = create_window();

    let texture_creator = canvas.texture_creator();

    macro_rules! texture {
        ($color: expr) => {
            create_texture_rect(&mut canvas, &texture_creator, $color)
                .expect("Failed to create texture");
        };
    }

    // generate all the textures needed later on at once
    let texture_cache = [
        texture!(GameColor::Red),
        texture!(GameColor::Green),
        texture!(GameColor::Blue),
        texture!(GameColor::Yellow),
        texture!(GameColor::Cyan),
        texture!(GameColor::Orange),
        texture!(GameColor::Purple),
        texture!(GameColor::Gray),
    ];

    let mut game = Game::new();

    draw_tetris_piece(&mut canvas, &texture_cache, &game);
    canvas.present();

    start_render_loop(&sdl_ctx, &mut canvas, &texture_cache, &mut game)
}


fn start_render_loop(
    sdl_context: &Sdl,
    canvas: &mut Canvas<Window>,
    textures: &[Texture; 8],
    game: &mut Game,
) {
    use self::Event::{KeyDown, Quit};
    use self::Keycode::*;

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get sdl context event pump");

    // loop till we receive exit signal QUIT/ESCAPE key
    'main: loop {
        let mut p = game.current_piece.unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Quit { .. } | KeyDown { keycode: Some(Escape), .. } => {
                        break 'main;
                    }
                    KeyDown { keycode: Some(Up), .. } => {
                        p.x -= 1;
                    }
                    KeyDown { keycode: Some(Down), .. } => {
                        p.x += 1;
                    }
                    KeyDown { keycode: Some(Left), .. } => {
                        p.y -= 1;
                    }
                    KeyDown { keycode: Some(Right), .. } => {
                        p.y += 1;
                    }
                    _ => {}
                }

                // TODO refactor this clamping to remove the ugly casting
                if p.x < 0 { p.x = 0 };
                if p.x > WIDTH as isize - 4 * TEXTURE_SIZE as isize { p.x = WIDTH as isize - 4 * TEXTURE_SIZE as isize };
                if p.y < 0 { p.y = 0 };
                if p.y > HEIGHT as isize { p.y = HEIGHT as isize };


                game.current_piece = Some(p);

                // set canvas background and clear it
                canvas.set_draw_color(GameColor::Blue);
                canvas.clear();

                draw_tetris_piece(canvas, &textures, &game);
                canvas.present();
            }

        sleep(Duration::new(0, 1_000_000_000u32 / 60)); // for 60 fps TODO: use better time sync
    }
}
