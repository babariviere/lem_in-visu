#[macro_use]
extern crate failure;
extern crate piston_window;

use piston_window::*;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;

mod data;
mod parser;

use data::*;
use parser::*;

fn ui_thread(mut map: Map, mut moves: AntMoves) {
    let mut window: PistonWindow = WindowSettings::new("Lem-in Visualiser", (600, 400))
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(e) = window.next() {
        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0., 0., 0., 1.], g);
                // TODO: map layout to avoid overlap
                // TODO: fn to do and undo action of each move
                for room in map.rooms().values() {
                    let (x, y) = room.pos();
                    let rect = [(x * 30) as f64, (y * 30) as f64, 25., 25.];
                    rectangle([1., 1., 1., 1.], rect, c.transform, g);
                    for link in room.links() {
                        let r2 = match map.get_room(&link.room2) {
                            Some(s) => s,
                            None => continue,
                        };
                        let l = [
                            (x * 30) as f64,
                            (y * 30) as f64,
                            (r2.x() * 30) as f64,
                            (r2.y() * 30) as f64,
                        ];
                        line([1., 1., 1., 1.], 1., l, c.transform, g);
                    }
                }
            });
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
