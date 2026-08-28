//! El golpe del jugador: un ciclo corto con un instante de contacto en el
//! medio. El daño no se aplica al apretar el botón sino cuando la animación
//! llega a ese instante, así lo que se ve en pantalla y lo que pasa en el mundo
//! son lo mismo, y cambiar la duración cambia las dos cosas a la vez.

use crate::config;
use crate::events::{Events, GameEvent};
use crate::gamepad;
use raylib::prelude::*;

#[derive(Default)]
pub struct Attack {
    /// Segundos que lleva el golpe en curso, o None si no hay ninguno.
    elapsed: Option<f32>,
    /// Segundos que faltan para poder tirar el siguiente.
    cooldown: f32,
    /// Si el golpe en curso ya resolvió su contacto.
    landed: bool,
}

impl Attack {
    pub fn new() -> Self {
        Attack::default()
    }

    /// Avanza el ciclo y lee la entrada. Devuelve true en el único cuadro en el
    /// que el golpe toca: quien llama resuelve ahí a quién le pegó.
    pub fn update(&mut self, rl: &RaylibHandle, dt: f32, events: &mut Events) -> bool {
        self.cooldown = (self.cooldown - dt).max(0.0);

        if let Some(elapsed) = self.elapsed.as_mut() {
            *elapsed += dt;
            if *elapsed >= config::ATTACK_DURATION {
                self.elapsed = None;
                self.cooldown = config::ATTACK_COOLDOWN;
            }
        }

        if pressed(rl) && self.elapsed.is_none() && self.cooldown <= 0.0 {
            self.elapsed = Some(0.0);
            self.landed = false;
            events.push(GameEvent::Attack);
        }

        self.contact()
    }

    /// Corta el golpe en curso, para cuando el mundo se congela.
    pub fn cancel(&mut self) {
        self.elapsed = None;
        self.cooldown = 0.0;
        self.landed = false;
    }

    /// Avance del golpe en [0, 1], o None si no hay ninguno en curso.
    pub fn progress(&self) -> Option<f32> {
        self.elapsed
            .map(|elapsed| (elapsed / config::ATTACK_DURATION).clamp(0.0, 1.0))
    }

    /// Cuadro de la hoja que toca dibujar. El 0 es la mano en reposo y el resto
    /// se reparte a lo largo del golpe, así que agregar cuadros a la hoja
    /// alarga la animación sin tocar nada más.
    pub fn frame(&self) -> usize {
        let Some(progress) = self.progress() else {
            return 0;
        };
        if config::ATTACK_FRAMES < 2 {
            return 0;
        }

        let swing = config::ATTACK_FRAMES - 1;
        1 + ((progress * swing as f32) as usize).min(swing - 1)
    }

    fn contact(&mut self) -> bool {
        let Some(elapsed) = self.elapsed else {
            return false;
        };
        if self.landed || elapsed < config::ATTACK_DURATION * config::ATTACK_CONTACT {
            return false;
        }

        self.landed = true;
        true
    }
}

fn pressed(rl: &RaylibHandle) -> bool {
    rl.is_mouse_button_pressed(config::ATTACK_MOUSE_BUTTON)
        || gamepad::button_pressed(rl, config::GAMEPAD_ATTACK_BUTTON)
}
