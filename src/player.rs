use raylib::prelude:: *;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vector2,
    pub angle: f32,
    pub fov: f32,
}