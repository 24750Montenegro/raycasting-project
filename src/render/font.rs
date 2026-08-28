//! Letras y dígitos de bloques. Son los mismos bloques del cartel de game
//! over: cada carácter distinto de espacio se pinta como un cuadrado, así que
//! el texto entra en el framebuffer sin depender de una fuente cargada. Todo
//! glifo mide 3x5 bloques, que es lo más chico donde el abecedario entero
//! todavía se lee.

use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

/// Ancho y alto de un dígito, en bloques, más la separación entre dos.
const DIGIT_COLUMNS: f32 = 3.0;
const DIGIT_ROWS: f32 = 5.0;
const DIGIT_SPACING: f32 = 1.0;

type Glyph = [&'static str; 5];

const DIGITS: [Glyph; 10] = [
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

/// El abecedario, en el mismo alto que los dígitos. Sin minúsculas: a 3x5 no
/// entran, y todo lo que escribe el juego son carteles cortos en mayúscula.
const LETTERS: [Glyph; 26] = [
    ["###", "# #", "###", "# #", "# #"], // A
    ["## ", "# #", "## ", "# #", "## "], // B
    ["###", "#  ", "#  ", "#  ", "###"], // C
    ["## ", "# #", "# #", "# #", "## "], // D
    ["###", "#  ", "###", "#  ", "###"], // E
    ["###", "#  ", "###", "#  ", "#  "], // F
    ["###", "#  ", "# #", "# #", "###"], // G
    ["# #", "# #", "###", "# #", "# #"], // H
    ["###", " # ", " # ", " # ", "###"], // I
    ["  #", "  #", "  #", "# #", "###"], // J
    ["# #", "# #", "## ", "# #", "# #"], // K
    ["#  ", "#  ", "#  ", "#  ", "###"], // L
    ["# #", "###", "###", "# #", "# #"], // M
    ["###", "# #", "# #", "# #", "# #"], // N
    ["###", "# #", "# #", "# #", "###"], // O
    ["###", "# #", "###", "#  ", "#  "], // P
    ["###", "# #", "# #", "###", "  #"], // Q
    ["## ", "# #", "## ", "# #", "# #"], // R
    ["###", "#  ", "###", "  #", "###"], // S
    ["###", " # ", " # ", " # ", " # "], // T
    ["# #", "# #", "# #", "# #", "###"], // U
    ["# #", "# #", "# #", "# #", " # "], // V
    ["# #", "# #", "###", "###", "# #"], // W
    ["# #", "# #", " # ", "# #", "# #"], // X
    ["# #", "# #", " # ", " # ", " # "], // Y
    ["###", "  #", " # ", "#  ", "###"], // Z
];

/// El dibujo de un carácter, o None si no hay ninguno: el espacio y lo que no
/// esté en las tablas dejan el hueco de un glifo y siguen de largo.
fn glyph(symbol: char) -> Option<&'static Glyph> {
    match symbol {
        '0'..='9' => Some(&DIGITS[symbol as usize - '0' as usize]),
        'A'..='Z' => Some(&LETTERS[symbol as usize - 'A' as usize]),
        _ => None,
    }
}

/// Ancho en pantalla que va a ocupar `text` con bloques de lado `block`.
pub fn text_width(text: &str, block: f32) -> f32 {
    let glyphs = text.chars().count() as f32;
    if glyphs == 0.0 {
        return 0.0;
    }
    (glyphs * DIGIT_COLUMNS + (glyphs - 1.0) * DIGIT_SPACING) * block
}

/// Dibuja `text` en mayúsculas con su esquina superior izquierda en `top_left`.
pub fn draw_text(
    framebuffer: &mut Framebuffer,
    text: &str,
    top_left: Vector2,
    block: f32,
    color: Color,
) {
    let mut left = top_left.x;

    for symbol in text.chars() {
        if let Some(glyph) = glyph(symbol) {
            draw_glyph(framebuffer, glyph, left, top_left.y, block, color);
        }
        left += (DIGIT_COLUMNS + DIGIT_SPACING) * block;
    }
}

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
    glyph: &Glyph,
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
