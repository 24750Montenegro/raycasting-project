use crate::config;
use crate::maze::{find_player_start, is_wall, Maze};
use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vector2,
    pub angle: f32,
    pub fov: f32,
}

impl Player {
    pub fn spawn(maze: &Maze, block_size: usize) -> Self {
        Player {
            pos: find_player_start(maze, block_size),
            angle: PI / 4.0,
            fov: config::FOV,
        }
    }

    /// Vector unitario hacia donde mira el jugador.
    pub fn direction(&self) -> Vector2 {
        Vector2::new(self.angle.cos(), self.angle.sin())
    }

    pub fn update(&mut self, rl: &RaylibHandle, maze: &Maze, block_size: usize) {
        let dt = rl.get_frame_time();

        self.rotate(rl, dt);
        self.walk(rl, maze, block_size, dt);
    }

    fn rotate(&mut self, rl: &RaylibHandle, dt: f32) {
        let mut turn = rl.get_mouse_delta().x * config::MOUSE_SENSITIVITY;

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            turn -= config::ROTATION_SPEED * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            turn += config::ROTATION_SPEED * dt;
        }

        // rem_euclid evita que el ángulo crezca sin límite
        self.angle = (self.angle + turn).rem_euclid(2.0 * PI);
    }

    fn walk(&mut self, rl: &RaylibHandle, maze: &Maze, block_size: usize, dt: f32) {
        let mut forward = 0.0f32;
        let mut strafe = 0.0f32;

        if rl.is_key_down(KeyboardKey::KEY_W) {
            forward += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            forward -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            strafe -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            strafe += 1.0;
        }

        if forward == 0.0 && strafe == 0.0 {
            return;
        }

        let speed = config::MOVE_SPEED * dt;
        let (sin, cos) = self.angle.sin_cos();
        let dx = (cos * forward - sin * strafe) * speed;
        let dy = (sin * forward + cos * strafe) * speed;

        // ejes por separado para poder deslizarse a lo largo de una pared
        if !self.collides(maze, self.pos.x + dx, self.pos.y, block_size) {
            self.pos.x += dx;
        }
        if !self.collides(maze, self.pos.x, self.pos.y + dy, block_size) {
            self.pos.y += dy;
        }
    }

    fn collides(&self, maze: &Maze, x: f32, y: f32, block_size: usize) -> bool {
        let r = config::PLAYER_RADIUS;

        is_wall(maze, x - r, y - r, block_size)
            || is_wall(maze, x + r, y - r, block_size)
            || is_wall(maze, x - r, y + r, block_size)
            || is_wall(maze, x + r, y + r, block_size)
    }
}
