//! Lectura del control (gamepad). El mapeo de sticks y botones vive acá para
//! que el resto del juego pida "avance" o "giro" y no ejes sueltos de raylib.
//!
//! El layout es el de Xbox, que es el que raylib normaliza para casi cualquier
//! mando genérico: stick izquierdo para caminar, stick derecho para girar.

use crate::config;
use raylib::prelude::*;

/// ¿Hay un control conectado en el puerto que usamos?
pub fn connected(rl: &RaylibHandle) -> bool {
    rl.is_gamepad_available(config::GAMEPAD_ID)
}

/// Stick izquierdo como (avance, desplazamiento lateral), en -1..1.
///
/// La zona muerta es radial: se mide el módulo del vector, no cada eje por
/// separado, así el andar en diagonal no necesita empujar más el stick. Lo que
/// queda del recorrido se reescala a 0..1 para que el movimiento arranque desde
/// cero apenas se sale de la zona muerta, en vez de dar un salto.
pub fn move_axes(rl: &RaylibHandle) -> (f32, f32) {
    if !connected(rl) {
        return (0.0, 0.0);
    }

    let x = axis(rl, GamepadAxis::GAMEPAD_AXIS_LEFT_X);
    let y = axis(rl, GamepadAxis::GAMEPAD_AXIS_LEFT_Y);

    let len = (x * x + y * y).sqrt();
    let dead = config::GAMEPAD_DEADZONE;
    if len <= dead {
        return (0.0, 0.0);
    }

    let strength = ((len - dead) / (1.0 - dead)).min(1.0);
    // el eje Y del stick da negativo hacia arriba
    (-y / len * strength, x / len * strength)
}

/// Giro del stick derecho, en -1..1: negativo gira a la izquierda.
pub fn look_x(rl: &RaylibHandle) -> f32 {
    if !connected(rl) {
        return 0.0;
    }

    let x = axis(rl, GamepadAxis::GAMEPAD_AXIS_RIGHT_X);
    let dead = config::GAMEPAD_DEADZONE;
    if x.abs() <= dead {
        return 0.0;
    }

    let strength = ((x.abs() - dead) / (1.0 - dead)).min(1.0);
    // curva expo: precisión para apuntar cerca del centro, giro rápido a fondo
    let curved = strength.powf(config::GAMEPAD_LOOK_EXPO);
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let turn = curved * sign;

    if config::GAMEPAD_INVERT_LOOK { -turn } else { turn }
}

/// Botón recién apretado en este cuadro (no se repite si se mantiene).
pub fn button_pressed(rl: &RaylibHandle, button: GamepadButton) -> bool {
    connected(rl) && rl.is_gamepad_button_pressed(config::GAMEPAD_ID, button)
}

fn axis(rl: &RaylibHandle, axis: GamepadAxis) -> f32 {
    rl.get_gamepad_axis_movement(config::GAMEPAD_ID, axis)
}
