mod game_over;
mod hud;
mod minimap;
mod shading;
mod sprites;
mod world;

pub use game_over::render_game_over;
pub use hud::render_health;
pub use minimap::{render_minimap, MinimapMode};
pub use sprites::render_enemies;
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
