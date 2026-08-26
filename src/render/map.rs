use super::cell_color;
use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use raylib::prelude::*;

const RAYS: usize = 60;

pub fn render_map(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: usize) {
    for (row, cells) in maze.iter().enumerate() {
        for (col, &cell) in cells.iter().enumerate() {
            framebuffer.fill_rect(
                (col * block_size) as i32,
                (row * block_size) as i32,
                block_size as i32,
                block_size as i32,
                cell_color(cell),
            );
        }
    }

    for i in 0..RAYS {
        let angle = player.angle - player.fov / 2.0 + (i as f32 / RAYS as f32) * player.fov;
        let hit = cast_ray(maze, player, angle, block_size);
        framebuffer.draw_line(player.pos, hit.hit, Color::WHITE);
    }

    framebuffer.fill_circle(player.pos, 6.0, Color::RED);
}
