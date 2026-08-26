use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::caster::cast_ray;

//llenar celdas
fn cell_color(cell: char) -> Color {
    match cell{
        '+' => Color::BLUE,
        '-' => Color::YELLOW,
        '|' => Color::GRAY,
        'g' => Color::RED,
        '#' => Color::BLACK,
        _ => Color::new(18, 19, 20, 255),   
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell:char) {
    let color = cell_color(cell);
    framebuffer.set_current_color(color);

    for y in yo..(yo + block_size) {
        for x in xo..(xo + block_size) {
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }
}

pub fn render_maze(framebuffer: &mut Framebuffer, maze: &Maze, block_size: usize, player: &Player) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }

    // Dibujar jugador
    framebuffer.set_current_color(Color::RED);
    let px = player.pos.x as i32;
    let py = player.pos.y as i32;

    //dibujar circulo
    let dr: i32 = 6; // radio del circulo
    for dy in -dr..=dr {
        for dx in -dr..=dr {
            if dx * dx + dy * dy > dr * dr {
                continue; // fuera del circulo
            }
            let x = px + dx;
            let y = py + dy;
            if x >= 0 && y >= 0 {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }

    // Dibujar rayos
    let num_rays = 30;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let ray_angle = player.angle - player.fov / 2.0 + current_ray * player.fov;
        cast_ray(framebuffer, maze, player, ray_angle, block_size, true);
    }
}

pub fn render_world(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
){
    let width = framebuffer.width;
    let height = framebuffer.height;
    let hw = width as f32 / 2.0; // mitad del ancho
    let hh = height as f32 / 2.0; // mitad del alto

    let dpp = hw / (player.fov / 2.0).tan(); // distancia a la pared en pixeles

    let half = (height / 2) as i32;
    framebuffer.fill_rect(0, 0, width as i32, half, Color::SKYBLUE); // cielo
    framebuffer.fill_rect(0, half, width as i32, half, Color::new(18, 19, 20, 255)); // suelo

    for i in 0..width {
        let current_ray = i as f32 / width as f32;
        let ray_angle = player.angle - player.fov / 2.0 + current_ray * player.fov;
        let intersect = cast_ray(framebuffer, maze, player, ray_angle, block_size, false);

        // Corregir efecto de distorsión
        let corrected_distance = (intersect.distance * (player.angle - ray_angle).cos()).max(0.0001); // evitar división por cero

        // Altura de la pared en pixeles
        let wall_height = (block_size as f32 / corrected_distance ) * dpp;

        let top = (hh - wall_height / 2.0).max(0.0) as i32;
        let bottom = (hh + wall_height / 2.0).min(height as f32) as i32;

        let mut color = cell_color(intersect.impact);

        if (bottom > top){
            framebuffer.fill_rect(i as i32, top , 1, bottom - top, color);
        }
    }
}
