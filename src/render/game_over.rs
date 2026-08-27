use crate::config;
use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

/// Cartel de game over sobre la escena congelada. El arte llega desde config
/// como líneas de texto: cada carácter distinto de espacio se pinta como un
/// bloque, así que se puede reescribir el dibujo sin tocar esto.
///
/// `gamepad` dice si hay un control conectado: la leyenda nombra el botón o la
/// tecla según con qué se esté jugando.
pub fn render_game_over(framebuffer: &mut Framebuffer, gamepad: bool) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    framebuffer.blend_rect(0, 0, width as i32, height as i32, config::GAME_OVER_VEIL);

    let hint_art = if gamepad {
        config::GAME_OVER_HINT_GAMEPAD
    } else {
        config::GAME_OVER_HINT
    };

    let title = measure(config::GAME_OVER_ART, width, config::GAME_OVER_FILL);
    let hint = measure(hint_art, width, config::GAME_OVER_HINT_FILL);

    // los dos bloques se centran juntos, no cada uno por su lado
    let gap = height * config::GAME_OVER_GAP;
    let total = title.height() + gap + hint.height();
    let mut top = (height - total) / 2.0;

    draw(framebuffer, config::GAME_OVER_ART, &title, width, top, config::GAME_OVER_COLOR);
    top += title.height() + gap;
    draw(framebuffer, hint_art, &hint, width, top, config::GAME_OVER_HINT_COLOR);
}

/// Tamaño de bloque con el que un arte ocupa `fill` del ancho de la pantalla.
struct Layout {
    block: f32,
    columns: f32,
    rows: f32,
}

impl Layout {
    fn width(&self) -> f32 {
        self.columns * self.block
    }

    fn height(&self) -> f32 {
        self.rows * self.block
    }
}

fn measure(art: &[&str], screen_width: f32, fill: f32) -> Layout {
    let columns = art.iter().map(|line| line.chars().count()).max().unwrap_or(0) as f32;
    let rows = art.len() as f32;

    Layout {
        block: if columns > 0.0 {
            (screen_width * fill / columns).max(1.0)
        } else {
            1.0
        },
        columns,
        rows,
    }
}

fn draw(
    framebuffer: &mut Framebuffer,
    art: &[&str],
    layout: &Layout,
    screen_width: f32,
    top: f32,
    color: Color,
) {
    let left = (screen_width - layout.width()) / 2.0;

    for (row, line) in art.iter().enumerate() {
        for (column, symbol) in line.chars().enumerate() {
            if symbol == ' ' {
                continue;
            }

            // se redondea desde las esquinas para que los bloques vecinos queden
            // pegados aunque el lado no sea entero
            let x = left + column as f32 * layout.block;
            let y = top + row as f32 * layout.block;

            framebuffer.fill_rect(
                x.round() as i32,
                y.round() as i32,
                (x + layout.block).round() as i32 - x.round() as i32,
                (y + layout.block).round() as i32 - y.round() as i32,
                color,
            );
        }
    }
}
