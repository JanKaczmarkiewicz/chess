use std::time::Duration;

use crate::core::chessman::chessman::ChessmanKind;
use crate::core::state::{PossibleMoveKind, Side, State};
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;

const TITLE: &str = "App";
const HEIGHT: u32 = 800;
const WIDTH: u32 = 800;

const BOARD_SIZE: usize = 8;

pub struct Renderer {
    canvas: Canvas<Window>,
    event_pump: EventPump,
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

        let event_pump = sdl_context.event_pump()?;

        return Ok(Self {
            texture_creator,
            canvas,
            event_pump,
        });
    }

    pub fn update(&mut self, state: &State) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(19, 16, 17));
        self.canvas.clear();

        let height = self.canvas.viewport().height();
        let width = self.canvas.viewport().width();

        let min_dimention = height.min(width);

        let start_width = (width - min_dimention) / 2;
        let start_height = (height - min_dimention) / 2;

        let tile_size = min_dimention / BOARD_SIZE as u32;

        for (y, row) in state.get_board().iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let white = Color::RGB(235, 236, 208);
                let black = Color::RGB(119, 149, 86);

                let color = if (y + x) % 2 == 0 { white } else { black };

                let x_start = start_width + (tile_size * x as u32);
                let y_start = start_height + (tile_size * y as u32);

                let rect = Rect::new(x_start as i32, y_start as i32, tile_size, tile_size);

                self.canvas.set_draw_color(color);
                self.canvas.fill_rect(rect)?;

                if let Some(selected_tile) = state.selected_tile {
                    if (x, y) == selected_tile {
                        self.canvas.set_draw_color(Color::RGB(0, 236, 208));
                        self.canvas.draw_rect(rect)?;
                    }
                }

                if let Some(possible_move) =
                    state.possible_moves.iter().find(|m| m.coordinate == (x, y))
                {
                    if possible_move.kind == PossibleMoveKind::Move {
                        self.canvas.set_draw_color(Color::RGB(0, 0, 208));
                        self.canvas.draw_rect(rect)?;
                    }

                    if possible_move.kind == PossibleMoveKind::Capture {
                        self.canvas.set_draw_color(Color::RGB(0, 236, 0));
                        self.canvas.draw_rect(rect)?;
                    }
                }

                if let Some(chessman) = tile {
                    let side_part = match chessman.get_side() {
                        Side::White => "w",
                        Side::Black => "b",
                    };

                    let chessman_part = match chessman.get_kind() {
                        ChessmanKind::Bishop => "b",
                        ChessmanKind::Queen => "q",
                        ChessmanKind::King => "k",
                        ChessmanKind::Knight => "n",
                        ChessmanKind::Rook => "r",
                        ChessmanKind::Pawn => "p",
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

    pub fn get_next_input(&mut self) -> Option<(i32, i32)> {
        loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        return None;
                    }
                    Event::MouseButtonDown { x, y, .. } => {
                        let height = self.canvas.viewport().height() as i32;
                        let width = self.canvas.viewport().width() as i32;

                        let min_dimention = height.min(width);

                        let tile_size = min_dimention / BOARD_SIZE as i32;

                        let start_width = (width - min_dimention) / 2;
                        let start_height = (height - min_dimention) / 2;

                        let normalized_x = x - start_width;
                        let normalized_y = y - start_height;

                        if normalized_x > 0
                            && normalized_x < width
                            && normalized_y > 0
                            && normalized_y < height
                        {
                            return Some((
                                (normalized_x as f32 / tile_size as f32).floor() as i32,
                                (normalized_y as f32 / tile_size as f32).floor() as i32,
                            ));
                        }
                    }
                    _ => {}
                }
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
