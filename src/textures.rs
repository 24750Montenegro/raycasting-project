use crate::config;
use image::{imageops, RgbaImage};
use raylib::prelude::*;
use std::collections::HashMap;

/// Imagen en RAM lista para muestrear desde el raycaster.
pub struct Texture {
    width: i32,
    height: i32,
    pixels: Vec<Color>,
}

impl Texture {
    fn load(path: &str, limit: u32) -> Option<Texture> {
        match image::open(path) {
            Ok(image) => Some(Texture::from_image(image.to_rgba8(), limit)),
            Err(e) => {
                eprintln!("Error al cargar la textura {}: {}", path, e);
                None
            }
        }
    }

    fn from_image(image: RgbaImage, limit: u32) -> Texture {
        // a la escala a la que se ven en pantalla, el detalle de más de una
        // imagen enorme solo aporta ruido al muestrear
        let image = if image.width().max(image.height()) > limit {
            let scale = limit as f32 / image.width().max(image.height()) as f32;
            let width = ((image.width() as f32 * scale) as u32).max(1);
            let height = ((image.height() as f32 * scale) as u32).max(1);
            imageops::resize(&image, width, height, imageops::FilterType::Triangle)
        } else {
            image
        };

        let (width, height) = image.dimensions();
        Texture {
            width: width as i32,
            height: height as i32,
            pixels: image
                .pixels()
                .map(|p| Color::new(p[0], p[1], p[2], p[3]))
                .collect(),
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    /// Muestra puntual con alfa, en coordenadas [0, 1]. Los sprites ocupan poca
    /// pantalla, asi que no necesitan el camino por columna de las paredes.
    pub fn sample(&self, u: f32, v: f32) -> Color {
        let x = ((u * self.width as f32) as i32).clamp(0, self.width - 1);
        let y = ((v * self.height as f32) as i32).clamp(0, self.height - 1);
        self.texel(x, y)
    }

    #[inline]
    fn texel(&self, x: i32, y: i32) -> Color {
        self.pixels[(y * self.width + x) as usize]
    }

    /// Resuelve la coordenada horizontal, que es la misma para toda la columna
    /// de pantalla, y deja pendiente solo la vertical.
    pub fn column(&self, u: f32, tiles: f32) -> TextureColumn<'_> {
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

/// Una textura de pared con su propio mosaico. Los cuadros de una hoja animada
/// se recorren en bucle; una imagen suelta es el caso de un solo cuadro.
struct Wall {
    frames: Vec<Texture>,
    tiles: f32,
    fps: f32,
}

impl Wall {
    /// Cuadro que toca mostrar a los `clock` segundos. Sin animación —un solo
    /// cuadro o fps en 0— siempre es el primero.
    fn frame(&self, clock: f32) -> Option<&Texture> {
        if self.frames.len() < 2 || self.fps <= 0.0 {
            return self.frames.first();
        }
        let index = (clock * self.fps) as usize % self.frames.len();
        self.frames.get(index)
    }
}

/// Todas las imagenes del juego cargadas en RAM: paredes, sprites de enemigos y
/// la hoja de corazones del HUD.
pub struct TextureManager {
    walls: HashMap<char, Wall>,
    enemies: HashMap<char, Texture>,
    items: HashMap<char, Texture>,
    hearts: Vec<Texture>,
    weapon: Vec<Texture>,
    /// Segundos corridos de las paredes animadas. Vive acá y no en el render
    /// para que dibujar el mundo siga siendo una función de solo lectura.
    clock: f32,
}

impl TextureManager {
    pub fn load() -> Self {
        let walls = config::WALL_TEXTURES
            .iter()
            .filter_map(|wall| {
                let frames = load_strip(wall.path, wall.frames, config::TEXTURE_MAX_SIZE);
                if frames.is_empty() {
                    return None; // sin imagen la pared cae al color plano
                }
                Some((
                    wall.cell,
                    Wall {
                        frames,
                        tiles: wall.tiles,
                        fps: wall.fps,
                    },
                ))
            })
            .collect();

        let enemies = config::ENEMY_TEXTURES
            .iter()
            .filter_map(|&(cell, path)| {
                let texture = Texture::load(path?, config::SPRITE_MAX_SIZE)?;
                Some((cell, texture))
            })
            .collect();

        let items = config::ITEM_TEXTURES
            .iter()
            .filter_map(|&(cell, path)| {
                let texture = Texture::load(path?, config::SPRITE_MAX_SIZE)?;
                Some((cell, texture))
            })
            .collect();

        TextureManager {
            walls,
            enemies,
            items,
            hearts: load_frames(config::HEART_SHEET, config::HEART_FRAMES),
            weapon: config::ATTACK_SHEET
                .map(|sheet| load_frames(sheet, config::ATTACK_FRAMES))
                .unwrap_or_default(),
            clock: 0.0,
        }
    }

    /// Corre el reloj de las paredes animadas. Va con el resto de la simulación
    /// y no con el dibujo, así lo que congela al mundo también las congela.
    pub fn animate(&mut self, dt: f32) {
        self.clock += dt;
    }

    /// Franja de textura de la pared `cell` en la coordenada `u` de su cara,
    /// repetida las veces que pida su entrada en WALL_TEXTURES. Si la pared es
    /// animada sale del cuadro que toca en este instante.
    pub fn wall_column(&self, cell: char, u: f32) -> Option<TextureColumn<'_>> {
        let wall = self.walls.get(&cell)?;
        Some(wall.frame(self.clock)?.column(u, wall.tiles))
    }

    /// Sprite del enemigo `kind`, o None si esa entrada no tiene textura y hay
    /// que dibujarlo con ENEMY_COLOR.
    pub fn enemy(&self, kind: char) -> Option<&Texture> {
        self.enemies.get(&kind)
    }

    /// Sprite del objeto `kind`, o None si esa entrada todavía no tiene
    /// imagen y hay que dibujarlo con ITEM_COLOR.
    pub fn item(&self, kind: char) -> Option<&Texture> {
        self.items.get(&kind)
    }

    pub fn heart(&self, frame: usize) -> Option<&Texture> {
        self.hearts.get(frame)
    }

    /// Cuadro de la animación del golpe, o None mientras ATTACK_SHEET no tenga
    /// una hoja: ahí el arma se dibuja con bloques.
    pub fn weapon(&self, frame: usize) -> Option<&Texture> {
        self.weapon.get(frame)
    }
}

/// Parte una hoja de pared en cuadros horizontales iguales. A diferencia de
/// load_frames no recorta nada: el cuadro de un video cubre su celda entera y
/// recortarlo por alfa movería la imagen de un cuadro al siguiente.
fn load_strip(path: &str, frames: usize, limit: u32) -> Vec<Texture> {
    let sheet = match image::open(path) {
        Ok(image) => image.to_rgba8(),
        Err(e) => {
            eprintln!("Error al cargar la textura {}: {}", path, e);
            return Vec::new();
        }
    };

    let frames = frames.max(1) as u32;
    let cell = sheet.width() / frames;
    if cell == 0 {
        eprintln!("La hoja {} no llega a {} cuadros de ancho", path, frames);
        return Vec::new();
    }

    (0..frames)
        .map(|frame| {
            let cut = imageops::crop_imm(&sheet, frame * cell, 0, cell, sheet.height());
            Texture::from_image(cut.to_image(), limit)
        })
        .collect()
}

/// Parte una hoja de sprites en cuadros horizontales. Los cuadros se recortan
/// todos al mismo rectangulo util —la union de lo que ocupa cada uno dentro de
/// su celda— para que la animacion no salte de un cuadro al siguiente.
fn load_frames(path: &str, frames: usize) -> Vec<Texture> {
    let sheet = match image::open(path) {
        Ok(image) => image.to_rgba8(),
        Err(e) => {
            eprintln!("Error al cargar la hoja {}: {}", path, e);
            return Vec::new();
        }
    };

    let cell = sheet.width() / frames.max(1) as u32;
    let Some(useful) = content_bounds(&sheet, cell, frames as u32) else {
        return Vec::new();
    };
    let (left, top, width, height) = useful;

    (0..frames as u32)
        .map(|frame| {
            let cut = imageops::crop_imm(&sheet, frame * cell + left, top, width, height);
            Texture::from_image(cut.to_image(), config::SPRITE_MAX_SIZE)
        })
        .collect()
}

/// Rectangulo, en coordenadas de celda, que cubre lo que hay dibujado en todos
/// los cuadros de la hoja.
fn content_bounds(sheet: &RgbaImage, cell: u32, frames: u32) -> Option<(u32, u32, u32, u32)> {
    let (mut left, mut top) = (u32::MAX, u32::MAX);
    let (mut right, mut bottom) = (0u32, 0u32);

    for (x, y, pixel) in sheet.enumerate_pixels() {
        // si el ancho no es multiplo exacto de los cuadros, lo que sobra a la
        // derecha no pertenece a ninguno
        if x >= cell * frames || pixel[3] <= config::SPRITE_ALPHA_CUTOFF {
            continue;
        }
        let local = x % cell;
        left = left.min(local);
        right = right.max(local);
        top = top.min(y);
        bottom = bottom.max(y);
    }

    if left > right || top > bottom {
        return None;
    }
    Some((left, top, right - left + 1, bottom - top + 1))
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

#[cfg(test)]
mod tests {
    use super::*;

    /// El ancho de la hoja tiene que ser múltiplo exacto de los cuadros que
    /// declara la tabla. Si no coincide, load_strip parte por donde no va y la
    /// pared se ve corrida sin que nada avise.
    #[test]
    fn las_hojas_de_pared_se_parten_en_cuadros_exactos() {
        for wall in config::WALL_TEXTURES {
            let (width, _) = image::image_dimensions(wall.path)
                .unwrap_or_else(|e| panic!("no se pudo leer {}: {}", wall.path, e));

            assert_eq!(
                width as usize % wall.frames.max(1),
                0,
                "{} mide {} de ancho y no se parte en {} cuadros",
                wall.path,
                width,
                wall.frames
            );
        }
    }

    /// Una pared animada tiene que mostrar otra imagen al pasar el tiempo de un
    /// cuadro, y volver a la primera al completar el bucle.
    #[test]
    fn la_pared_animada_avanza_y_vuelve_a_empezar() {
        let Some(animated) = config::WALL_TEXTURES
            .iter()
            .find(|wall| wall.frames > 1 && wall.fps > 0.0)
        else {
            return; // no hay ninguna pared animada configurada
        };
        let mut textures = TextureManager::load();

        let first = brightness(&textures, animated.cell);
        textures.animate(1.0 / animated.fps);
        let second = brightness(&textures, animated.cell);
        assert_ne!(
            first, second,
            "la pared {} no cambió de cuadro",
            animated.cell
        );

        // lo que queda del bucle, para caer de nuevo en el primer cuadro
        textures.animate((animated.frames - 1) as f32 / animated.fps);
        assert_eq!(
            first,
            brightness(&textures, animated.cell),
            "el bucle no cerró"
        );
    }

    /// Suma de los canales de una grilla de muestras de la pared `cell`: alcanza
    /// para distinguir un cuadro de otro sin depender de un píxel puntual.
    fn brightness(textures: &TextureManager, cell: char) -> i64 {
        let steps = 8;
        let mut total = 0i64;

        for x in 0..steps {
            let column = textures
                .wall_column(cell, (x as f32 + 0.5) / steps as f32)
                .expect("la pared no tiene textura cargada");

            for y in 0..steps {
                let texel = column.at((y as f32 + 0.5) / steps as f32);
                total += texel.iter().map(|c| *c as i64).sum::<i64>();
            }
        }
        total
    }
}
