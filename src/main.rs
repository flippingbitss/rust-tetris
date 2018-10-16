extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use std::thread::sleep;
use std::time::Duration;

// constants
const TITLE: &'static str = "Tetris in Rust";
const WIDTH: u32 = 480;
const HEIGHT: u32 = 860;
const TEXTURE_SIZE: u32 = 32;

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

    draw_example_texture(&mut canvas);

    start_render_loop(&sdl_context)
}

// an example texture
fn draw_example_texture(canvas: &mut Canvas<Window>) {
    let texture_creator = canvas.texture_creator();

    let mut square_texture = texture_creator
        .create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
        .unwrap();

    // use the canvas to draw a texture
    canvas
        .with_texture_canvas(&mut square_texture, |texture| {
            texture.set_draw_color(Color::RGB(0, 255, 0));
            texture.clear();
        })
        .unwrap();

    // set canvas background and clear it
    canvas.set_draw_color(Color::RGB(255, 0, 0));
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
