#[macro_use]
extern crate failure;
extern crate piston_window;

use piston_window::*;
use std::io::{self, BufRead};
use std::time::{Duration, Instant};

mod data;
mod map;
mod parser;
mod render;

use data::*;
use map::Map;
use parser::*;
use render::*;

struct ViewSettings {
    pub x: f64,
    pub y: f64,
    pub scale: f64,
    pub mouse_move: bool,
    pub mouse_scroll: bool,
}

impl ViewSettings {
    pub fn reset(&mut self) {
        self.x = 0.;
        self.y = 0.;
        self.scale = 1.;
        self.mouse_move = false;
        self.mouse_scroll = false;
    }
}

impl Default for ViewSettings {
    fn default() -> Self {
        ViewSettings {
            x: 0.,
            y: 0.,
            scale: 1.,
            mouse_move: false,
            mouse_scroll: false,
        }
    }
}

fn ui_thread(map: MapData, moves: &[Vec<AntMove>]) {
    let mut window: PistonWindow = WindowSettings::new("Lem-in Visualiser", (600, 400))
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut settings = ViewSettings::default();
    let mut instant = Instant::now();
    let three_secs = Duration::from_secs(3);
    let mut moves_idx = 0;
    let mut map: Map = map.into();
    while let Some(e) = window.next() {
        if settings.scale < 0. {
            settings.scale = 0.01;
        }
        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |mut c, g| {
                clear([0., 0., 0., 1.], g);
                // TODO: map layout to avoid overlap
                // TODO: fn to do and undo action of each move
                c.transform = c.transform
                    .trans(settings.x * settings.scale, settings.y * settings.scale)
                    .zoom(settings.scale);
                map.render(c, g);
            });
        }
        if let Some(k) = e.press_args() {
            match k {
                Button::Keyboard(Key::Left) | Button::Keyboard(Key::A) => {
                    settings.x += 1.;
                }
                Button::Keyboard(Key::Right) | Button::Keyboard(Key::D) => {
                    settings.x -= 1.;
                }
                Button::Keyboard(Key::Up) | Button::Keyboard(Key::W) => {
                    settings.y += 1.;
                }
                Button::Keyboard(Key::Down) | Button::Keyboard(Key::S) => {
                    settings.y -= 1.;
                }
                Button::Keyboard(Key::Plus) | Button::Keyboard(Key::Equals) => {
                    settings.scale += 0.2;
                }
                Button::Keyboard(Key::Minus) => {
                    settings.scale -= 0.2;
                }
                Button::Keyboard(Key::Z) => settings.mouse_scroll = !settings.mouse_scroll,
                Button::Keyboard(Key::R) => {
                    settings.reset();
                }
                Button::Mouse(MouseButton::Left) => {
                    settings.mouse_move = true;
                    settings.mouse_scroll = false;
                }
                _e => {
                    //println!("{:?}", e);
                }
            }
        }
        if let Some(r) = e.release_args() {
            match r {
                Button::Mouse(MouseButton::Left) => settings.mouse_move = false,
                _ => {}
            }
        }
        if let Some(false) = e.cursor_args() {
            settings.mouse_move = false;
        }
        e.mouse_relative(|dx, dy| {
            if settings.mouse_move {
                settings.x += dx / 10. * settings.scale;
                settings.y += dy / 10. * settings.scale;
            } else if settings.mouse_scroll {
                settings.scale += dy / 100.;
            }
        });
        e.mouse_scroll(|_dx, dy| settings.scale += dy / 100.);
        if instant.elapsed() > three_secs && moves_idx < moves.len() {
            for m in &moves[moves_idx] {
                println!("{:?}", m);
                map.apply_move(&m);
            }
            moves_idx += 1;
            instant = Instant::now();
        }
    }
}

fn main() {
    let mut map = {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let mut line = String::new();
        stdin
            .read_line(&mut line)
            .expect("unable to read number of ants");
        let ants = line.trim().parse().expect("expecting a number"); // TODO: could be error instead
        MapData::new(ants)
    };
    let mut moves = Vec::new();
    parse(&mut map, &mut moves);
    ui_thread(map, &moves);
}
