use super::cell_color;
use crate::caster::cast_ray;
use crate::config;
use crate::enemy::Enemies;
use crate::framebuffer::Framebuffer;
use crate::maze::{dimensions, is_solid, Maze};
use crate::player::Player;
use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MinimapMode {
    /// Esquina de la pantalla, sin tapar la vista.
    Compact,
    /// Centrado y lo más grande que quepa.
    Expanded,
}

impl MinimapMode {
    pub fn toggled(self) -> Self {
        match self {
            MinimapMode::Compact => MinimapMode::Expanded,
            MinimapMode::Expanded => MinimapMode::Compact,
        }
    }
}

/// Dónde cae el minimapa en pantalla y cuánto encoge al mundo para caber ahí.
struct Layout {
    origin: Vector2,
    size: Vector2,
    cell: f32,
    /// píxeles de pantalla por píxel del mundo
    scale: f32,
}

impl Layout {
    fn to_screen(&self, world: Vector2) -> Vector2 {
        self.origin + world * self.scale
    }
}

pub fn render_minimap(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    enemies: &Enemies,
    mode: MinimapMode,
    block_size: usize,
) {
    let layout = layout(framebuffer, maze, mode, block_size);

    draw_panel(framebuffer, &layout);
    draw_walls(framebuffer, maze, &layout);
    draw_rays(framebuffer, maze, player, &layout, block_size);
    draw_enemies(framebuffer, enemies, &layout);
    draw_player(framebuffer, player, &layout);
}

fn draw_enemies(framebuffer: &mut Framebuffer, enemies: &Enemies, layout: &Layout) {
    let radius = (layout.cell * 0.2).max(2.0);

    for enemy in enemies.iter() {
        framebuffer.fill_circle(layout.to_screen(enemy.pos), radius, config::ENEMY_COLOR);
    }
}

fn layout(framebuffer: &Framebuffer, maze: &Maze, mode: MinimapMode, block_size: usize) -> Layout {
    let (cols, rows) = dimensions(maze);
    let screen = Vector2::new(framebuffer.width as f32, framebuffer.height as f32);

    let cell = match mode {
        MinimapMode::Compact => config::MINIMAP_COMPACT_CELL,
        MinimapMode::Expanded => {
            let fill = config::MINIMAP_EXPANDED_FILL;
            (screen.x * fill / cols as f32).min(screen.y * fill / rows as f32)
        }
    };

    let size = Vector2::new(cols as f32 * cell, rows as f32 * cell);
    let origin = match mode {
        MinimapMode::Compact => Vector2::new(config::MINIMAP_MARGIN, config::MINIMAP_MARGIN),
        MinimapMode::Expanded => (screen - size) / 2.0,
    };

    Layout {
        origin,
        size,
        cell,
        scale: cell / block_size as f32,
    }
}

fn draw_panel(framebuffer: &mut Framebuffer, layout: &Layout) {
    let border = config::MINIMAP_BORDER_WIDTH;
    let padding = config::MINIMAP_PADDING;

    blend_box(
        framebuffer,
        layout.origin - Vector2::one() * (padding + border),
        layout.size + Vector2::one() * 2.0 * (padding + border),
        config::MINIMAP_BORDER_COLOR,
    );
    blend_box(
        framebuffer,
        layout.origin - Vector2::one() * padding,
        layout.size + Vector2::one() * 2.0 * padding,
        config::MINIMAP_BACKGROUND,
    );
}

fn draw_walls(framebuffer: &mut Framebuffer, maze: &Maze, layout: &Layout) {
    for (row, cells) in maze.iter().enumerate() {
        for (col, &cell) in cells.iter().enumerate() {
            if !is_solid(cell) {
                continue;
            }

            // los bordes se redondean a partir de las esquinas para que las
            // celdas vecinas queden pegadas aunque `cell` no sea entero
            let x = layout.origin.x + col as f32 * layout.cell;
            let y = layout.origin.y + row as f32 * layout.cell;

            framebuffer.blend_rect(
                x.round() as i32,
                y.round() as i32,
                (x + layout.cell).round() as i32 - x.round() as i32,
                (y + layout.cell).round() as i32 - y.round() as i32,
                fade(cell_color(cell), config::MINIMAP_OPACITY),
            );
        }
    }
}

fn draw_rays(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    layout: &Layout,
    block_size: usize,
) {
    let rays = config::MINIMAP_RAYS.max(1);
    let from = layout.to_screen(player.pos);

    for i in 0..rays {
        let angle =
            player.angle - player.fov / 2.0 + (i as f32 / rays as f32) * player.fov;
        let hit = cast_ray(maze, player, angle, block_size);

        framebuffer.draw_line(
            from,
            layout.to_screen(hit.hit),
            fade(config::MINIMAP_RAY_COLOR, config::MINIMAP_RAY_OPACITY),
        );
    }
}

fn draw_player(framebuffer: &mut Framebuffer, player: &Player, layout: &Layout) {
    let center = layout.to_screen(player.pos);
    let radius = (layout.cell * 0.22).max(2.0);

    framebuffer.draw_line(
        center,
        center + player.direction() * layout.cell * 1.4,
        config::MINIMAP_PLAYER_COLOR,
    );
    framebuffer.fill_circle(center, radius, config::MINIMAP_PLAYER_COLOR);
}

fn blend_box(framebuffer: &mut Framebuffer, origin: Vector2, size: Vector2, color: Color) {
    framebuffer.blend_rect(
        origin.x.round() as i32,
        origin.y.round() as i32,
        size.x.round() as i32,
        size.y.round() as i32,
        color,
    );
}

fn fade(color: Color, opacity: f32) -> Color {
    Color::new(
        color.r,
        color.g,
        color.b,
        (opacity.clamp(0.0, 1.0) * 255.0) as u8,
    )
}
