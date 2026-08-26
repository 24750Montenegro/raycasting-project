use super::cell_color;
use crate::caster::cast_ray;
use crate::config;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use raylib::prelude::*;

/// Color acumulado de una fila de la columna junto con la cobertura que le
/// aportaron los rayos. Guardarlo así permite promediar varios rayos por columna
/// y quedarse con bordes de pared con cobertura fraccionaria.
#[derive(Clone, Copy, Default)]
struct Accumulator {
    red: f32,
    green: f32,
    blue: f32,
    coverage: f32,
}

impl Accumulator {
    fn add(&mut self, color: Color, coverage: f32) {
        self.red += color.r as f32 * coverage;
        self.green += color.g as f32 * coverage;
        self.blue += color.b as f32 * coverage;
        self.coverage += coverage;
    }

    fn resolve(&self, samples: f32) -> Option<Color> {
        if self.coverage <= 0.0 {
            return None;
        }
        let alpha = (self.coverage / samples).clamp(0.0, 1.0);

        Some(Color::new(
            (self.red / self.coverage) as u8,
            (self.green / self.coverage) as u8,
            (self.blue / self.coverage) as u8,
            (alpha * 255.0) as u8,
        ))
    }
}

pub fn render_world(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
) {
    let width = framebuffer.width as i32;
    let height = framebuffer.height as i32;
    let horizon = framebuffer.height as f32 / 2.0;
    // distancia del ojo al plano de proyección, en píxeles
    let projection = (framebuffer.width as f32 / 2.0) / (player.fov / 2.0).tan();

    framebuffer.fill_rect(0, 0, width, horizon as i32, config::SKY_COLOR);
    framebuffer.fill_rect(
        0,
        horizon as i32,
        width,
        height - horizon as i32,
        config::FLOOR_COLOR,
    );

    let samples = config::SAMPLES_PER_COLUMN.max(1);
    let mut column = vec![Accumulator::default(); height as usize];

    for x in 0..width {
        column.fill(Accumulator::default());

        for sample in 0..samples {
            let offset = (sample as f32 + 0.5) / samples as f32;
            let angle = ray_angle(player, x as f32 + offset, framebuffer.width as f32);
            let hit = cast_ray(maze, player, angle, block_size);

            // proyectar sobre el eje de la cámara corrige el ojo de pez
            let depth = (hit.distance * (angle - player.angle).cos()).max(f32::EPSILON);
            let wall_height = (block_size as f32 / depth) * projection;
            let color = cell_color(hit.impact);

            accumulate(&mut column, horizon, wall_height, |_| color);
        }

        for (y, accumulator) in column.iter().enumerate() {
            if let Some(color) = accumulator.resolve(samples as f32) {
                framebuffer.blend_pixel(x, y as i32, color);
            }
        }
    }
}

/// Reparte la pared sobre las filas que toca. Las filas de los extremos reciben
/// solo la fracción que la pared cubre, que es lo que suaviza el escalonado.
fn accumulate(
    column: &mut [Accumulator],
    horizon: f32,
    wall_height: f32,
    mut shade: impl FnMut(f32) -> Color,
) {
    let top = horizon - wall_height / 2.0;
    let bottom = horizon + wall_height / 2.0;

    let first = top.floor().max(0.0) as usize;
    let last = (bottom.ceil() as usize).min(column.len());

    for y in first..last {
        let from = (y as f32).max(top);
        let to = ((y + 1) as f32).min(bottom);
        let coverage = (to - from).clamp(0.0, 1.0);

        if coverage > 0.0 {
            // altura relativa dentro de la pared del trozo que sí cubre la fila
            let v = ((from + to) / 2.0 - top) / wall_height;
            column[y].add(shade(v), coverage);
        }
    }
}

/// Ángulo del rayo que atraviesa la columna `x` de la pantalla.
pub fn ray_angle(player: &Player, x: f32, width: f32) -> f32 {
    player.angle - player.fov / 2.0 + (x / width) * player.fov
}
