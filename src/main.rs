extern crate sdl2;

mod lib;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

use std::thread::sleep;
use std::time::Duration;

use self::lib::{GameColor, Piece, PieceType};

// constants
const TITLE: &'static str = "Tetris in Rust";

const NUM_BLOCKS_X: u32 = 10;
const NUM_BLOCKS_Y: u32 = 18;
const TEXTURE_SIZE: u32 = 32;
const WIDTH: u32 = NUM_BLOCKS_X * TEXTURE_SIZE; // 480;
const HEIGHT: u32 = NUM_BLOCKS_Y * TEXTURE_SIZE; //860;

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
    let square_texture =
        create_square_texture(&mut canvas, &texture_creator).expect("Failed to create texture");

    // set canvas background and clear it
    canvas.set_draw_color(Color::from(GameColor::Red));
    canvas.clear();

    // copy the texture onto the canvas and draw
    canvas
        .copy(
            &square_texture,
            None,
            Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE),
        )
        .unwrap();



    canvas.present();

    start_render_loop(&sdl_context)
}

fn create_square_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Option<Texture<'a>> {
    let result = texture_creator.create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE);

    if let Ok(mut square_texture) = result {
        canvas
            .with_texture_canvas(&mut square_texture, |texture| {
                texture.set_draw_color(Color::from(GameColor::Blue));
                texture.clear();
            })
            .expect("Failed texture drawing");

        Some(square_texture)
    } else {
        None
    }
}

fn start_render_loop(sdl_context: &Sdl) {
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
