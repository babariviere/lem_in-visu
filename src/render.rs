use piston_window::*;

pub const ROOM_SIZE: f64 = 20.;
pub const ROOM_GAP: f64 = 10.;
pub const ANT_COLOR: [f32; 4] = [0., 0., 1., 0.5];
pub const ROOM_COLOR: [f32; 4] = [1., 1., 1., 0.5];
pub const WAY_COLOR: [f32; 4] = [1., 0., 0., 0.2];

pub trait Render {
    fn render(&self, c: context::Context, g: &mut G2d);
}
