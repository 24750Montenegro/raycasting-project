mod font;
mod game_over;
mod hud;
mod minimap;
mod shading;
mod sprites;
mod weapon;
mod world;

pub use game_over::render_banner;
pub use hud::{render_health, render_pickup, render_score};
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

/// Lo mismo pero girando la textura `angle` radianes alrededor de `pivot`, que
/// va en fracción del rectángulo: (0, 0) es su esquina de arriba a la izquierda
/// y (1, 1) la de abajo a la derecha. Recorre el rectángulo que ocupa ya girada
/// y para cada píxel deshace el giro para saber de qué téxel sale: al revés
/// —recorriendo la textura y pintando cada téxel donde caiga— el estirón dejaría
/// agujeros entre píxel y píxel.
fn blit_rotated(
    framebuffer: &mut Framebuffer,
    texture: &Texture,
    top_left: Vector2,
    size: Vector2,
    angle: f32,
    pivot: Vector2,
) {
    // el ángulo crece en el sentido de las agujas del reloj: la y de la
    // pantalla apunta hacia abajo
    let (sin, cos) = angle.sin_cos();
    let center = top_left + size * pivot;
    let corner = size * pivot; // el pivote visto desde la esquina de la textura

    let (mut min, mut max) = (
        Vector2::new(f32::INFINITY, f32::INFINITY),
        Vector2::new(f32::NEG_INFINITY, f32::NEG_INFINITY),
    );
    for offset in [
        Vector2::zero(),
        Vector2::new(size.x, 0.0),
        Vector2::new(0.0, size.y),
        size,
    ] {
        let local = offset - corner;
        let turned = Vector2::new(
            local.x * cos - local.y * sin,
            local.x * sin + local.y * cos,
        );
        min = Vector2::new(min.x.min(turned.x), min.y.min(turned.y));
        max = Vector2::new(max.x.max(turned.x), max.y.max(turned.y));
    }

    for y in (center.y + min.y).floor() as i32..(center.y + max.y).ceil() as i32 {
        for x in (center.x + min.x).floor() as i32..(center.x + max.x).ceil() as i32 {
            let offset = Vector2::new(x as f32 + 0.5 - center.x, y as f32 + 0.5 - center.y);
            // el giro de vuelta, que es el mismo con el ángulo cambiado de signo
            let local = Vector2::new(
                offset.x * cos + offset.y * sin,
                offset.y * cos - offset.x * sin,
            ) + corner;

            let (u, v) = (local.x / size.x, local.y / size.y);
            if !(0.0..1.0).contains(&u) || !(0.0..1.0).contains(&v) {
                continue;
            }

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
