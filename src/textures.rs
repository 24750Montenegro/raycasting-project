use crate::config;
use raylib::prelude::*;
use std::collections::HashMap;

/// Imagen en RAM lista para muestrear desde el raycaster.
struct Texture {
    width: i32,
    height: i32,
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
            width: width as i32,
            height: height as i32,
            pixels,
        })
    }

    #[inline]
    fn texel(&self, x: i32, y: i32) -> Color {
        self.pixels[(y * self.width + x) as usize]
    }

    /// Resuelve la coordenada horizontal, que es la misma para toda la columna
    /// de pantalla, y deja pendiente solo la vertical.
    fn column(&self, u: f32, tiles: f32) -> TextureColumn<'_> {
        let x = fract(u * tiles) * self.width as f32;

        if !config::TEXTURE_BILINEAR {
            let x = wrap(x as i32, self.width);
            return TextureColumn {
                texture: self,
                tiles,
                left: x,
                right: x,
                weight: 0.0,
            };
        }

        // el -0.5 pone la muestra en el centro del téxel, que es donde el peso
        // de la interpolación debe valer 1
        let x = x - 0.5;
        let floor = x.floor();

        TextureColumn {
            texture: self,
            tiles,
            left: wrap(floor as i32, self.width),
            right: wrap(floor as i32 + 1, self.width),
            weight: x - floor,
        }
    }
}

/// Franja vertical de una textura con la coordenada horizontal ya resuelta.
/// Muestrear por columna en vez de por píxel es lo que mantiene el render en
/// presupuesto: la búsqueda de la textura y el envoltorio en x se hacen una vez.
pub struct TextureColumn<'a> {
    texture: &'a Texture,
    tiles: f32,
    left: i32,
    right: i32,
    weight: f32,
}

impl TextureColumn<'_> {
    /// Devuelve los canales sin redondear: quien pinta todavía tiene que
    /// aplicarles luz y niebla, y redondear a cada paso solo pierde precisión.
    #[inline]
    pub fn at(&self, v: f32) -> [f32; 3] {
        let texture = self.texture;
        let y = fract(v * self.tiles) * texture.height as f32;

        if !config::TEXTURE_BILINEAR {
            let texel = texture.texel(self.left, wrap(y as i32, texture.height));
            return [texel.r as f32, texel.g as f32, texel.b as f32];
        }

        let y = y - 0.5;
        let floor = y.floor();
        let top = wrap(floor as i32, texture.height);
        let bottom = wrap(floor as i32 + 1, texture.height);

        let upper_left = texture.texel(self.left, top);
        let upper_right = texture.texel(self.right, top);
        let lower_left = texture.texel(self.left, bottom);
        let lower_right = texture.texel(self.right, bottom);

        let (tx, ty) = (self.weight, y - floor);
        let bilinear = |a: u8, b: u8, c: u8, d: u8| {
            let upper = a as f32 + (b as f32 - a as f32) * tx;
            let lower = c as f32 + (d as f32 - c as f32) * tx;
            upper + (lower - upper) * ty
        };

        [
            bilinear(upper_left.r, upper_right.r, lower_left.r, lower_right.r),
            bilinear(upper_left.g, upper_right.g, lower_left.g, lower_right.g),
            bilinear(upper_left.b, upper_right.b, lower_left.b, lower_right.b),
        ]
    }
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

    /// Franja de textura de la pared `cell` en la coordenada `u` de su cara. La
    /// textura se repite `TEXTURE_TILES_PER_BLOCK` veces por celda en cada eje.
    pub fn wall_column(&self, cell: char, u: f32) -> Option<TextureColumn<'_>> {
        self.walls
            .get(&cell)
            .map(|texture| texture.column(u, config::TEXTURE_TILES_PER_BLOCK))
    }
}

/// Parte fraccionaria. Las coordenadas nunca son negativas, así que basta floor.
#[inline]
fn fract(value: f32) -> f32 {
    value - value.floor()
}

/// Envuelve un índice que se salió a lo sumo un téxel del borde.
#[inline]
fn wrap(index: i32, size: i32) -> i32 {
    if index < 0 {
        index + size
    } else if index >= size {
        index - size
    } else {
        index
    }
}
