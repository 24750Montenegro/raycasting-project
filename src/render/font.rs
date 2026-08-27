//! Dígitos de bloques para el HUD. Son los mismos bloques del cartel de game
//! over: cada carácter distinto de espacio se pinta como un cuadrado, así que
//! el número entra en el framebuffer sin depender de una fuente cargada.

use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

/// Ancho y alto de un dígito, en bloques, más la separación entre dos.
const DIGIT_COLUMNS: f32 = 3.0;
const DIGIT_ROWS: f32 = 5.0;
const DIGIT_SPACING: f32 = 1.0;

const DIGITS: [[&str; 5]; 10] = [
    ["###", "# #", "# #", "# #", "###"],
    ["  #", "  #", "  #", "  #", "  #"],
    ["###", "  #", "###", "#  ", "###"],
    ["###", "  #", "###", "  #", "###"],
    ["# #", "# #", "###", "  #", "  #"],
    ["###", "#  ", "###", "  #", "###"],
    ["###", "#  ", "###", "# #", "###"],
    ["###", "  #", "  #", "  #", "  #"],
    ["###", "# #", "###", "# #", "###"],
    ["###", "# #", "###", "  #", "###"],
];

/// Ancho en pantalla que va a ocupar `value` con bloques de lado `block`.
pub fn number_width(value: u32, block: f32) -> f32 {
    let digits = digits_of(value).len() as f32;
    (digits * DIGIT_COLUMNS + (digits - 1.0) * DIGIT_SPACING) * block
}

pub fn number_height(block: f32) -> f32 {
    DIGIT_ROWS * block
}

/// Dibuja `value` con su esquina superior izquierda en `top_left`.
pub fn draw_number(
    framebuffer: &mut Framebuffer,
    value: u32,
    top_left: Vector2,
    block: f32,
    color: Color,
) {
    let mut left = top_left.x;

    for digit in digits_of(value) {
        draw_glyph(framebuffer, &DIGITS[digit as usize], left, top_left.y, block, color);
        left += (DIGIT_COLUMNS + DIGIT_SPACING) * block;
    }
}

fn draw_glyph(
    framebuffer: &mut Framebuffer,
    glyph: &[&str; 5],
    left: f32,
    top: f32,
    block: f32,
    color: Color,
) {
    for (row, line) in glyph.iter().enumerate() {
        for (column, symbol) in line.chars().enumerate() {
            if symbol == ' ' {
                continue;
            }

            // se redondea desde las esquinas para que los bloques vecinos queden
            // pegados aunque el lado no sea entero
            let x = left + column as f32 * block;
            let y = top + row as f32 * block;

            framebuffer.fill_rect(
                x.round() as i32,
                y.round() as i32,
                (x + block).round() as i32 - x.round() as i32,
                (y + block).round() as i32 - y.round() as i32,
                color,
            );
        }
    }
}

/// Cifras de `value`, de la más significativa a la menos. El cero es un dígito.
fn digits_of(value: u32) -> Vec<u32> {
    if value == 0 {
        return vec![0];
    }

    let mut digits = Vec::new();
    let mut left = value;
    while left > 0 {
        digits.push(left % 10);
        left /= 10;
    }
    digits.reverse();
    digits
}
