#[macro_use]
extern crate failure;
extern crate piston_window;

use std::io::{self, BufRead};

mod data;

use data::*;

enum ReadingState {
    Room,
    Link,
    Moves,
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();
    stdin
        .read_line(&mut line)
        .expect("unable to read number of ants");
    let ants = line.trim().parse().expect("expecting a number"); // TODO: could be error instead
    let mut map = Map::new(ants);
    let mut state = ReadingState::Room;
    // TODO: do parsing in another thread
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        if line.starts_with("##") {
            stdin.read_line(&mut line).unwrap();
        }
        match state {
            ReadingState::Room => match Room::parse_room(&line) {
                Ok(r) => map.add_room(r),
                Err(_) => {
                    // TODO: add link
                    state = ReadingState::Link;
                }
            },
            ReadingState::Link => {
                // TODO: add link
            }
            ReadingState::Moves => {
                // TODO: store result and display
            }
        }
        // TODO:
    }
    // TODO: when exiting loop, we have to handle display
}
