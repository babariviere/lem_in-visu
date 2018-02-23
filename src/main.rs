#[macro_use]
extern crate failure;
extern crate piston_window;

use std::io::{self, BufRead};
use std::str::FromStr;

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
    let mut moves = Vec::new();
    let mut state = ReadingState::Room;
    // TODO: do parsing in another thread
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
                Ok(r) => map.add_room(r),
                Err(_) => {
                    // TODO: add link
                    state = ReadingState::Link;
                    let link = Link::from_str(&line).expect("erf");
                    map.add_link(link);
                }
            },
            ReadingState::Link => {
                // TODO: add link
                match Link::from_str(&line) {
                    Ok(link) => map.add_link(link),
                    Err(_) => {
                        state = ReadingState::Moves;
                    }
                }
            }
            ReadingState::Moves => {
                // TODO: store result and display
                let mut turn = Vec::new();
                for mov in line.split_whitespace() {
                    match AntMove::parse(&mov) {
                        Ok(m) => turn.push(m),
                        Err(_) => {}
                    }
                }
                moves.push(turn);
            }
        }
        // TODO:
    }
    println!("{:#?}", map);
    println!("{:#?}", moves);
    // TODO: when exiting loop, we have to handle display
}
