use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::maze::{Maze, is_solid};
use crate::player::Player;

pub enum Side {
    Vertical,
    Horizontal,
}

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub side: Side,
    pub hit_x: f32,
    pub hit_y: f32,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool, // para ver los rayos en 2D
) -> Intersect {
    let mut prev_i = player.pos.x.max(0.0) as usize / block_size;
    let mut prev_j = player.pos.y.max(0.0) as usize / block_size;
    let mut d = 0.0f32;
    framebuffer.set_current_color(Color::WHITE);

    loop {
        let x = player.pos.x + d * a.cos();
        let y = player.pos.y + d * a.sin();

        // Verificar si el rayo ha salido del laberinto
        if x < 0.0 || y < 0.0 {
            return Intersect { distance: d.max(0.0001), impact: '#', side: Side::Vertical, hit_x: x, hit_y: y }; // fuera del laberinto
        }

        let i = x as usize / block_size; // columna
        let j = y as usize / block_size; // fila

        let cell = maze
            .get(j)
            .and_then(|row| row.get(i))
            .copied();

        match cell {
            Some(c) if is_solid(c) => {
                let side = pick_side(
                    i != prev_i,
                    j != prev_j,
                    x / block_size as f32,
                    y / block_size as f32,
                    a.cos(),
                    a.sin(),
                );
                return Intersect {
                    distance: d.max(0.0001),
                    impact: c,
                    side,
                    hit_x: x,
                    hit_y: y,
                };
            }

            None => {
                return Intersect {
                    distance: d.max(0.0001),
                    impact: '#',
                    side: Side::Vertical,
                    hit_x: x,
                    hit_y: y,
                };
            }

            _ => {}
        }

        if draw_line {
            framebuffer.set_pixel(x as u32, y as u32);
        }

        prev_i = i;
        prev_j = j;
        d += 1.0;
    }
}

fn pick_side(cambio_i: bool, cambio_j: bool, cx: f32, cy: f32, cos_a: f32, sin_a: f32) -> Side {
    match (cambio_i, cambio_j) {
        (true, false) => Side::Vertical,
        (false, true) => Side::Horizontal,
        _ => {
            let fx = cx - cx.floor();   // qué tan adentro de la celda vamos, en X (0.0 .. 1.0)
            let fy = cy - cy.floor();
            let past_x = if cos_a > 0.0 { fx } else { 1.0 - fx };
            let past_y = if sin_a > 0.0 { fy } else { 1.0 - fy };
            if past_x < past_y { Side::Vertical } else { Side::Horizontal }
        }
    }
}
