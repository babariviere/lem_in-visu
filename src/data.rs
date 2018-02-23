use failure::Error;
use std::collections::HashMap;
use std::str::FromStr;

pub struct AntMove {
    room1: String,
    room2: String,
}

impl AntMove {}

#[derive(Debug, Fail)]
pub enum RoomParseError {
    #[fail(display = "invalid room")] InvalidRoom,
    #[fail(display = "invalid room kind")] InvalidRoomKind,
    #[fail(display = "invalid room name")] InvalidRoomName,
    #[fail(display = "invalid coord")] InvalidCoord,
}

pub struct Link {
    room1: String,
    room2: String,
}

pub enum RoomKind {
    Start,
    End,
    Normal,
}

impl FromStr for RoomKind {
    type Err = RoomParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "##start" => Ok(RoomKind::Start),
            "##end" => Ok(RoomKind::End),
            _ => Err(RoomParseError::InvalidRoomKind),
        }
    }
}

pub struct Room {
    name: String,
    kind: RoomKind,
    pos: (usize, usize),
    full: bool,
}

impl Room {
    pub fn new(name: String, kind: RoomKind, pos: (usize, usize)) -> Room {
        Room {
            name: name,
            kind: kind,
            pos: pos,
            full: false,
        }
    }

    pub fn parse_room(s: &str) -> Result<Self, Error> {
        let mut room;
        let kind;
        let mut split = s.split('\n');
        match split.next() {
            Some(ref s) if s.starts_with("##") => {
                kind = RoomKind::from_str(s)?;
            }
            Some(ref s) => {
                return Ok(Room::from_str(s)?);
            }
            _ => bail!(RoomParseError::InvalidRoom),
        }
        room = Self::from_str(split.next().ok_or(RoomParseError::InvalidRoom)?)?;
        room.kind = kind;
        Ok(room)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &RoomKind {
        &self.kind
    }

    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }

    pub fn full(&self) -> bool {
        self.full
    }
}

impl FromStr for Room {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();
        let name = splitted.next().ok_or(RoomParseError::InvalidRoomName)?;
        let x = splitted
            .next()
            .ok_or(RoomParseError::InvalidCoord)?
            .parse()
            .map_err(|_| RoomParseError::InvalidCoord)?;
        let y = splitted
            .next()
            .ok_or(RoomParseError::InvalidCoord)?
            .parse()
            .map_err(|_| RoomParseError::InvalidCoord)?;
        Ok(Room::new(name.to_string(), RoomKind::Normal, (x, y)))
    }
}

pub struct Map {
    rooms: HashMap<String, Room>,
    ants: usize,
}

impl Map {
    pub fn new(ants: usize) -> Map {
        Map {
            rooms: HashMap::new(),
            ants: ants,
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.name.clone(), room);
    }

    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }

    pub fn ants(&self) -> usize {
        self.ants
    }
}
