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
    /// Coordenada horizontal del impacto sobre la cara, en el rango [0, 1).
    pub tex_u: f32,
}

pub fn cast_ray(maze: &Maze, player: &Player, angle: f32, block_size: usize) -> Intersect {
    let direction = Vector2::new(angle.cos(), angle.sin());
    let block = block_size as f32;
    let step = config::RAY_STEP * block;
    let max_distance = config::RAY_MAX_DEPTH * block;

    let mut empty = 0.0f32;
    while empty < max_distance {
        let solid = empty + step;

        if solid_at(maze, player.pos + direction * solid, block_size).is_some() {
            return refine(maze, player.pos, direction, empty, solid, block_size);
        }
        empty = solid;
    }

    Intersect {
        distance: max_distance,
        impact: OUT_OF_BOUNDS,
        side: Side::Vertical,
        hit: player.pos + direction * max_distance,
        tex_u: 0.0,
    }
}

/// Biseca entre el último punto vacío y el primero sólido. Sin esto la distancia
/// queda cuantizada al paso del ray march y las paredes tiemblan al moverse.
fn refine(
    maze: &Maze,
    origin: Vector2,
    direction: Vector2,
    mut empty: f32,
    mut solid: f32,
    block_size: usize,
) -> Intersect {
    for _ in 0..config::RAY_REFINE_STEPS {
        let middle = 0.5 * (empty + solid);

        if solid_at(maze, origin + direction * middle, block_size).is_some() {
            solid = middle;
        } else {
            empty = middle;
        }
    }

    let hit = origin + direction * solid;
    let (side, tex_u) = face_at(hit / block_size as f32, direction);

    Intersect {
        distance: solid.max(f32::EPSILON),
        impact: solid_at(maze, hit, block_size).unwrap_or(OUT_OF_BOUNDS),
        side,
        hit,
        tex_u,
    }
}

/// Cara por la que el rayo entró a la celda y coordenada horizontal sobre ella.
/// `cell_pos` viene en unidades de celda.
fn face_at(cell_pos: Vector2, direction: Vector2) -> (Side, f32) {
    let fx = cell_pos.x.rem_euclid(1.0);
    let fy = cell_pos.y.rem_euclid(1.0);

    // qué tan adentro de la celda quedó el impacto respecto de cada cara de entrada
    let into_x = if direction.x > 0.0 { fx } else { 1.0 - fx };
    let into_y = if direction.y > 0.0 { fy } else { 1.0 - fy };

    // la cara de entrada es la que quedó más cerca; la otra coordenada recorre la cara
    if into_x < into_y {
        let u = if direction.x > 0.0 { fy } else { 1.0 - fy };
        (Side::Vertical, u)
    } else {
        let u = if direction.y > 0.0 { 1.0 - fx } else { fx };
        (Side::Horizontal, u)
    }
}
