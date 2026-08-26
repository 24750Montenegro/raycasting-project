//! Parámetros ajustables del motor. Todo lo que se quiera "tunear" vive aquí.

use raylib::prelude::*;
use std::f32::consts::PI;

// ── Ventana ──────────────────────────────────────────────────────────────
pub const WINDOW_WIDTH: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;
pub const TARGET_FPS: u32 = 60;

// ── Mundo ────────────────────────────────────────────────────────────────
pub const MAZE_FILE: &str = "maze.txt";
pub const BLOCK_SIZE: usize = 40;

// ── Jugador ──────────────────────────────────────────────────────────────
pub const FOV: f32 = PI / 3.0;
pub const MOVE_SPEED: f32 = 200.0;
pub const ROTATION_SPEED: f32 = PI / 1.5;
pub const MOUSE_SENSITIVITY: f32 = 0.002;
pub const PLAYER_RADIUS: f32 = 10.0;

// ── Raycaster ────────────────────────────────────────────────────────────
/// Paso del ray march, en celdas. Más chico = menos riesgo de saltarse una pared.
pub const RAY_STEP: f32 = 0.2;
/// Distancia máxima de trazado, en celdas.
pub const RAY_MAX_DEPTH: f32 = 64.0;
/// Bisecciones para afinar el impacto. Cada una parte a la mitad el error del
/// paso: con 12 el impacto queda dentro de ~0.002 px del plano de la pared.
pub const RAY_REFINE_STEPS: u32 = 12;
/// Rayos por columna de pantalla. Más de uno suaviza los bordes verticales a
/// costa de trazar esa cantidad de rayos por píxel de ancho.
pub const SAMPLES_PER_COLUMN: u32 = 1;

// ── Iluminación ──────────────────────────────────────────────────────────
/// Luz mínima: ninguna superficie se pinta más oscura que esto.
pub const LIGHT_AMBIENT: f32 = 0.16;
/// Intensidad de la luz que acompaña al jugador.
pub const LIGHT_INTENSITY: f32 = 1.0;
/// Alcance de esa luz, en celdas.
pub const LIGHT_RANGE: f32 = 12.0;
/// Exponente de la caída: >1 concentra la luz cerca del jugador.
pub const LIGHT_FALLOFF: f32 = 1.7;
/// Brillo relativo de cada cara de las paredes.
pub const LIGHT_VERTICAL_FACE: f32 = 1.0;
pub const LIGHT_HORIZONTAL_FACE: f32 = 0.62;

// ── Texturas ─────────────────────────────────────────────────────────────
/// Textura de cada carácter del laberinto. Los que no aparezcan se pintan con
/// el color plano de render::cell_color.
pub const WALL_TEXTURES: &[(char, &str)] = &[
    ('-', "assets/netherbrick.webp"),
    ('+', "assets/nether_rock.jpg"),
];
/// Cuántas veces se repite la textura dentro de una celda, en cada eje.
pub const TEXTURE_TILES_PER_BLOCK: f32 = 4.0;
/// Lado máximo al que se reescalan las texturas al cargarlas. Bajarlo también
/// acelera el render: cuanto más chica la textura, mejor cae en caché.
pub const TEXTURE_MAX_SIZE: u32 = 128;
/// Filtrado bilineal al muestrear. Apagarlo deja los téxeles duros y casi
/// duplica los cuadros por segundo: son 4 lecturas de textura por píxel.
pub const TEXTURE_BILINEAR: bool = true;

// ── Niebla ───────────────────────────────────────────────────────────────
pub const FOG_ENABLED: bool = true;
pub const FOG_COLOR: Color = Color::new(14, 13, 17, 255);
/// Celdas a partir de las cuales la niebla empieza a notarse...
pub const FOG_START: f32 = 1.5;
/// ...y celdas a partir de las cuales lo tapa todo.
pub const FOG_END: f32 = 11.0;
/// Exponente de la mezcla: >1 retrasa la niebla, <1 la adelanta.
pub const FOG_DENSITY: f32 = 1.35;

// ── Minimapa ─────────────────────────────────────────────────────────────
/// Lado de cada celda del minimapa compacto, en píxeles de pantalla.
pub const MINIMAP_COMPACT_CELL: f32 = 9.0;
/// Separación del minimapa compacto respecto de la esquina.
pub const MINIMAP_MARGIN: f32 = 16.0;
/// Fracción de la pantalla que ocupa el minimapa expandido (queda centrado).
pub const MINIMAP_EXPANDED_FILL: f32 = 0.85;
pub const MINIMAP_PADDING: f32 = 6.0;
pub const MINIMAP_BORDER_WIDTH: f32 = 2.0;
/// Opacidad de las paredes del minimapa sobre la vista 3D.
pub const MINIMAP_OPACITY: f32 = 0.85;
pub const MINIMAP_BACKGROUND: Color = Color::new(10, 10, 14, 190);
pub const MINIMAP_BORDER_COLOR: Color = Color::new(120, 122, 140, 220);
pub const MINIMAP_PLAYER_COLOR: Color = Color::new(235, 90, 70, 255);
pub const MINIMAP_RAY_COLOR: Color = Color::new(255, 226, 150, 255);
pub const MINIMAP_RAY_OPACITY: f32 = 0.30;
pub const MINIMAP_RAYS: usize = 48;

// ── Colores base ─────────────────────────────────────────────────────────
pub const SKY_COLOR: Color = Color::new(38, 42, 58, 255);
pub const FLOOR_COLOR: Color = Color::new(24, 22, 22, 255);
