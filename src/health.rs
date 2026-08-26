use crate::config;

/// Vida del jugador, contada en medios corazones para que los medios golpes
/// sean exactos y no dependan de redondear un flotante.
pub struct Health {
    halves: u32,
    /// Segundos que faltan para poder volver a recibir daño.
    cooldown: f32,
    /// Segundos que lleva corriendo el latido del HUD, si está corriendo.
    hit_animation: f32,
}

impl Health {
    pub fn full() -> Self {
        Health {
            halves: config::HEART_SLOTS * 2,
            cooldown: 0.0,
            hit_animation: config::HEART_HIT_ANIMATION,
        }
    }

    pub fn halves(&self) -> u32 {
        self.halves
    }

    pub fn is_empty(&self) -> bool {
        self.halves == 0
    }

    pub fn tick(&mut self, dt: f32) {
        self.cooldown = (self.cooldown - dt).max(0.0);
        self.hit_animation = (self.hit_animation + dt).min(config::HEART_HIT_ANIMATION);
    }

    /// Quita medio corazón. Devuelve false si el golpe no entró porque todavía
    /// corre la invulnerabilidad del anterior.
    pub fn take_hit(&mut self) -> bool {
        if self.cooldown > 0.0 || self.halves == 0 {
            return false;
        }
        self.halves -= 1;
        self.cooldown = config::DAMAGE_COOLDOWN;
        self.hit_animation = 0.0;
        true
    }

    /// Avance del latido en [0, 1], o None si ya terminó.
    pub fn hit_progress(&self) -> Option<f32> {
        if self.hit_animation >= config::HEART_HIT_ANIMATION {
            return None;
        }
        Some(self.hit_animation / config::HEART_HIT_ANIMATION)
    }
}
