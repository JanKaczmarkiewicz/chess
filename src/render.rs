use std::time::Duration;

use crate::core::state::{Chessman, Side, State};
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

const TITLE: &str = "App";
const HEIGHT: u32 = 800;
const WIDTH: u32 = 800;

const BOARD_SIZE: usize = 8;

pub struct Renderer {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
}

impl Renderer {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

        let window = video_subsystem
            .window(TITLE, WIDTH, HEIGHT)
            .resizable()
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let texture_creator = canvas.texture_creator();

        return Ok(Self {
            texture_creator,
            canvas,
            sdl_context,
        });
    }

    pub fn update(self: &mut Self, state: State) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(19, 16, 17));
        self.canvas.clear();

        let height = self.canvas.viewport().height();
        let width = self.canvas.viewport().width();

        let min_dimention = height.min(width);

        let start_width = (width - min_dimention) / 2;
        let start_height = (height - min_dimention) / 2;

        let tile_size = min_dimention / BOARD_SIZE as u32;

        for (i, row) in state.board.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                let white = Color::RGB(235, 236, 208);
                let black = Color::RGB(119, 149, 86);

                let color = if (i + j) % 2 == 0 { white } else { black };

                let x_start = start_width + (tile_size * j as u32);
                let y_start = start_height + (tile_size * i as u32);

                let rect = Rect::new(x_start as i32, y_start as i32, tile_size, tile_size);

                self.canvas.set_draw_color(color);
                self.canvas.fill_rect(rect)?;

                if let Some((chessman, side)) = tile {
                    let side_part = match side {
                        Side::White => "w",
                        Side::Black => "b",
                    };

                    let chessman_part = match chessman {
                        Chessman::Bishop => "b",
                        Chessman::Queen => "q",
                        Chessman::King => "k",
                        Chessman::Knight => "n",
                        Chessman::Rook => "r",
                        Chessman::Pawn => "p",
                    };

                    let filename = format!("{side_part}{chessman_part}");

                    let texture = self
                        .texture_creator
                        .load_texture(format!("./images/{}.png", filename))?;
                    self.canvas.copy(&texture, None, rect)?;
                }
            }
        }

        self.canvas.present();

        Ok(())
    }

    pub fn wait_for_input(self: &Self, _state: ()) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump()?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {}
                }
            }
            // Time management!
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}
