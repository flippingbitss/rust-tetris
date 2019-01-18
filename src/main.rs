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
        texture!(GameColor::Pink),
    ];

    let mut game = Game::new();

    draw_tetris_piece(&mut canvas, &texture_cache, &game.current_piece.unwrap());
    canvas.present();

    start_render_loop(&sdl_ctx, &mut canvas, &texture_cache, &mut game)
}


fn start_render_loop(
    sdl_context: &Sdl,
    canvas: &mut Canvas<Window>,
    textures: &[Texture; 9],
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
        let mut dx = 0;
        let mut dy = 0;

        for event in event_pump.poll_iter() {
            match event {
                Quit { .. } | KeyDown { keycode: Some(Escape), .. } => { break 'main; }
                KeyDown { keycode: Some(Left), .. }  => { dx -= 1; }
                KeyDown { keycode: Some(Right), .. } => { dx += 1; }
                KeyDown { keycode: Some(Up), .. }    => { p.rotate(&game.game_map); }
                KeyDown { keycode: Some(Down), .. }  => { dy += 1; }
                KeyDown { keycode: Some(Space), .. }  => { p = Piece::from(random::<PieceType>()); }
                _ => {}
            }
        }

        p.move_position(&game.game_map, p.x + dx, p.y + dy); \
        game.current_piece = Some(p);

        // set canvas background and clear it
        canvas.set_draw_color(GameColor::Gray);
        canvas.clear();

        draw_tetris_piece(canvas, &textures, &p);
        canvas.present();


        sleep(Duration::new(0, 1_000_000_000u32 / 60)); // for 60 fps TODO: use better time sync
    }
}
