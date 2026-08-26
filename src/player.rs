use raylib::prelude:: *;
use std::f32::consts::PI;
use crate::maze::{is_wall, Maze};


pub struct Player {
    pub pos: Vector2,
    pub angle: f32,
    pub fov: f32,
}

const MOVE_SPEED: f32 = 200.0;
const ROTATION_SPEED: f32 = PI/1.5; // radianes por segundo
const MOUSE_SENSITIVITY: f32 = 0.002; // sensibilidad del ratón
const PLAYER_RADIUS: f32 = 10.0; // radio del jugador

pub fn process_events(
    player: &mut Player,
    rl: &RaylibHandle,
    maze: &Maze,
    block_size: usize,
){
    //tiempo del frame anterior
    let dt = rl.get_frame_time();


    // MOVIMIENTO

    let mut forward = 0.0f32;
    let mut strafe = 0.0f32;

    // movimiento a la izquierda
    if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
        strafe -= 1.0;
    }

    // movimiento a la derecha
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
        strafe += 1.0;
    }

    // movimiento hacia adelante
    if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
        forward += 1.0;
    }

    // movimiento hacia atrás
    if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
        forward -= 1.0;
    }

    // Angulo de la camara con el mouse

    let mouse_delta = rl.get_mouse_delta();
    player.angle += mouse_delta.x * MOUSE_SENSITIVITY;

    let two_pi = 2.0 * PI;
    player.angle = player.angle.rem_euclid(two_pi); // para evitar infinito crecimiento del angulo

    if forward == 0.0 && strafe == 0.0 {
        return; // no hay movimiento
    }

    let speed = MOVE_SPEED * dt;

    //dirección del movimiento
    let dx = player.angle.cos() * forward * speed - player.angle.sin() * strafe;
    let dy = player.angle.sin() * forward * speed + player.angle.cos() * strafe;

    try_move(player, maze, block_size, dx, dy);
}

fn try_move(player: &mut Player, maze: &Maze, block_size: usize, dx: f32, dy: f32) {
    let new_x = player.pos.x + dx;
    let new_y = player.pos.y + dy;

    if !collides(maze, new_x, player.pos.y, block_size) {
        player.pos.x = new_x;
    }
    if !collides(maze, player.pos.x, new_y, block_size) {
        player.pos.y = new_y;
    }
}

fn collides(maze: &Maze, x: f32, y: f32, block_size: usize) -> bool {
    let r = PLAYER_RADIUS;

    is_wall(maze, x-r, y-r, block_size) ||
    is_wall(maze, x+r, y-r, block_size) ||
    is_wall(maze, x-r, y+r, block_size) ||
    is_wall(maze, x+r, y+r, block_size)
}