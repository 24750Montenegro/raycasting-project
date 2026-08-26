use crate::config;
use crate::framebuffer::Framebuffer;
use crate::health::Health;
use crate::textures::TextureManager;
use raylib::prelude::*;

/// Corazones abajo a la izquierda. Cada slot vale dos medios: se dibuja lleno,
/// a la mitad o vacío según cuántos medios de vida queden por delante de él.
pub fn render_health(framebuffer: &mut Framebuffer, health: &Health, textures: &TextureManager) {
    let slot = config::HEART_SIZE;
    let baseline = framebuffer.height as f32 - config::HEART_MARGIN - slot / 2.0;
    let beat = beat_scale(health);

    for index in 0..config::HEART_SLOTS {
        let remaining = health.halves() as i32 - index as i32 * 2;
        let frame = match remaining {
            2.. => config::HEART_FRAME_FULL,
            1 => config::HEART_FRAME_HALF,
            _ => config::HEART_FRAME_EMPTY,
        };

        // el slot no se mueve aunque el corazón lata: crece desde su centro
        let center = Vector2::new(
            config::HEART_MARGIN + index as f32 * (slot + config::HEART_SPACING) + slot / 2.0,
            baseline,
        );
        draw_heart(framebuffer, textures, frame, center, slot * beat);
    }
}

/// Al recibir un golpe los corazones pegan un salto de tamaño y vuelven.
fn beat_scale(health: &Health) -> f32 {
    match health.hit_progress() {
        Some(progress) => 1.0 + config::HEART_HIT_SCALE * (1.0 - progress).powi(2),
        None => 1.0,
    }
}

fn draw_heart(
    framebuffer: &mut Framebuffer,
    textures: &TextureManager,
    frame: usize,
    center: Vector2,
    size: f32,
) {
    let Some(texture) = textures.heart(frame) else {
        return;
    };

    // el cuadro no tiene por qué ser cuadrado: se encaja en el slot sin deformar
    let aspect = texture.width() as f32 / texture.height() as f32;
    let (width, height) = if aspect >= 1.0 {
        (size, size / aspect)
    } else {
        (size * aspect, size)
    };
    let left = center.x - width / 2.0;
    let top = center.y - height / 2.0;

    for y in top.floor() as i32..(top + height).ceil() as i32 {
        let v = (y as f32 + 0.5 - top) / height;

        for x in left.floor() as i32..(left + width).ceil() as i32 {
            let u = (x as f32 + 0.5 - left) / width;
            let texel = texture.sample(u, v);

            if texel.a > config::SPRITE_ALPHA_CUTOFF {
                framebuffer.blend_pixel(x, y, texel);
            }
        }
    }
}
