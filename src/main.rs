extern crate sdl2;

mod constants;
mod others;
mod game_color;
mod piece;

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

use crate::others::{PieceType, Presence};
use crate::game_color::GameColor;
use crate::piece::Piece;
use crate::constants::*;

struct Game {
    current_piece: Option<Piece>,
    score: usize,
    lines_cleared: usize,
    game_map: Vec<Vec<Presence>>,
}

impl Game {
    fn new() -> Game {
        Game {
            current_piece: Some(Piece::from(random::<PieceType>())),
            score: 0,
            lines_cleared: 0,
            game_map: vec![vec![Presence::No; NUM_BLOCKS_X as usize]; NUM_BLOCKS_Y as usize],
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

    //    println!("{:?}", format!("{:b}",curr).split(""));
    //    println!("{:?}", mat);
    canvas.present();

    start_render_loop(&sdl_context, &mut canvas, &texture_cache, &mut game)
}

fn draw_tetris_piece(canvas: &mut Canvas<Window>, textures: &[Texture; 8], game: &Game) {
    println!("drawing piece");
    let piece = game.current_piece.unwrap();
    let mat = piece.get_block_matrix(piece.current_state);
    let block_tex = &textures[piece.color as usize];
    let border_tex = &textures[GameColor::Gray as usize];
    let x = piece.x as usize * TEXTURE_SIZE as usize;
    let y = piece.y as usize * TEXTURE_SIZE as usize;

    for row in 0..4 {
        for col in 0..4 {
            if mat[row][col] != Presence::No {
                let y_offset = (y + row * TEXTURE_SIZE as usize) as i32;
                let x_offset = (x + col * TEXTURE_SIZE as usize) as i32;

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
