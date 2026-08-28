//! El bate en primer plano. La animación del golpe no sale de una hoja de
//! cuadros dibujados: es la misma imagen girada, corrida y agrandada a lo largo
//! del ciclo del golpe. Con una sola pieza de arte se ve el envión hacia atrás,
//! el barrido que cruza la pantalla y la vuelta a la esquina, y lo que se ve
//! queda atado al mismo reloj que resuelve el daño.

use crate::attack::Attack;
use crate::config;
use crate::framebuffer::Framebuffer;
use crate::textures::TextureManager;
use raylib::prelude::*;
use std::f32::consts::PI;

pub fn render_weapon(framebuffer: &mut Framebuffer, attack: &Attack, textures: &TextureManager) {
    let Some(texture) = textures.weapon(attack.frame()) else {
        return; // sin imagen no hay arma que dibujar
    };

    let screen = Vector2::new(framebuffer.width as f32, framebuffer.height as f32);
    let height = screen.y * config::WEAPON_HEIGHT;
    let rest = Vector2::new(
        height * texture.width() as f32 / texture.height() as f32,
        height,
    );

    let swing = attack.progress().map_or(0.0, swing_curve);
    let margin = screen.y * config::WEAPON_MARGIN;

    // hacia atrás toma envión y hacia adelante barre, con arcos bien distintos:
    // el envión es apenas un gesto y la ida cruza media pantalla. El giro va al
    // revés de las agujas del reloj porque la y de la pantalla crece hacia abajo
    let arc = if swing < 0.0 {
        config::WEAPON_WINDUP
    } else {
        config::WEAPON_SWING_ARC
    };
    // el bate se corre en la misma proporción en la que gira, así el envión se
    // recuesta contra la esquina en vez de irse de la pantalla
    let shift = swing * arc / config::WEAPON_SWING_ARC * config::WEAPON_SWING;

    // y crece hacia el contacto: es lo que lo acerca a la cámara y hace que el
    // golpe se lea como uno hacia adelante y no como uno de costado
    let size = rest * (1.0 + config::WEAPON_THRUST * swing.max(0.0));
    // el puño es el punto que se queda quieto, tanto al girar como al crecer
    let hand = Vector2::new(
        screen.x - rest.x - margin - rest.x * shift,
        screen.y - rest.y + margin - rest.y * shift,
    ) + rest * config::WEAPON_PIVOT;
    let top_left = hand - size * config::WEAPON_PIVOT;

    super::blit_rotated(
        framebuffer,
        texture,
        top_left,
        size,
        -swing * arc,
        config::WEAPON_PIVOT,
    );
}

/// Dónde está el bate a lo largo del golpe, en [-1, 1]: negativo mientras toma
/// envión hacia atrás, 1 justo en el instante del contacto y de vuelta en 0 al
/// terminar la recuperación. Sale de dos tramos de seno que se tocan en el
/// contacto, así que el recorrido no tiene saltos y correr ATTACK_CONTACT
/// corre a la vez el golpe que se ve y el que se resuelve.
fn swing_curve(progress: f32) -> f32 {
    let contact = config::ATTACK_CONTACT.clamp(0.05, 0.95);

    if progress < contact {
        // -1 a un tercio del camino —el punto más atrás del envión— y +1 al
        // llegar al contacto
        return -(progress / contact * 1.5 * PI).sin();
    }
    // y de ahí vuelve al reposo, frenando sobre el final
    ((progress - contact) / (1.0 - contact) * PI / 2.0).cos()
}
