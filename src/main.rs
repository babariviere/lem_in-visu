#[macro_use]
extern crate failure;
extern crate piston_window;

use piston_window::*;
use std::io::{self, BufRead};

mod data;
mod parser;
mod render;

use data::*;
use parser::*;
use render::*;

fn ui_thread(map: Map, _moves: AntMoves) {
    let mut window: PistonWindow = WindowSettings::new("Lem-in Visualiser", (600, 400))
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut x = 0.;
    let mut y = 0.;
    let mut scale = 1.;
    while let Some(e) = window.next() {
        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |mut c, g| {
                clear([0., 0., 0., 1.], g);
                // TODO: map layout to avoid overlap
                // TODO: fn to do and undo action of each move
                c.transform = c.transform.trans(x * scale, y * scale).zoom(scale);
                for room in map.rooms().values() {
                    room.render(&map, c, g);
                }
            });
        }
        if let Some(k) = e.press_args() {
            match k {
                Button::Keyboard(keyboard::Key::Left) => {
                    x += 1.;
                }
                Button::Keyboard(keyboard::Key::Right) => {
                    x -= 1.;
                }
                Button::Keyboard(keyboard::Key::Up) => {
                    y += 1.;
                }
                Button::Keyboard(keyboard::Key::Down) => {
                    y -= 1.;
                }
                Button::Keyboard(keyboard::Key::Plus) | Button::Keyboard(keyboard::Key::Equals) => {
                    scale += 0.1;
                }
                Button::Keyboard(keyboard::Key::Minus) => {
                    scale -= 0.1;
                }
                _e => {
                    //println!("{:?}", e);
                }
            }
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
        Map::new(ants)
    };
    let mut moves = Vec::new();
    parse(&mut map, &mut moves);
    ui_thread(map, moves);
}
