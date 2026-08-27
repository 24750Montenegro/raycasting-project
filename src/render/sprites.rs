use super::shading;
use crate::config;
use crate::enemy::Enemies;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::textures::{Texture, TextureManager};
use raylib::prelude::*;

/// Un enemigo ya proyectado a pantalla. La luz y la niebla vienen colapsadas en
/// una recta por canal, igual que en las paredes.
struct Billboard<'a> {
    texture: Option<&'a Texture>,
    /// Distancia sobre el eje de la cámara, que es contra la que se compara el
    /// depth buffer de las paredes.
    depth: f32,
    top_left: Vector2,
    size: Vector2,
    light: f32,
    fog: [f32; 3],
}

pub fn render_enemies(
    framebuffer: &mut Framebuffer,
    enemies: &Enemies,
    player: &Player,
    textures: &TextureManager,
    depth_buffer: &[f32],
    block_size: usize,
) {
    let half_width = framebuffer.width as f32 / 2.0;
    let horizon = framebuffer.height as f32 / 2.0;
    let projection = half_width / (player.fov / 2.0).tan();

    let mut visible: Vec<Billboard> = enemies
        .iter()
        .filter_map(|enemy| {
            project(
                enemy.pos,
                enemy.kind,
                player,
                textures,
                half_width,
                horizon,
                projection,
                block_size,
            )
        })
        .collect();

    // de atrás hacia adelante, para que el de adelante tape al de atrás
    visible.sort_by(|a, b| b.depth.total_cmp(&a.depth));

    for billboard in &visible {
        draw(framebuffer, billboard, depth_buffer);
    }
}

#[allow(clippy::too_many_arguments)]
fn project<'a>(
    pos: Vector2,
    kind: char,
    player: &Player,
    textures: &'a TextureManager,
    half_width: f32,
    horizon: f32,
    projection: f32,
    block_size: usize,
) -> Option<Billboard<'a>> {
    let offset = pos - player.pos;
    let (sin, cos) = player.angle.sin_cos();

    // el offset rotado -angle: x queda hacia adelante, y hacia el costado
    let depth = offset.x * cos + offset.y * sin;
    let lateral = offset.y * cos - offset.x * sin;

    if depth < config::SPRITE_NEAR_PLANE {
        return None; // detrás de la cámara o encima del ojo
    }

    let block = block_size as f32;
    let cell = (block / depth) * projection; // lo que mide una celda a esa distancia
    let height = cell * config::ENEMY_SIZE;
    // el enemigo es angosto: el alto manda y el ancho sale de su proporción
    let size = Vector2::new(height * config::ENEMY_ASPECT, height);
    let center_x = half_width + (lateral / depth) * projection;
    // apoyado en el piso, que es donde termina la pared de esa misma celda
    let floor = horizon + cell / 2.0;

    let distance = depth / block;
    let fog = shading::fog_factor(distance);

    Some(Billboard {
        texture: textures.enemy(kind),
        depth,
        top_left: Vector2::new(center_x - size.x / 2.0, floor - size.y),
        size,
        light: shading::sprite_light(distance) * (1.0 - fog),
        fog: [
            config::FOG_COLOR.r as f32 * fog,
            config::FOG_COLOR.g as f32 * fog,
            config::FOG_COLOR.b as f32 * fog,
        ],
    })
}

fn draw(framebuffer: &mut Framebuffer, billboard: &Billboard, depth_buffer: &[f32]) {
    let first_x = billboard.top_left.x.floor().max(0.0) as i32;
    let last_x = (billboard.top_left.x + billboard.size.x)
        .ceil()
        .min(framebuffer.width as f32) as i32;
    let first_y = billboard.top_left.y.floor().max(0.0) as i32;
    let last_y = (billboard.top_left.y + billboard.size.y)
        .ceil()
        .min(framebuffer.height as f32) as i32;

    for x in first_x..last_x {
        // si la pared de esa columna está más cerca, el enemigo queda tapado
        if depth_buffer[x as usize] <= billboard.depth {
            continue;
        }
        let u = (x as f32 + 0.5 - billboard.top_left.x) / billboard.size.x;

        for y in first_y..last_y {
            let v = (y as f32 + 0.5 - billboard.top_left.y) / billboard.size.y;
            let texel = match billboard.texture {
                Some(texture) => texture.sample(u, v),
                None => config::ENEMY_COLOR,
            };

            if texel.a <= config::SPRITE_ALPHA_CUTOFF {
                continue;
            }
            framebuffer.blend_pixel(x, y, billboard.tint(texel));
        }
    }
}

impl Billboard<'_> {
    #[inline]
    fn tint(&self, texel: Color) -> Color {
        let channel = |value: u8, fog: f32| (value as f32 * self.light + fog) as u8;

        Color::new(
            channel(texel.r, self.fog[0]),
            channel(texel.g, self.fog[1]),
            channel(texel.b, self.fog[2]),
            texel.a,
        )
    }
}
