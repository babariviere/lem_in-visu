use data::*;
use render::*;
use piston_window::*;
use std::collections::HashMap;

fn coord_to_center(coord: (usize, usize)) -> (i64, i64) {
    let mut x = coord.0 as i64;
    let mut y = coord.1 as i64;
    x *= TILE_SIZE;
    y *= TILE_SIZE;
    x += ROOM_GAP;
    y += ROOM_GAP;
    (x, y)
}

fn closest_coord(c1: (i64, i64), c2: (i64, i64)) -> (i64, i64) {
    let dx = c2.0 - c1.0;
    let dy = c2.1 - c1.1;
    if dy.abs() > dx.abs() {
        if dx < 0 {
            (c1.0 - TILE_SIZE / 3, c1.1)
        } else {
            (c1.0 + TILE_SIZE / 3, c1.1)
        }
    } else if dy < 0 {
        (c1.0, c1.1 + TILE_SIZE / 3)
    } else {
        (c1.0, c1.1 - TILE_SIZE / 3)
    }
}

pub struct Way {
    points: Vec<(i64, i64)>,
}

impl Way {
    pub fn build(map: &HashMap<String, Room>, link: &Link) -> Way {
        let mut points = Vec::new();
        let r1 = map.get(&link.room1).unwrap();
        let mut c1 = coord_to_center(r1.pos());
        let r2 = map.get(&link.room2).unwrap();
        let c2 = coord_to_center(r2.pos());
        points.push(c1);
        c1 = closest_coord(c1, c2);
        points.push(c1);
        while c1.0 != c2.0 || c1.1 != c2.1 {
            c1.0 -= c1.0 % TILE_SIZE;
            c1.1 -= c1.1 % TILE_SIZE;
            let dx = c2.0 - c1.0;
            let dy = c2.1 - c1.1;
            println!("dx={} dy={}", dx, dy);
            println!(
                "rx={} ry={} s={}",
                (dx % TILE_SIZE),
                (dy % TILE_SIZE),
                TILE_SIZE
            );
            if dx.abs() >= TILE_SIZE {
                if dx > 0 {
                    c1.0 += TILE_SIZE;
                } else {
                    c1.0 -= TILE_SIZE;
                }
            } else {
                c1.0 += dx;
            }
            if dy.abs() >= TILE_SIZE {
                if dy > 0 {
                    c1.1 += TILE_SIZE;
                } else {
                    c1.1 -= TILE_SIZE;
                }
            } else {
                c1.1 += dy;
            }
            //c1.1 %= TILE_SIZE;
            // TODO: check collision
            println!("{:?} {:?}", c1, c2);
            //::std::thread::sleep_ms(300);
            if c1 != c2 {
                points.push(c1);
            }
        }
        c1 = closest_coord(c2, c1);
        points.push(c1);
        points.push(c2);
        Way { points }
    }
}

impl Render for Way {
    fn render(&self, c: context::Context, g: &mut G2d) {
        let round_line = line::Line::new(WAY_COLOR, 1.).shape(line::Shape::Bevel);
        for i in 0..self.points.len() - 1 {
            round_line.draw(
                [
                    self.points[i].0 as f64 + ROOM_SIZE as f64 / 2.,
                    self.points[i].1 as f64 + ROOM_SIZE as f64 / 2.,
                    self.points[i + 1].0 as f64 + ROOM_SIZE as f64 / 2.,
                    self.points[i + 1].1 as f64 + ROOM_SIZE as f64 / 2.,
                ],
                &c.draw_state,
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

    pub fn apply_move(&mut self, ant_move: &AntMove) {
        self.rooms.get_mut(&ant_move.room1).map(|r| r.set_empty());
        self.rooms.get_mut(&ant_move.room2).map(|r| r.set_full());
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
