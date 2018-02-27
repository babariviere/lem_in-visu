use failure::Error;
use piston_window::*;
use std::str::FromStr;
use render::*;

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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Room {
    name: String,
    kind: RoomKind,
    pos: (usize, usize),
    full: bool,
    links: Vec<String>,
}

impl Room {
    pub fn new(name: String, kind: RoomKind, pos: (usize, usize)) -> Room {
        Room {
            name,
            kind,
            pos,
            full: false,
            links: Vec::new(),
        }
    }

    pub fn parse(s: &str) -> Result<Self, Error> {
        let mut room;
        let kind;
        let mut split = s.split('\n');
        match split.next() {
            Some(s) if s.starts_with("##") => {
                kind = RoomKind::from_str(s)?;
            }
            Some(s) => {
                return Ok(Room::from_str(s)?);
            }
            _ => bail!(RoomParseError::InvalidRoom),
        }
        room = Self::from_str(split.next().ok_or(RoomParseError::InvalidRoom)?)?;
        room.kind = kind;
        Ok(room)
    }

    pub fn add_link(&mut self, link: String) {
        self.links.push(link);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }

    pub fn links(&self) -> &Vec<String> {
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

impl Render for Room {
    fn render(&self, c: context::Context, g: &mut G2d) {
        let (x, y) = self.pos();
        let x = x as f64 * (ROOM_SIZE + ROOM_GAP * 2.) + ROOM_GAP;
        let y = y as f64 * (ROOM_SIZE + ROOM_GAP * 2.) + ROOM_GAP;
        let rect = [x, y, ROOM_SIZE, ROOM_SIZE];
        rectangle(ROOM_COLOR, rect, c.transform, g);
    }
}
