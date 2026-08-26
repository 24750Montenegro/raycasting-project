use super::{cell_color, shading};
use crate::caster::{cast_ray, Intersect};
use crate::config;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::textures::{TextureColumn, TextureManager};
use raylib::prelude::*;
use std::ops::Range;

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
    #[inline]
    fn add(&mut self, color: [f32; 3], coverage: f32) {
        self.red += color[0] * coverage;
        self.green += color[1] * coverage;
        self.blue += color[2] * coverage;
        self.coverage += coverage;
    }

    /// Color resuelto de la fila, con la cobertura como alfa: en los bordes de
    /// la pared es fraccionaria y deja ver el fondo que ya está pintado.
    #[inline]
    fn resolve(&self, per_sample: f32) -> Option<Color> {
        if self.coverage <= 0.0 {
            return None;
        }
        let average = 1.0 / self.coverage;
        let alpha = (self.coverage * per_sample).clamp(0.0, 1.0);

        Some(Color::new(
            (self.red * average) as u8,
            (self.green * average) as u8,
            (self.blue * average) as u8,
            (alpha * 255.0) as u8,
        ))
    }
}

/// Todo lo que define el color de una columna de pared, resuelto una sola vez
/// por rayo: la textura y la luz y la niebla colapsadas en una recta por canal.
/// Por píxel solo queda muestrear y aplicar esa recta.
struct ColumnShader<'a> {
    texture: Option<TextureColumn<'a>>,
    flat: [f32; 3],
    light: f32,
    fog: [f32; 3],
}

impl<'a> ColumnShader<'a> {
    fn new(textures: &'a TextureManager, hit: &Intersect, light: f32, fog: f32) -> Self {
        let flat = cell_color(hit.impact);

        ColumnShader {
            texture: textures.wall_column(hit.impact, hit.tex_u),
            flat: [flat.r as f32, flat.g as f32, flat.b as f32],
            light: light * (1.0 - fog),
            fog: [
                config::FOG_COLOR.r as f32 * fog,
                config::FOG_COLOR.g as f32 * fog,
                config::FOG_COLOR.b as f32 * fog,
            ],
        }
    }

    #[inline]
    fn at(&self, v: f32) -> [f32; 3] {
        let base = match &self.texture {
            Some(texture) => texture.at(v),
            None => self.flat,
        };

        [
            base[0] * self.light + self.fog[0],
            base[1] * self.light + self.fog[1],
            base[2] * self.light + self.fog[2],
        ]
    }
}

pub fn render_world(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    textures: &TextureManager,
    block_size: usize,
) {
    let width = framebuffer.width as i32;
    let height = framebuffer.height as f32;
    let horizon = height / 2.0;
    // distancia del ojo al plano de proyección, en píxeles
    let projection = (framebuffer.width as f32 / 2.0) / (player.fov / 2.0).tan();

    render_background(framebuffer, horizon, projection);

    let samples = config::SAMPLES_PER_COLUMN.max(1);
    let per_sample = 1.0 / samples as f32;
    let mut column = vec![Accumulator::default(); framebuffer.height as usize];

    for x in 0..width {
        let mut touched = 0..0usize;

        for sample in 0..samples {
            let offset = (sample as f32 + 0.5) / samples as f32;
            let angle = ray_angle(player, x as f32 + offset, framebuffer.width as f32);
            let hit = cast_ray(maze, player, angle, block_size);

            // proyectar sobre el eje de la cámara corrige el ojo de pez
            let depth = (hit.distance * (angle - player.angle).cos()).max(f32::EPSILON);
            let wall_height = (block_size as f32 / depth) * projection;

            let distance = depth / block_size as f32;
            let shader = ColumnShader::new(
                textures,
                &hit,
                shading::wall_light(distance, hit.side),
                shading::fog_factor(distance),
            );

            let span = accumulate(&mut column, horizon, wall_height, |v| shader.at(v));
            touched = merge(touched, span);
        }

        for y in touched {
            if let Some(color) = column[y].resolve(per_sample) {
                framebuffer.blend_pixel(x, y as i32, color);
            }
            column[y] = Accumulator::default();
        }
    }
}

/// Cielo y suelo fila por fila: cada fila del plano horizontal cae a una
/// distancia fija del jugador, asi la niebla del fondo empalma con la de las
/// paredes en vez de cortarse contra ellas.
fn render_background(framebuffer: &mut Framebuffer, horizon: f32, projection: f32) {
    let width = framebuffer.width as i32;

    for y in 0..framebuffer.height as i32 {
        let to_horizon = (y as f32 + 0.5 - horizon).abs().max(0.5);
        // el ojo va a media celda del suelo y del techo, de ahi el 0.5
        let distance = 0.5 * projection / to_horizon;
        let base = if (y as f32) < horizon {
            config::SKY_COLOR
        } else {
            config::FLOOR_COLOR
        };

        framebuffer.fill_rect(0, y, width, 1, shading::apply_fog(base, distance));
    }
}

/// Reparte la pared sobre las filas que toca y devuelve cuáles fueron. Las filas
/// de los extremos reciben solo la fracción que la pared cubre, que es lo que
/// suaviza el escalonado.
fn accumulate(
    column: &mut [Accumulator],
    horizon: f32,
    wall_height: f32,
    mut shade: impl FnMut(f32) -> [f32; 3],
) -> Range<usize> {
    let top = horizon - wall_height / 2.0;
    let bottom = horizon + wall_height / 2.0;

    let first = top.floor().max(0.0) as usize;
    let last = (bottom.ceil().max(0.0) as usize).min(column.len());
    if last <= first {
        return 0..0;
    }
    let inv_height = 1.0 / wall_height;

    for y in first..last {
        let from = (y as f32).max(top);
        let to = ((y + 1) as f32).min(bottom);
        let coverage = (to - from).clamp(0.0, 1.0);

        if coverage > 0.0 {
            // altura relativa dentro de la pared del trozo que sí cubre la fila
            let v = ((from + to) * 0.5 - top) * inv_height;
            column[y].add(shade(v), coverage);
        }
    }
    first..last
}

fn merge(a: Range<usize>, b: Range<usize>) -> Range<usize> {
    if a.is_empty() {
        return b;
    }
    if b.is_empty() {
        return a;
    }
    a.start.min(b.start)..a.end.max(b.end)
}

/// Ángulo del rayo que atraviesa la columna `x` de la pantalla.
pub fn ray_angle(player: &Player, x: f32, width: f32) -> f32 {
    player.angle - player.fov / 2.0 + (x / width) * player.fov
}
