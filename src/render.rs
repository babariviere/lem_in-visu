use piston_window::*;

pub const ROOM_SIZE: i64 = 20;
pub const ROOM_GAP: i64 = 10;
pub const TILE_SIZE: i64 = ROOM_SIZE + (ROOM_GAP * 2);
pub const ANT_COLOR: [f32; 4] = [0., 0., 1., 1.];
pub const ROOM_COLOR: [f32; 4] = [1., 1., 1., 1.];

pub trait Render {
    fn render(&self, c: context::Context, g: &mut G2d);
}
