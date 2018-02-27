use data::*;
use render::*;
use piston_window::*;
use std::collections::HashMap;

fn coord_to_center(coord: (usize, usize)) -> (f64, f64) {
    let mut x = coord.0 as f64;
    let mut y = coord.1 as f64;
    x *= ROOM_SIZE + ROOM_GAP * 2.;
    y *= ROOM_SIZE + ROOM_GAP * 2.;
    x += ROOM_GAP;
    y += ROOM_GAP;
    (x, y)
}

fn closest_coord(c1: (f64, f64), c2: (f64, f64)) -> (f64, f64) {
    let dx = c1.0 - c2.0;
    let dy = c1.1 - c2.1;
    if dx.abs() > dy.abs() {
        if dx < 0. {
            (c1.0 + ROOM_SIZE / 2. + ROOM_GAP / 2., c1.1)
        } else {
            (c1.0 - ROOM_SIZE / 2. - ROOM_GAP / 2., c1.1)
        }
    } else if dy < 0. {
        (c1.0, c1.1 + ROOM_SIZE / 2. + ROOM_GAP / 2.)
    } else {
        (c1.0, c1.1 - ROOM_SIZE / 2. - ROOM_GAP / 2.)
    }
}

pub struct Way {
    points: Vec<(f64, f64)>,
}

impl Way {
    pub fn build(map: &HashMap<String, Room>, link: &Link) -> Way {
        let mut points = Vec::new();
        let r1 = map.get(&link.room1).unwrap();
        let p1 = coord_to_center(r1.pos());
        let r2 = map.get(&link.room2).unwrap();
        let p2 = coord_to_center(r2.pos());
        points.push(p1);
        points.push(closest_coord(p1, p2));
        points.push(closest_coord(p2, p1));
        points.push(p2);
        Way { points }
    }
}

impl Render for Way {
    fn render(&self, c: context::Context, g: &mut G2d) {
        for i in 0..self.points.len() - 1 {
            line(
                WAY_COLOR,
                1.,
                [
                    self.points[i].0 + ROOM_SIZE / 2.,
                    self.points[i].1 + ROOM_SIZE / 2.,
                    self.points[i + 1].0 + ROOM_SIZE / 2.,
                    self.points[i + 1].1 + ROOM_SIZE / 2.,
                ],
                c.transform,
                g,
            );
        }
    }
}

pub struct Map {
    rooms: HashMap<String, Room>,
    ants: usize,
    ways: Vec<Way>,
}

impl Map {
    fn build_ways(&mut self) {
        for room in self.rooms.values() {
            for link in room.links() {
                self.ways.push(Way::build(
                    &self.rooms,
                    &Link::new(room.name().to_owned(), link.clone()),
                ));
            }
        }
    }

    pub fn new(rooms: HashMap<String, Room>, ants: usize) -> Map {
        let mut map = Map {
            rooms,
            ants,
            ways: Vec::new(),
        };
        map.build_ways();
        map
    }
}

impl Render for Map {
    fn render(&self, c: context::Context, g: &mut G2d) {
        for way in &self.ways {
            way.render(c, g);
        }
        for room in self.rooms.values() {
            room.render(c, g);
        }
    }
}
