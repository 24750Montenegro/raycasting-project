use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;


pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player, 
    a: f32,
    block_size: usize,
    draw_line: bool, // para ver los rayos en 2D
) -> Intersect {
    let mut d = 0.0f32;
    framebuffer.set_current_color(Color::WHITE);

    loop {
        let x = player.pos.x + d * a.cos();
        let y = player.pos.y + d * a.sin();

        // Verificar si el rayo ha salido del laberinto
        if x < 0.0 || y < 0.0 {
            return Intersect { distance: d, impact: '#' }; // fuera del laberinto
        }

        let i = x as usize / block_size; // columna
        let j = y as usize / block_size; // fila

        match maze.get(j).and_then(|row| row.get(i)){
            Some(&cell) if cell != ' ' && cell != 'p' => {
                return Intersect { distance: d, impact: cell };
            }

            None => {
                // El rayo ha salido del laberinto hacemos que choque
                return Intersect { distance: d, impact: '#' }; // fuera del laberinto
            }

            _ => {}
        }

        if draw_line {
            framebuffer.set_pixel(x as u32, y as u32);
        }

        d += 1.0;



    }
}