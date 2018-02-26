use data::*;
use piston_window::*;

const ROOM_SIZE: f64 = 20.;
const ROOM_GAP: f64 = 5.;

fn calc_way(map: &Map, link: &Link) -> Vec<(f64, f64)> {
    let mut res = Vec::new();
    let r1 = map.get_room(&link.room1).unwrap();
    let (x1, y1) = r1.pos();
    let mut x1 = x1 as f64;
    let mut y1 = y1 as f64;
    x1 *= ROOM_SIZE + ROOM_GAP * 2.;
    y1 *= ROOM_SIZE + ROOM_GAP * 2.;
    let r2 = map.get_room(&link.room2).unwrap();
    let (x2, y2) = r2.pos();
    let mut x2 = x2 as f64;
    let mut y2 = y2 as f64;
    x2 *= ROOM_SIZE + ROOM_GAP * 2.;
    y2 *= ROOM_SIZE + ROOM_GAP * 2.;
    res.push((x1 as f64, y1 as f64));
    res.push((x2 as f64, y1 as f64));
    res.push((x2 as f64, y2 as f64));
    res
}

impl Room {
    pub fn render(&self, map: &Map, c: context::Context, g: &mut G2d) {
        let (x, y) = self.pos();
        let rect = [
            x as f64 * (ROOM_SIZE + ROOM_GAP * 2.) + ROOM_GAP,
            y as f64 * (ROOM_SIZE + ROOM_GAP * 2.) + ROOM_GAP,
            ROOM_SIZE,
            ROOM_SIZE,
        ];
        rectangle([1., 1., 1., 1.], rect, c.transform, g);
        for link in self.links() {
            let ways = calc_way(map, link);
            for i in 0..ways.len() - 1 {
                line(
                    [1., 0., 0., 1.],
                    1.,
                    [ways[i].0, ways[i].1, ways[i + 1].0, ways[i + 1].1],
                    c.transform,
                    g,
                );
            }
            calc_way(map, link).iter().fold((), |a, b| {});
        }
    }
}
