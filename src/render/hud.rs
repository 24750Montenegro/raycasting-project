use super::font;
use crate::config;
use crate::framebuffer::Framebuffer;
use crate::health::Health;
use crate::textures::{Texture, TextureManager};
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
        if let Some(texture) = textures.heart(frame) {
            draw_icon(framebuffer, texture, center, slot * beat);
        }
    }
}

/// Número de nivel arriba a la izquierda, puntaje arriba a la derecha y, abajo
/// a la derecha, cuántos objetos se llevan encima sin entregar todavía.
pub fn render_score(
    framebuffer: &mut Framebuffer,
    level: u32,
    score: u32,
    carried: u32,
    textures: &TextureManager,
) {
    let right = framebuffer.width as f32 - config::SCORE_MARGIN;

    font::draw_number(
        framebuffer,
        level,
        Vector2::new(config::SCORE_MARGIN, config::SCORE_MARGIN),
        config::LEVEL_BLOCK,
        config::LEVEL_COLOR,
    );

    font::draw_number(
        framebuffer,
        score,
        Vector2::new(
            right - font::number_width(score, config::SCORE_BLOCK),
            config::SCORE_MARGIN,
        ),
        config::SCORE_BLOCK,
        config::SCORE_COLOR,
    );

    let block = config::CARRY_BLOCK;
    // a la misma altura que los corazones, para que el HUD lea como una línea
    let baseline = framebuffer.height as f32 - config::HEART_MARGIN - config::HEART_SIZE / 2.0;
    let number_left = right - font::number_width(carried, block);

    font::draw_number(
        framebuffer,
        carried,
        Vector2::new(number_left, baseline - font::number_height(block) / 2.0),
        block,
        config::ITEM_COLOR,
    );

    let icon = Vector2::new(
        number_left - config::CARRY_SPACING - config::CARRY_ICON_SIZE / 2.0,
        baseline,
    );
    draw_carry_icon(framebuffer, textures, icon);
}

/// Ícono del contador: el sprite del primer tipo de objeto si ya tiene textura,
/// y mientras tanto un disco del color con el que se dibuja en el mundo.
fn draw_carry_icon(framebuffer: &mut Framebuffer, textures: &TextureManager, center: Vector2) {
    let size = config::CARRY_ICON_SIZE;
    let texture = config::ITEM_TEXTURES
        .first()
        .and_then(|&(kind, _)| textures.item(kind));

    match texture {
        Some(texture) => draw_icon(framebuffer, texture, center, size),
        None => framebuffer.fill_circle(center, size / 2.0, config::ITEM_COLOR),
    }
}

/// Al recibir un golpe los corazones pegan un salto de tamaño y vuelven.
fn beat_scale(health: &Health) -> f32 {
    match health.hit_progress() {
        Some(progress) => 1.0 + config::HEART_HIT_SCALE * (1.0 - progress).powi(2),
        None => 1.0,
    }
}

/// Dibuja una textura centrada en `center`, encajada en un cuadrado de lado
/// `size` sin deformarla: el cuadro no tiene por qué ser cuadrado.
fn draw_icon(framebuffer: &mut Framebuffer, texture: &Texture, center: Vector2, size: f32) {
    let aspect = texture.width() as f32 / texture.height() as f32;
    let fitted = if aspect >= 1.0 {
        Vector2::new(size, size / aspect)
    } else {
        Vector2::new(size * aspect, size)
    };

    super::blit(framebuffer, texture, center - fitted / 2.0, fitted);
}
