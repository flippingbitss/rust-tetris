use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

use crate::constants::*;
use crate::others::{Presence};
use crate::game_color::GameColor;
use crate::game::Game;
use crate::piece::Piece;


pub fn draw_tetris_piece(canvas: &mut Canvas<Window>, textures: &[Texture; 8], piece: &Piece) {
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
                //println!("({},{}) ({},{})", x, y, col, row);
                canvas.copy(
                    &border_tex,
                    None,
                    Rect::new(x_offset, y_offset, TEXTURE_SIZE, TEXTURE_SIZE),
                ).unwrap();

                canvas.copy(
                    &block_tex,
                    None,
                    Rect::new(
                        x_offset + BORDER_WIDTH as i32,
                        y_offset + BORDER_WIDTH as i32,
                        TEXTURE_SIZE_INNER,
                        TEXTURE_SIZE_INNER,
                    ),
                ).unwrap();
            }
        }
    }
}

pub fn create_texture_rect<'a>(
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

pub fn create_window() -> (Sdl, Canvas<Window>) {
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

    (sdl_context, canvas)
}