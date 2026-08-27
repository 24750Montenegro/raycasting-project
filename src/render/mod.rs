mod font;
mod game_over;
mod hud;
mod minimap;
mod shading;
mod sprites;
mod weapon;
mod world;

pub use game_over::render_game_over;
pub use hud::{render_health, render_score};
pub use minimap::{render_minimap, MinimapMode};
pub use sprites::render_sprites;
pub use weapon::render_weapon;
pub use world::render_world;

use crate::config;
use crate::framebuffer::Framebuffer;
use crate::textures::Texture;
use raylib::prelude::*;

/// Dibuja una textura estirada al rectángulo dado, salteando lo transparente.
/// Lo usan el HUD y el arma, que pintan en coordenadas de pantalla y no pasan
/// por el proyectado de los billboards.
fn blit(framebuffer: &mut Framebuffer, texture: &Texture, top_left: Vector2, size: Vector2) {
    for y in top_left.y.floor() as i32..(top_left.y + size.y).ceil() as i32 {
        let v = (y as f32 + 0.5 - top_left.y) / size.y;

        for x in top_left.x.floor() as i32..(top_left.x + size.x).ceil() as i32 {
            let u = (x as f32 + 0.5 - top_left.x) / size.x;
            let texel = texture.sample(u, v);

            if texel.a > config::SPRITE_ALPHA_CUTOFF {
                framebuffer.blend_pixel(x, y, texel);
            }
        }
    }
}

//llenar celdas
pub fn cell_color(cell: char) -> Color {
    match cell {
        '+' => Color::BLUE,
        '-' => Color::YELLOW,
        '|' => Color::GRAY,
        'g' => Color::RED,
        '#' => Color::BLACK,
        _ => Color::new(18, 19, 20, 255),
    }
}
