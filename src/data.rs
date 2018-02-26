use failure::Error;
use std::collections::HashMap;
use std::str::FromStr;

pub type AntMoves = Vec<Vec<AntMove>>;

#[derive(Debug)]
pub struct AntMove {
    pub room1: String,
    pub room2: String,
}

impl AntMove {
    pub fn parse(s: &str) -> Result<AntMove, ()> {
        if s.chars().next() != Some('L') {
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

#[derive(Debug, Fail)]
pub enum RoomParseError {
    #[fail(display = "invalid room")]
    InvalidRoom,
    #[fail(display = "invalid room kind")]
    InvalidRoomKind,
    #[fail(display = "invalid room name")]
    InvalidRoomName,
    #[fail(display = "invalid coord")]
    InvalidCoord,
}

#[derive(Debug)]
pub struct Link {
    pub room1: String,
    pub room2: String,
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

#[derive(Debug)]
pub struct Room {
    name: String,
    kind: RoomKind,
    pos: (usize, usize),
    full: bool,
    links: Vec<Link>,
}

impl Room {
    pub fn new(name: String, kind: RoomKind, pos: (usize, usize)) -> Room {
        Room {
            name: name,
            kind: kind,
            pos: pos,
            full: false,
            links: Vec::new(),
        }
    }

    pub fn parse(s: &str) -> Result<Self, Error> {
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

    pub fn x(&self) -> usize {
        self.pos.0
    }

    pub fn y(&self) -> usize {
        self.pos.1
    }

    pub fn full(&self) -> bool {
        self.full
    }

    pub fn links(&self) -> &Vec<Link> {
        &self.links
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

#[derive(Debug)]
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

    pub fn add_link(&mut self, link: Link) {
        let mut room = self.rooms.get_mut(&link.room1);
        if let Some(ref mut r) = room {
            r.links.push(link);
        }
    }

    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }

    pub fn ants(&self) -> usize {
        self.ants
    }

    pub fn rooms<'a>(&self) -> &HashMap<String, Room> {
        &self.rooms
    }
}
