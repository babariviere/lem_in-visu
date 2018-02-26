use data::*;
use std::io::{self, BufRead};
use std::str::FromStr;

enum ReadingState {
    Room,
    Link,
    Moves,
}

pub fn parse(map: &mut Map, moves: &mut AntMoves) {
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
                Ok(r) => map.add_room(r),
                Err(_) => {
                    state = ReadingState::Link;
                    let link = Link::from_str(&line).expect("erf");
                    map.add_link(link);
                }
            },
            ReadingState::Link => match Link::from_str(&line) {
                Ok(link) => map.add_link(link),
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
                moves.push(turn);
            }
        }
    }
}
