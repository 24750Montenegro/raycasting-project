use super::cell_color;
use crate::caster::cast_ray;
use crate::config;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

pub fn render_world(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
) {
    let width = framebuffer.width as i32;
    let height = framebuffer.height as i32;
    let horizon = framebuffer.height as f32 / 2.0;
    // distancia del ojo al plano de proyección, en píxeles
    let projection = (framebuffer.width as f32 / 2.0) / (player.fov / 2.0).tan();

    framebuffer.fill_rect(0, 0, width, horizon as i32, config::SKY_COLOR);
    framebuffer.fill_rect(
        0,
        horizon as i32,
        width,
        height - horizon as i32,
        config::FLOOR_COLOR,
    );

    for x in 0..width {
        let angle = ray_angle(player, x as f32 + 0.5, framebuffer.width as f32);
        let hit = cast_ray(maze, player, angle, block_size);

        // proyectar sobre el eje de la cámara corrige el ojo de pez
        let depth = (hit.distance * (angle - player.angle).cos()).max(f32::EPSILON);
        let wall_height = (block_size as f32 / depth) * projection;

        let top = (horizon - wall_height / 2.0).max(0.0).round() as i32;
        let bottom = (horizon + wall_height / 2.0).min(height as f32).round() as i32;

        if bottom > top {
            framebuffer.fill_rect(x, top, 1, bottom - top, cell_color(hit.impact));
        }
    }
}

/// Ángulo del rayo que atraviesa la columna `x` de la pantalla.
pub fn ray_angle(player: &Player, x: f32, width: f32) -> f32 {
    player.angle - player.fov / 2.0 + (x / width) * player.fov
}
