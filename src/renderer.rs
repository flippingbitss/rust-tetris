use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

use crate::constants::*;
use crate::others::{Presence};
use crate::game_color::GameColor;
use crate::game::{Game,GameMap};
use crate::piece::Piece;


pub fn draw_tetris_piece(canvas: &mut Canvas<Window>, textures: &[Texture; 9], piece: &Piece) {
    let mat = piece.get_block_matrix(piece.current_state);

    let block_tex = &textures[piece.color as usize];
    let border_tex = &textures[GameColor::Gray as usize];

    let (min_x, max_x, min_y, max_y) = piece.get_filled_region(mat);

    let x = piece.x;
    let y = piece.y;

    for row in 0..4{
        for col in 0..4 {
                let y_offset = ((y + row as isize) * TEXTURE_SIZE as isize) as i32;
                let x_offset = ((x + col as isize) * TEXTURE_SIZE as isize) as i32;

                if mat[row][col] != Presence::No {
                    // draw filled region
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

                } else {
                    // debug bounds ----- TODO: remove this debug block
                    // draw empty cells
                    canvas.copy(
                        &block_tex,
                        None,
                        Rect::new(x_offset, y_offset, TEXTURE_SIZE, TEXTURE_SIZE),
                    ).unwrap();

                    canvas.copy(
                        &border_tex,
                        None,
                        Rect::new(x_offset + BORDER_WIDTH as i32, y_offset + BORDER_WIDTH as i32, TEXTURE_SIZE_INNER, TEXTURE_SIZE_INNER),
                    ).unwrap();
                }

//            }
        }
    }

    // debug bounds ------- TODO: remove these debug bounds
   let min_x = (x as i32 + min_x as i32) * TEXTURE_SIZE as i32;
   let min_y = (y as i32 + min_y as i32) * TEXTURE_SIZE as i32;
   let max_x = (x as i32 + max_x as i32) * TEXTURE_SIZE as i32;
   let max_y = (y as i32 + max_y as i32) * TEXTURE_SIZE as i32;

    canvas.set_draw_color(GameColor::Pink);
    let mut points = vec![
        Point::new(min_x, min_y),
        Point::new(max_x, min_y),
        Point::new(max_x, max_y),
        Point::new(min_x, max_y),
        Point::new(min_x, min_y),
    ];
    canvas.draw_lines(points.as_slice());
    // -----
}


pub fn draw_map(canvas: &mut Canvas<Window>, textures: &[Texture; 9], game_map: &GameMap) {
    let border_tex = &textures[GameColor::Gray as usize];

    for row in 0..NUM_BLOCKS_Y {
        for col in 0..NUM_BLOCKS_X  {
            if let Presence::Yes(color) = game_map[row][col] {
                let block_tex = &textures[color as usize];
                let x_offset = col as i32 * TEXTURE_SIZE as i32;
                let y_offset = row as i32 * TEXTURE_SIZE as i32;

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