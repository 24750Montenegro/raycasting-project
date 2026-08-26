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

// ── Colores base ─────────────────────────────────────────────────────────
pub const SKY_COLOR: Color = Color::new(38, 42, 58, 255);
pub const FLOOR_COLOR: Color = Color::new(24, 22, 22, 255);
