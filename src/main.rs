#[macro_use]
extern crate failure;
extern crate piston_window;

use piston_window::*;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;

mod data;

use data::*;

enum ReadingState {
    Room,
    Link,
    Moves,
}

enum ParserData {
    Room(Room),
    Link(Link),
    Moves(Vec<AntMove>),
}

fn parsing(tx: mpsc::Sender<ParserData>) {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut state = ReadingState::Room;
    loop {
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(s) => {
                if s == 0 {
                    break;
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
        if line.starts_with("##") {
            stdin.read_line(&mut line).unwrap();
        }
        if line.is_empty() {
            state = ReadingState::Moves;
            continue;
        }
        match state {
            ReadingState::Room => match Room::parse(&line) {
                Ok(r) => tx.send(ParserData::Room(r)).unwrap(),
                Err(_) => {
                    state = ReadingState::Link;
                    let link = Link::from_str(&line).expect("erf");
                    tx.send(ParserData::Link(link)).unwrap();
                }
            },
            ReadingState::Link => match Link::from_str(&line) {
                Ok(link) => tx.send(ParserData::Link(link)).unwrap(),
                Err(_) => {
                    state = ReadingState::Moves;
                }
            },
            ReadingState::Moves => {
                let mut turn = Vec::new();
                for mov in line.split_whitespace() {
                    match AntMove::parse(&mov) {
                        Ok(m) => turn.push(m),
                        Err(_) => {}
                    }
                }
                tx.send(ParserData::Moves(turn)).unwrap();
            }
        }
    }
}

fn ui_thread(mut map: Map, mut moves: AntMoves, rx: mpsc::Receiver<ParserData>) {
    let mut window: PistonWindow = WindowSettings::new("Lem-in Visualiser", (600, 400))
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(e) = window.next() {
        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0., 0., 0., 1.], g);
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
        while let Ok(data) = rx.try_recv() {
            match data {
                ParserData::Room(r) => map.add_room(r),
                ParserData::Link(l) => map.add_link(l),
                ParserData::Moves(m) => moves.push(m),
            }
        }
    }
}

fn main() {
    let map = {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let mut line = String::new();
        stdin
            .read_line(&mut line)
            .expect("unable to read number of ants");
        let ants = line.trim().parse().expect("expecting a number"); // TODO: could be error instead
        Map::new(ants)
    };
    let moves = Vec::new();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        parsing(tx);
    });
    ui_thread(map, moves, rx);
}
