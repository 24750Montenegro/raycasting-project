use crate::config;
use crate::maze::{solid_at, Maze, OUT_OF_BOUNDS};
use crate::player::Player;
use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Vertical,   // cara perpendicular al eje X
    Horizontal, // cara perpendicular al eje Y
}

pub struct Intersect {
    /// Distancia euclidiana desde el jugador, en píxeles del mundo.
    pub distance: f32,
    pub impact: char,
    pub side: Side,
    pub hit: Vector2,
}

pub fn cast_ray(maze: &Maze, player: &Player, angle: f32, block_size: usize) -> Intersect {
    let direction = Vector2::new(angle.cos(), angle.sin());
    let block = block_size as f32;
    let step = config::RAY_STEP * block;
    let max_distance = config::RAY_MAX_DEPTH * block;

    let mut prev_i = player.pos.x.max(0.0) as usize / block_size;
    let mut prev_j = player.pos.y.max(0.0) as usize / block_size;
    let mut distance = 0.0f32;

    while distance < max_distance {
        distance += step;
        let point = player.pos + direction * distance;

        if let Some(cell) = solid_at(maze, point, block_size) {
            let i = point.x.max(0.0) as usize / block_size;
            let j = point.y.max(0.0) as usize / block_size;
            let side = pick_side(i != prev_i, j != prev_j, point / block, direction);

            return Intersect {
                distance: distance.max(f32::EPSILON),
                impact: cell,
                side,
                hit: point,
            };
        }

        prev_i = point.x.max(0.0) as usize / block_size;
        prev_j = point.y.max(0.0) as usize / block_size;
    }

    Intersect {
        distance: max_distance,
        impact: OUT_OF_BOUNDS,
        side: Side::Vertical,
        hit: player.pos + direction * max_distance,
    }
}

fn pick_side(crossed_i: bool, crossed_j: bool, cell_pos: Vector2, direction: Vector2) -> Side {
    match (crossed_i, crossed_j) {
        (true, false) => Side::Vertical,
        (false, true) => Side::Horizontal,
        _ => {
            // qué tan adentro de la celda vamos en cada eje (0.0 .. 1.0)
            let fx = cell_pos.x.rem_euclid(1.0);
            let fy = cell_pos.y.rem_euclid(1.0);
            let past_x = if direction.x > 0.0 { fx } else { 1.0 - fx };
            let past_y = if direction.y > 0.0 { fy } else { 1.0 - fy };

            if past_x < past_y {
                Side::Vertical
            } else {
                Side::Horizontal
            }
        }
    }
}
