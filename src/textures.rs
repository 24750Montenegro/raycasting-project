use crate::config;
use raylib::prelude::*;
use std::collections::HashMap;

/// Imagen en RAM lista para muestrear desde el raycaster.
struct Texture {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl Texture {
    fn load(path: &str) -> Option<Texture> {
        let image = match image::open(path) {
            Ok(image) => image,
            Err(e) => {
                eprintln!("Error al cargar la textura {}: {}", path, e);
                return None;
            }
        };

        // a la escala a la que se ven en pantalla, el detalle de más de una
        // textura enorme solo aporta ruido al muestrear
        let limit = config::TEXTURE_MAX_SIZE;
        let image = if image.width().max(image.height()) > limit {
            image.resize(limit, limit, image::imageops::FilterType::Triangle)
        } else {
            image
        };

        let rgba = image.to_rgba8();
        let (width, height) = rgba.dimensions();
        let pixels = rgba
            .pixels()
            .map(|p| Color::new(p[0], p[1], p[2], p[3]))
            .collect();

        Some(Texture {
            width,
            height,
            pixels,
        })
    }

    #[inline]
    fn texel(&self, x: i32, y: i32) -> Color {
        let x = x.rem_euclid(self.width as i32) as u32;
        let y = y.rem_euclid(self.height as i32) as u32;
        self.pixels[(y * self.width + x) as usize]
    }

    /// Muestrea en coordenadas normalizadas; fuera de [0, 1) la textura se repite.
    fn sample(&self, u: f32, v: f32) -> Color {
        let x = u.rem_euclid(1.0) * self.width as f32;
        let y = v.rem_euclid(1.0) * self.height as f32;

        if !config::TEXTURE_BILINEAR {
            return self.texel(x as i32, y as i32);
        }

        // el -0.5 pone la muestra en el centro del téxel, que es donde el peso
        // de la interpolación debe valer 1
        let (x, y) = (x - 0.5, y - 0.5);
        let (x0, y0) = (x.floor(), y.floor());
        let (tx, ty) = (x - x0, y - y0);
        let (x0, y0) = (x0 as i32, y0 as i32);

        blend(
            blend(self.texel(x0, y0), self.texel(x0 + 1, y0), tx),
            blend(self.texel(x0, y0 + 1), self.texel(x0 + 1, y0 + 1), tx),
            ty,
        )
    }
}

fn blend(from: Color, to: Color, t: f32) -> Color {
    let lerp = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t) as u8;

    Color::new(
        lerp(from.r, to.r),
        lerp(from.g, to.g),
        lerp(from.b, to.b),
        lerp(from.a, to.a),
    )
}

/// Asocia cada carácter del laberinto con su textura, según config::WALL_TEXTURES.
pub struct TextureManager {
    walls: HashMap<char, Texture>,
}

impl TextureManager {
    pub fn load() -> Self {
        let walls = config::WALL_TEXTURES
            .iter()
            .filter_map(|&(cell, path)| Texture::load(path).map(|texture| (cell, texture)))
            .collect();

        TextureManager { walls }
    }

    /// Color de la pared `cell` en el punto (u, v) de su cara. La textura se
    /// repite `TEXTURE_TILES_PER_BLOCK` veces por celda en cada eje.
    pub fn wall(&self, cell: char, u: f32, v: f32) -> Option<Color> {
        let tiles = config::TEXTURE_TILES_PER_BLOCK;

        self.walls
            .get(&cell)
            .map(|texture| texture.sample(u * tiles, v * tiles))
    }
}
