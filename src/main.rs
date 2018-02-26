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
mod render;

use data::*;
use parser::*;
use render::*;

fn ui_thread(map: Map, _moves: AntMoves) {
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
                    room.render(&map, c, g);
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
