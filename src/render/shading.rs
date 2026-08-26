use crate::caster::Side;
use crate::config;
use raylib::prelude::*;

/// Factor de luz en [0, 1] para una pared a `distance` celdas del jugador.
pub fn wall_light(distance: f32, side: Side) -> f32 {
    let falloff = (1.0 - distance / config::LIGHT_RANGE)
        .clamp(0.0, 1.0)
        .powf(config::LIGHT_FALLOFF);
    let lit = config::LIGHT_AMBIENT
        + (1.0 - config::LIGHT_AMBIENT) * falloff * config::LIGHT_INTENSITY;

    (lit * face_light(side)).clamp(0.0, 1.0)
}

/// Las caras horizontales se oscurecen a propósito: esa diferencia constante es
/// lo que hace que las esquinas se lean como esquinas y no como un plano continuo.
fn face_light(side: Side) -> f32 {
    match side {
        Side::Vertical => config::LIGHT_VERTICAL_FACE,
        Side::Horizontal => config::LIGHT_HORIZONTAL_FACE,
    }
}

pub fn shade(color: Color, light: f32) -> Color {
    Color::new(
        (color.r as f32 * light) as u8,
        (color.g as f32 * light) as u8,
        (color.b as f32 * light) as u8,
        color.a,
    )
}
