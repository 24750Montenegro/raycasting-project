mod map;
mod world;

pub use map::render_map;
pub use world::render_world;

use raylib::prelude::*;

//llenar celdas
pub fn cell_color(cell: char) -> Color {
    match cell {
        '+' => Color::BLUE,
        '-' => Color::YELLOW,
        '|' => Color::GRAY,
        'g' => Color::RED,
        '#' => Color::BLACK,
        _ => Color::new(18, 19, 20, 255),
    }
}
