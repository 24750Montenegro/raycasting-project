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
pub const SAMPLES_PER_COLUMN: u32 = 2;

// ── Iluminación ─────────────────────────────────────────────
/// Luz mínima: ninguna superficie se pinta más oscura que esto.
pub const LIGHT_AMBIENT: f32 = 0.12;
/// Intensidad de la luz que acompaña al jugador.
pub const LIGHT_INTENSITY: f32 = 1.0;
/// Alcance de esa luz, en celdas.
pub const LIGHT_RANGE: f32 = 9.0;
/// Exponente de la caída: >1 concentra la luz cerca del jugador.
pub const LIGHT_FALLOFF: f32 = 1.7;
/// Brillo relativo de cada cara de las paredes.
pub const LIGHT_VERTICAL_FACE: f32 = 1.0;
pub const LIGHT_HORIZONTAL_FACE: f32 = 0.62;

// ── Colores base ─────────────────────────────────────────────────────────
pub const SKY_COLOR: Color = Color::new(38, 42, 58, 255);
pub const FLOOR_COLOR: Color = Color::new(24, 22, 22, 255);
