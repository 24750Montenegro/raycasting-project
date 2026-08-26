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
    /// Distancia euclidiana desde el jugador, en pixeles del mundo.
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

    let inv_block = 1.0 / block;
    let mut near = 0.0f32;
    let mut near_cell = cell_at(player.pos, inv_block);

    while near < max_distance {
        let far = near + step;
        let point = player.pos + direction * far;
        let far_cell = cell_at(point, inv_block);

        // la celda de la esquina va primero: si hay pared ahi, esta mas cerca
        // que cualquier cosa que encuentre la muestra de `far`
        if let Some(corner) =
            corner_between(maze, player, direction, near, far, near_cell, far_cell, block_size)
        {
            return corner;
        }

        if solid_at(maze, point, block_size).is_some() {
            return refine(maze, player.pos, direction, near, far, block_size);
        }

        near = far;
        near_cell = far_cell;
    }

    Intersect {
        distance: max_distance,
        impact: OUT_OF_BOUNDS,
        side: Side::Vertical,
        hit: player.pos + direction * max_distance,
        tex_u: 0.0,
    }
}

/// El march mira puntos, no segmentos. Entre dos muestras el rayo cruza a lo
/// sumo una linea de celda por eje; si cruzo las dos, atraveso una celda
/// intermedia que nunca se muestreo, y si esa celda era pared el rayo se colaba
/// por la esquina. Eso es lo que hacia temblar las esquinas al moverse.
fn corner_between(
    maze: &Maze,
    player: &Player,
    direction: Vector2,
    near: f32,
    far: f32,
    from: (i32, i32),
    to: (i32, i32),
    block_size: usize,
) -> Option<Intersect> {
    if from.0 == to.0 || from.1 == to.1 {
        return None; // cruce simple: no queda ninguna celda sin mirar
    }
    let block = block_size as f32;

    // distancia a la que el rayo cruza cada una de las dos lineas
    let line_x = if direction.x > 0.0 { to.0 } else { from.0 } as f32 * block;
    let line_y = if direction.y > 0.0 { to.1 } else { from.1 } as f32 * block;
    let to_vertical = (line_x - player.pos.x) / direction.x;
    let to_horizontal = (line_y - player.pos.y) / direction.y;

    // el punto medio entre ambos cruces cae dentro de la celda salteada
    let middle = (to_vertical + to_horizontal) / 2.0;
    if !middle.is_finite() || middle < near || middle > far {
        return None; // las cuentas no cuadran con los indices: mejor no tocar nada
    }
    let impact = solid_at(maze, player.pos + direction * middle, block_size)?;

    // se entra por la linea que se cruza primero
    let side = if to_vertical < to_horizontal {
        Side::Vertical
    } else {
        Side::Horizontal
    };
    let distance = to_vertical.min(to_horizontal);
    let hit = player.pos + direction * distance;

    Some(Intersect {
        distance: distance.max(f32::EPSILON),
        impact,
        side,
        hit,
        tex_u: face_coordinate(hit / block, direction, side),
    })
}

/// Biseca entre el ultimo punto vacio y el primero solido. Sin esto la distancia
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
    let cell_pos = hit / block_size as f32;
    let side = face_at(cell_pos, direction);

    Intersect {
        distance: solid.max(f32::EPSILON),
        impact: solid_at(maze, hit, block_size).unwrap_or(OUT_OF_BOUNDS),
        side,
        hit,
        tex_u: face_coordinate(cell_pos, direction, side),
    }
}

/// Celda que ocupa el punto. Recibe el inverso del lado para no dividir en cada
/// paso del march.
#[inline]
fn cell_at(point: Vector2, inv_block: f32) -> (i32, i32) {
    (
        (point.x * inv_block).floor() as i32,
        (point.y * inv_block).floor() as i32,
    )
}

/// Cara por la que el rayo entro a la celda: la que quedo mas cerca del impacto.
/// `cell_pos` viene en unidades de celda.
fn face_at(cell_pos: Vector2, direction: Vector2) -> Side {
    let fx = cell_pos.x.rem_euclid(1.0);
    let fy = cell_pos.y.rem_euclid(1.0);

    let into_x = if direction.x > 0.0 { fx } else { 1.0 - fx };
    let into_y = if direction.y > 0.0 { fy } else { 1.0 - fy };

    if into_x < into_y {
        Side::Vertical
    } else {
        Side::Horizontal
    }
}

/// Coordenada horizontal del impacto sobre la cara. El sentido del rayo decide
/// hacia donde corre, para que la textura no salga espejada entre caras.
fn face_coordinate(cell_pos: Vector2, direction: Vector2, side: Side) -> f32 {
    match side {
        Side::Vertical => {
            let fy = cell_pos.y.rem_euclid(1.0);
            if direction.x > 0.0 { fy } else { 1.0 - fy }
        }
        Side::Horizontal => {
            let fx = cell_pos.x.rem_euclid(1.0);
            if direction.y > 0.0 { 1.0 - fx } else { fx }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::load_maze;

    /// March de referencia: paso finisimo, sin bisecar ni corregir esquinas.
    fn reference(maze: &Maze, origin: Vector2, angle: f32, block_size: usize) -> f32 {
        let direction = Vector2::new(angle.cos(), angle.sin());
        let step = 0.0005 * block_size as f32;
        let mut distance = 0.0f32;

        while distance < config::RAY_MAX_DEPTH * block_size as f32 {
            distance += step;
            if solid_at(maze, origin + direction * distance, block_size).is_some() {
                return distance;
            }
        }
        distance
    }

    /// Ningun rayo debe atravesar una pared. Cuando el march solo miraba puntos,
    /// los rayos que rozaban un vertice se colaban hasta la pared de atras, y
    /// como que se cuelen depende de donde caen las muestras, las esquinas
    /// temblaban al moverse.
    #[test]
    fn ningun_rayo_atraviesa_una_pared() {
        let maze = load_maze("maze.txt");
        let block = 40usize;
        let mut fugas = 0;

        for row in 1..7 {
            for col in 1..29 {
                let pos = Vector2::new(
                    col as f32 * block as f32 + 20.0,
                    row as f32 * block as f32 + 20.0,
                );
                if solid_at(&maze, pos, block).is_some() {
                    continue;
                }
                let player = Player { pos, angle: 0.0, fov: config::FOV };

                for i in 0..720 {
                    let angle = i as f32 * std::f32::consts::TAU / 720.0;

                    // solo es fuga ver MAS lejos que la referencia; frenar en el
                    // vertice exacto de una diagonal es lo conservador y no
                    // parpadea, porque no depende del paso del march
                    let exceso = cast_ray(&maze, &player, angle, block).distance
                        - reference(&maze, pos, angle, block);

                    if exceso > 1.0 {
                        fugas += 1;
                    }
                }
            }
        }
        assert_eq!(fugas, 0, "hay rayos que atraviesan paredes");
    }
}
