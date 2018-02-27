#[macro_use]
extern crate failure;
extern crate piston_window;

use piston_window::*;
use std::io::{self, BufRead};

mod data;
mod map;
mod parser;
mod render;

use data::*;
use map::Map;
use parser::*;
use render::*;

fn ui_thread(map: MapData, _moves: &[Vec<AntMove>]) {
    let mut window: PistonWindow = WindowSettings::new("Lem-in Visualiser", (600, 400))
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut x = 0.;
    let mut y = 0.;
    let mut scale = 1.;
    let mut mouse_move = false;
    let mut mouse_scroll = false;
    let map: Map = map.into();
    while let Some(e) = window.next() {
        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |mut c, g| {
                clear([0., 0., 0., 1.], g);
                // TODO: map layout to avoid overlap
                // TODO: fn to do and undo action of each move
                c.transform = c.transform.trans(x * scale, y * scale).zoom(scale);
                map.render(c, g);
            });
        }
        if let Some(k) = e.press_args() {
            match k {
                Button::Keyboard(Key::Left) | Button::Keyboard(Key::A) => {
                    x += 1.;
                }
                Button::Keyboard(Key::Right) | Button::Keyboard(Key::D) => {
                    x -= 1.;
                }
                Button::Keyboard(Key::Up) | Button::Keyboard(Key::W) => {
                    y += 1.;
                }
                Button::Keyboard(Key::Down) | Button::Keyboard(Key::S) => {
                    y -= 1.;
                }
                Button::Keyboard(Key::Plus) | Button::Keyboard(Key::Equals) => {
                    scale += 0.2;
                }
                Button::Keyboard(Key::Minus) => {
                    if scale >= 0.3 {
                        scale -= 0.2;
                    }
                }
                Button::Keyboard(Key::Z) => mouse_scroll = !mouse_scroll,
                Button::Mouse(MouseButton::Left) => {
                    mouse_move = true;
                    mouse_scroll = false;
                }
                _e => {
                    //println!("{:?}", e);
                }
            }
        }
        if let Some(r) = e.release_args() {
            match r {
                Button::Mouse(MouseButton::Left) => mouse_move = false,
                _ => {}
            }
        }
        e.mouse_relative(|dx, dy| {
            if mouse_move {
                x += dx / 10. * scale;
                y += dy / 10. * scale;
            } else if mouse_scroll {
                scale += dy / 100.;
            }
        });
        e.mouse_scroll(|_dx, dy| scale += dy / 100.);
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
