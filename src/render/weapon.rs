//! El arma en primer plano. Mientras ATTACK_SHEET no tenga una hoja cargada se
//! dibuja un puño de bloques que hace el mismo recorrido: el golpe se ve y se
//! puede afinar el tiempo, y cuando aparezcan las imágenes solo hay que apuntar
//! la config a la hoja.

use crate::attack::Attack;
use crate::config;
use crate::framebuffer::Framebuffer;
use crate::textures::TextureManager;
use raylib::prelude::*;
use std::f32::consts::PI;

pub fn render_weapon(framebuffer: &mut Framebuffer, attack: &Attack, textures: &TextureManager) {
    let screen = Vector2::new(framebuffer.width as f32, framebuffer.height as f32);
    let height = screen.y * config::WEAPON_HEIGHT;
    let size = Vector2::new(height * config::WEAPON_ASPECT, height);

    // el golpe sale y vuelve dentro del mismo ciclo: medio seno da ese
    // recorrido sin tener que listar posiciones cuadro por cuadro
    let swing = attack.progress().map_or(0.0, |progress| (progress * PI).sin());
    let margin = screen.y * config::WEAPON_MARGIN;

    let top_left = Vector2::new(
        screen.x - size.x - margin - size.x * config::WEAPON_SWING * swing,
        screen.y - size.y + margin - size.y * config::WEAPON_SWING * swing,
    );

    match textures.weapon(attack.frame()) {
        Some(texture) => super::blit(framebuffer, texture, top_left, size),
        None => draw_placeholder(framebuffer, top_left, size, swing),
    }
}

/// Puño de reemplazo: un antebrazo y una mano, que es lo mínimo para leer el
/// gesto. Se inclina un poco con el golpe recortando el ancho de cada fila.
fn draw_placeholder(framebuffer: &mut Framebuffer, top_left: Vector2, size: Vector2, swing: f32) {
    let hand = Vector2::new(size.x, size.x * 0.8);
    let arm = Vector2::new(size.x * 0.55, size.y - hand.y);

    framebuffer.fill_rect(
        (top_left.x + (size.x - hand.x) / 2.0) as i32,
        top_left.y as i32,
        hand.x as i32,
        hand.y as i32,
        config::WEAPON_COLOR,
    );
    framebuffer.fill_rect(
        (top_left.x + (size.x - arm.x) / 2.0 + size.x * 0.2 * swing) as i32,
        (top_left.y + hand.y) as i32,
        arm.x as i32,
        arm.y as i32,
        config::WEAPON_ARM_COLOR,
    );
}
