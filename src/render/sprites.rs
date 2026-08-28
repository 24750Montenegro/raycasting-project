use super::shading;
use crate::config;
use crate::enemy::Enemies;
use crate::framebuffer::Framebuffer;
use crate::items::Items;
use crate::player::Player;
use crate::textures::{ItemFace, Texture, TextureManager};
use raylib::prelude::*;

/// Una cosa del mundo que se dibuja mirando siempre a la cámara. Enemigos y
/// objetos solo se diferencian en estos datos, así que comparten el proyectado,
/// el orden por profundidad y el recorte contra las paredes.
struct Sprite<'a> {
    pos: Vector2,
    texture: Option<&'a Texture>,
    /// Color plano mientras esa entrada no tenga textura.
    color: Color,
    /// Alto como fracción de una celda, y ancho como fracción de ese alto.
    height: f32,
    aspect: f32,
    /// Cuánto se despega del piso, en fracción de una celda.
    lift: f32,
    /// Cuánto se mezcla con ENEMY_HIT_COLOR, para el destello del golpe.
    flash: f32,
}

/// Un sprite ya proyectado a pantalla. La luz y la niebla vienen colapsadas en
/// una recta por canal, igual que en las paredes.
struct Billboard<'a> {
    texture: Option<&'a Texture>,
    color: Color,
    /// Distancia sobre el eje de la cámara, que es contra la que se compara el
    /// depth buffer de las paredes.
    depth: f32,
    top_left: Vector2,
    size: Vector2,
    light: f32,
    flash: f32,
    fog: [f32; 3],
}

pub fn render_sprites(
    framebuffer: &mut Framebuffer,
    enemies: &Enemies,
    items: &Items,
    player: &Player,
    textures: &TextureManager,
    depth_buffer: &[f32],
    block_size: usize,
) {
    let half_width = framebuffer.width as f32 / 2.0;
    let horizon = framebuffer.height as f32 / 2.0;
    let projection = half_width / (player.fov / 2.0).tan();

    let enemies = enemies.iter().map(|enemy| Sprite {
        pos: enemy.pos,
        texture: textures.enemy(enemy.kind, enemy.attacking()),
        color: config::ENEMY_COLOR,
        height: config::ENEMY_SIZE,
        aspect: config::ENEMY_ASPECT,
        lift: enemy.lift(),
        flash: enemy.flash(),
    });
    let items = items.loose().map(|item| {
        let texture = textures.item(item.kind, ItemFace::Idle);
        Sprite {
            pos: item.pos,
            texture,
            color: config::ITEM_COLOR,
            height: config::ITEM_SIZE,
            // el ancho sale de la imagen: la cara de contento es más ancha que
            // la de siempre —abre las alas— y estirarla a un ancho fijo le
            // sacaría justo eso
            aspect: texture.map_or(config::ITEM_ASPECT, |texture| {
                texture.width() as f32 / texture.height() as f32
            }),
            // esperan apoyados en el piso: el festejo de levantarlos no pasa
            // acá sino en pantalla, que es donde entran enteros
            lift: 0.0,
            flash: 0.0,
        }
    });

    let mut visible: Vec<Billboard> = enemies
        .chain(items)
        .filter_map(|sprite| project(&sprite, player, half_width, horizon, projection, block_size))
        .collect();

    // de atrás hacia adelante, para que el de adelante tape al de atrás
    visible.sort_by(|a, b| b.depth.total_cmp(&a.depth));

    for billboard in &visible {
        draw(framebuffer, billboard, depth_buffer);
    }
}

fn project<'a>(
    sprite: &Sprite<'a>,
    player: &Player,
    half_width: f32,
    horizon: f32,
    projection: f32,
    block_size: usize,
) -> Option<Billboard<'a>> {
    let offset = sprite.pos - player.pos;
    let (sin, cos) = player.angle.sin_cos();

    // el offset rotado -angle: x queda hacia adelante, y hacia el costado
    let depth = offset.x * cos + offset.y * sin;
    let lateral = offset.y * cos - offset.x * sin;

    if depth < config::SPRITE_NEAR_PLANE {
        return None; // detrás de la cámara o encima del ojo
    }

    let block = block_size as f32;
    let cell = (block / depth) * projection; // lo que mide una celda a esa distancia
    let height = cell * sprite.height;
    // el alto manda y el ancho sale de su proporción: así un cuerpo angosto se
    // dibuja angosto sin tener que tocarle el alto
    let size = Vector2::new(height * sprite.aspect, height);
    let center_x = half_width + (lateral / depth) * projection;
    // apoyado en el piso, que es donde termina la pared de esa misma celda, y
    // levantado lo que pida el sprite
    let floor = horizon + cell / 2.0 - cell * sprite.lift;

    let distance = depth / block;
    let fog = shading::fog_factor(distance);

    Some(Billboard {
        texture: sprite.texture,
        color: sprite.color,
        depth,
        top_left: Vector2::new(center_x - size.x / 2.0, floor - size.y),
        size,
        light: shading::sprite_light(distance) * (1.0 - fog),
        flash: sprite.flash,
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
        // si la pared de esa columna está más cerca, el sprite queda tapado
        if depth_buffer[x as usize] <= billboard.depth {
            continue;
        }
        let u = (x as f32 + 0.5 - billboard.top_left.x) / billboard.size.x;

        for y in first_y..last_y {
            let v = (y as f32 + 0.5 - billboard.top_left.y) / billboard.size.y;
            let texel = match billboard.texture {
                Some(texture) => texture.sample(u, v),
                None => billboard.color,
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
        // el destello va antes que la luz: es el sprite el que se aclara, no la
        // luz que le llega, así que también se ve de lejos
        let texel = shading::mix(texel, config::ENEMY_HIT_COLOR, self.flash);
        let channel = |value: u8, fog: f32| (value as f32 * self.light + fog) as u8;

        Color::new(
            channel(texel.r, self.fog[0]),
            channel(texel.g, self.fog[1]),
            channel(texel.b, self.fog[2]),
            texel.a,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parado, el enemigo tiene que entrar entero por debajo del horizonte
    /// —que es la altura del ojo del jugador— y en el pico del salto tiene que
    /// pasarlo. Eso es lo que se ve: un bicho chiquito al que hay que mirar
    /// para abajo hasta que se te tira encima.
    #[test]
    fn el_salto_mete_al_enemigo_en_la_camara() {
        let player = Player {
            pos: Vector2::zero(),
            angle: 0.0,
            fov: config::FOV,
        };
        // de frente y a la distancia desde la que ataca, que es el peor caso:
        // cuanto más cerca, más grande se proyecta el sprite
        let enemy = |lift| Sprite {
            pos: Vector2::new(config::ENEMY_ATTACK_REACH, 0.0),
            texture: None,
            color: config::ENEMY_COLOR,
            height: config::ENEMY_SIZE,
            aspect: config::ENEMY_ASPECT,
            lift,
            flash: 0.0,
        };

        let half_width = config::WINDOW_WIDTH as f32 / 2.0;
        let horizon = config::WINDOW_HEIGHT as f32 / 2.0;
        let projection = half_width / (config::FOV / 2.0).tan();
        // la y crece hacia abajo: por encima del horizonte es un valor menor
        let top = |lift| {
            let sprite = enemy(lift);
            project(
                &sprite,
                &player,
                half_width,
                horizon,
                projection,
                config::BLOCK_SIZE,
            )
            .expect("el enemigo quedó fuera de la cámara")
            .top_left
            .y
        };

        assert!(
            top(0.0) > horizon,
            "parado le tapa la vista: su borde de arriba cae en {}",
            top(0.0)
        );
        assert!(
            top(config::ENEMY_ATTACK_LIFT) < horizon,
            "el salto no pasa la línea de la cámara: llega a {}",
            top(config::ENEMY_ATTACK_LIFT)
        );
    }
}
