use map::Map;
use std::collections::HashMap;
use std::str::FromStr;

mod room;

pub use self::room::*;

pub type AntMoves = Vec<Vec<AntMove>>;

#[derive(Debug)]
pub struct AntMove {
    pub room1: String,
    pub room2: String,
}

impl AntMove {
    pub fn parse(s: &str) -> Result<AntMove, ()> {
        if !s.starts_with('L') {
            return Err(());
        }
        let splitted = s[1..]
            .trim()
            .split('-')
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        if splitted.len() != 2 {
            return Err(());
        }
        let mut splitted = splitted.into_iter();
        Ok(AntMove {
            room1: splitted.next().unwrap(),
            room2: splitted.next().unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct Link {
    pub room1: String,
    pub room2: String,
}

impl Link {
    pub fn new(room1: String, room2: String) -> Link {
        Link { room1, room2 }
    }
}

impl FromStr for Link {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.trim()
            .split('-')
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        if splitted.len() != 2 {
            return Err(());
        }
        let mut splitted = splitted.into_iter();
        Ok(Link {
            room1: splitted.next().unwrap(),
            room2: splitted.next().unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct MapData {
    rooms: HashMap<String, Room>,
    ants: usize,
}

impl MapData {
    pub fn new(ants: usize) -> MapData {
        MapData {
            rooms: HashMap::new(),
            ants,
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.name().to_owned(), room);
    }

    pub fn add_link(&mut self, link: &Link) {
        if let Some(ref mut r) = self.rooms.get_mut(&link.room1) {
            r.add_link(link.room2.clone());
        }
        //if let Some(ref mut r) = self.rooms.get_mut(&link.room2) {
        //    r.links.push(link.room1.clone());
        //}
    }
}

impl Into<Map> for MapData {
    fn into(self) -> Map {
        Map::new(self.rooms, self.ants)
    }
}
