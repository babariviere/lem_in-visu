use data::*;
use piston_window::*;

const ROOM_SIZE: f64 = 20.;
const ROOM_GAP: f64 = 10.;
const ROOM_COLOR: [f32; 4] = [1., 1., 1., 0.5];

type Way = Vec<(f64, f64)>;

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
            return (c1.0 + ROOM_SIZE / 2. + ROOM_GAP / 2., c1.1);
        } else {
            return (c1.0 - ROOM_SIZE / 2. - ROOM_GAP / 2., c1.1);
        }
    } else {
        if dy < 0. {
            return (c1.0, c1.1 + ROOM_SIZE / 2. + ROOM_GAP / 2.);
        } else {
            return (c1.0, c1.1 - ROOM_SIZE / 2. - ROOM_GAP / 2.);
        }
    }
}

fn calc_way(map: &Map, r1: &str, r2: &str) -> Way {
    let mut res = Vec::new();
    let r1 = map.get_room(&r1).unwrap();
    let p1 = coord_to_center(r1.pos());
    let r2 = map.get_room(&r2).unwrap();
    let p2 = coord_to_center(r2.pos());
    res.push(p1);
    res.push(closest_coord(p1, p2));
    res.push(closest_coord(p2, p1));
    res.push(p2);
    res
}

// TODO: create lane

fn render_way(way: Way, c: context::Context, g: &mut G2d) {
    for i in 0..way.len() - 1 {
        line(
            [1., 0., 0., 0.2],
            1.,
            [
                way[i].0 + ROOM_SIZE / 2.,
                way[i].1 + ROOM_SIZE / 2.,
                way[i + 1].0 + ROOM_SIZE / 2.,
                way[i + 1].1 + ROOM_SIZE / 2.,
            ],
            c.transform,
            g,
        );
    }
}

impl Room {
    pub fn render(&self, map: &Map, c: context::Context, g: &mut G2d) {
        let (x, y) = self.pos();
        let x = x as f64 * (ROOM_SIZE + ROOM_GAP * 2.) + ROOM_GAP;
        let y = y as f64 * (ROOM_SIZE + ROOM_GAP * 2.) + ROOM_GAP;
        let rect = [x, y, ROOM_SIZE, ROOM_SIZE];
        rectangle(ROOM_COLOR, rect, c.transform, g);
        for link in self.links() {
            let r2 = map.get_room(&link).unwrap();
            let way = calc_way(map, self.name(), r2.name());
            render_way(way, c, g);
        }
    }
}
//let (x2, y2) = r2.pos();
//let x2 = x2 as f64 * (ROOM_SIZE + ROOM_GAP * 2.) + ROOM_GAP;
//let y2 = y2 as f64 * (ROOM_SIZE + ROOM_GAP * 2.) + ROOM_GAP;
//line(
//    ROOM_COLOR,
//    1.,
//    [
//        x + ROOM_SIZE / 2.,
//        y + ROOM_SIZE / 2.,
//        x2 + ROOM_SIZE / 2.,
//        y2 + ROOM_SIZE / 2.,
//    ],
//    c.transform,
//    g,
//);
