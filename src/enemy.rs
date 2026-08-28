use crate::config;
use crate::config::Level;
use crate::events::{Events, GameEvent};
use crate::health::Health;
use crate::maze::{collides, take_spawns, Maze};
use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Enemy {
    pub pos: Vector2,
    /// Carácter con el que apareció, que es como se busca su textura.
    pub kind: char,
    /// Golpes que le quedan por aguantar. En cero deja de contar para todo.
    hits_left: u32,
    /// Velocidad de persecución, que la pone el nivel.
    speed: f32,
    /// Segundos que le quedan de estar frenado por el último golpe.
    stagger: f32,
    /// Segundos que le quedan al destello del último golpe.
    flash: f32,
    /// Segundos que lleva el salto con el que ataca, o None si está en el piso.
    leap: Option<f32>,
    /// Segundos que faltan para poder volver a saltar.
    cooldown: f32,
    /// Si el salto en curso ya resolvió su contacto.
    landed: bool,
}

/// Los enemigos del nivel. Aparecen en las celdas listadas en ENEMY_TEXTURES,
/// que se vacían del laberinto al construirlos.
pub struct Enemies {
    enemies: Vec<Enemy>,
}

impl Enemies {
    pub fn spawn(maze: &mut Maze, level: &Level, block_size: usize) -> Self {
        let spawns: Vec<char> = config::ENEMY_TEXTURES.iter().map(|enemy| enemy.cell).collect();

        let enemies = take_spawns(maze, &spawns, block_size)
            .into_iter()
            .map(|(kind, pos)| Enemy {
                pos,
                kind,
                hits_left: level.enemy_hits,
                speed: level.enemy_speed,
                stagger: 0.0,
                flash: 0.0,
                leap: None,
                cooldown: 0.0,
                landed: false,
            })
            .collect();

        Enemies { enemies }
    }

    /// Los que siguen en pie: los caídos no se dibujan ni estorban.
    pub fn iter(&self) -> impl Iterator<Item = &Enemy> {
        self.enemies.iter().filter(|enemy| enemy.alive())
    }

    pub fn update(
        &mut self,
        maze: &Maze,
        target: Vector2,
        health: &mut Health,
        events: &mut Events,
        block_size: usize,
        dt: f32,
    ) {
        let touch = config::ENEMY_RADIUS + config::PLAYER_RADIUS;

        for enemy in self.enemies.iter_mut() {
            if !enemy.alive() {
                continue;
            }

            enemy.stagger = (enemy.stagger - dt).max(0.0);
            enemy.flash = (enemy.flash - dt).max(0.0);

            if enemy.stagger <= 0.0 {
                enemy.chase(maze, target, touch, block_size, dt);
            }

            // el golpe entra arriba del salto, y take_hit devuelve false
            // mientras corra la invulnerabilidad: el evento sale solo cuando
            // el enemigo llegó y el jugador estaba para recibirlo
            if enemy.attack(target, dt, events) && health.take_hit() {
                events.push(GameEvent::PlayerHurt);
            }
        }
    }

    /// Resuelve un golpe del jugador parado en `from` mirando hacia `facing`.
    /// Se lo lleva el enemigo más cercano dentro del cono, uno por golpe: el
    /// alcance corto es lo que hace que valga la pena elegir a quién pegarle.
    pub fn strike(
        &mut self,
        maze: &Maze,
        from: Vector2,
        facing: Vector2,
        events: &mut Events,
        block_size: usize,
    ) {
        let limit = (config::ATTACK_ARC / 2.0).cos();
        let mut target: Option<(usize, f32)> = None;

        for (index, enemy) in self.enemies.iter().enumerate() {
            if !enemy.alive() {
                continue;
            }

            let offset = enemy.pos - from;
            let distance = offset.length();
            if distance > config::ATTACK_RANGE + config::ENEMY_RADIUS || distance == 0.0 {
                continue;
            }
            // el coseno del ángulo entre el frente y el enemigo: comparar contra
            // el del medio arco evita sacar el ángulo con atan2
            if (offset / distance).dot(facing) < limit {
                continue;
            }

            if target.is_none_or(|(_, closest)| distance < closest) {
                target = Some((index, distance));
            }
        }

        let Some((index, _)) = target else {
            return;
        };
        let enemy = &mut self.enemies[index];

        enemy.take_hit(maze, from, block_size);
        events.push(if enemy.alive() {
            GameEvent::EnemyHit
        } else {
            GameEvent::EnemyDown
        });
    }
}

impl Enemy {
    pub fn alive(&self) -> bool {
        self.hits_left > 0
    }

    /// Cuánto destella ahora mismo, en [0, 1]. Es lo que mezcla el sprite con
    /// ENEMY_HIT_COLOR para que se note que el golpe entró.
    pub fn flash(&self) -> f32 {
        self.flash / config::ENEMY_HIT_FLASH
    }

    /// Si está en el aire atacando, que es cuando muestra la otra cara.
    pub fn attacking(&self) -> bool {
        self.leap.is_some()
    }

    /// Cuánto se despega del piso ahora mismo, en fracción de una celda: el
    /// arco del salto, y cero mientras esté parado.
    pub fn lift(&self) -> f32 {
        let Some(elapsed) = self.leap else {
            return 0.0;
        };
        let progress = (elapsed / config::ENEMY_ATTACK_DURATION).clamp(0.0, 1.0);
        // el pico cae en el instante del contacto, así que el golpe entra
        // siempre en lo más alto por más que se toque ENEMY_ATTACK_CONTACT
        let peak = config::ENEMY_ATTACK_CONTACT.clamp(0.05, 0.95);
        let rise = if progress < peak {
            progress / peak
        } else {
            (1.0 - progress) / (1.0 - peak)
        };

        // el seno redondea la punta: el salto se frena arriba en vez de rebotar
        (rise * PI / 2.0).sin() * config::ENEMY_ATTACK_LIFT
    }

    /// Avanza el salto con el que ataca y lo lanza cuando el jugador está a
    /// tiro. Devuelve true en el único cuadro en el que el golpe toca, que es
    /// el punto más alto: pegar ahí es lo que hace que el golpe se vea venir.
    fn attack(&mut self, target: Vector2, dt: f32, events: &mut Events) -> bool {
        self.cooldown = (self.cooldown - dt).max(0.0);

        if let Some(elapsed) = self.leap.as_mut() {
            *elapsed += dt;
            if *elapsed >= config::ENEMY_ATTACK_DURATION {
                self.leap = None;
                self.cooldown = config::ENEMY_ATTACK_COOLDOWN;
            }
        }

        let in_reach = (target - self.pos).length() <= config::ENEMY_ATTACK_REACH;
        if in_reach && self.leap.is_none() && self.cooldown <= 0.0 && self.stagger <= 0.0 {
            self.leap = Some(0.0);
            self.landed = false;
            events.push(GameEvent::EnemyAttack);
        }

        // contact() da el salto por resuelto aunque el jugador ya no esté a
        // tiro: correrse mientras el bicho está en el aire es lo que lo falla
        self.contact() && in_reach
    }

    fn contact(&mut self) -> bool {
        let Some(elapsed) = self.leap else {
            return false;
        };
        if self.landed || elapsed < config::ENEMY_ATTACK_DURATION * config::ENEMY_ATTACK_CONTACT {
            return false;
        }

        self.landed = true;
        true
    }

    fn take_hit(&mut self, maze: &Maze, from: Vector2, block_size: usize) {
        self.hits_left = self.hits_left.saturating_sub(1);
        self.flash = config::ENEMY_HIT_FLASH;
        self.stagger = config::ENEMY_STAGGER;
        // el golpe le corta el salto en el aire: pegar primero es la forma de
        // no comerse el que venía
        self.leap = None;
        self.cooldown = config::ENEMY_ATTACK_COOLDOWN;

        let offset = self.pos - from;
        let distance = offset.length();
        if distance == 0.0 {
            return;
        }
        self.slide(maze, offset / distance * config::ATTACK_KNOCKBACK, block_size);
    }

    fn chase(&mut self, maze: &Maze, target: Vector2, reach: f32, block_size: usize, dt: f32) {
        let to_target = target - self.pos;
        let distance = to_target.length();

        // al tocarlo se queda pegado en vez de seguir de largo y atravesarlo
        if distance <= reach {
            return;
        }

        self.slide(maze, to_target / distance * self.speed * dt, block_size);
    }

    /// Mueve el enemigo `step`, un eje por vez, para que se deslice por las
    /// paredes en vez de quedarse trabado contra una esquina.
    fn slide(&mut self, maze: &Maze, step: Vector2, block_size: usize) {
        let radius = config::ENEMY_RADIUS;

        if !collides(maze, Vector2::new(self.pos.x + step.x, self.pos.y), radius, block_size) {
            self.pos.x += step.x;
        }
        if !collides(maze, Vector2::new(self.pos.x, self.pos.y + step.y), radius, block_size) {
            self.pos.y += step.y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// El golpe tiene que caer en el punto más alto del salto: si se resolviera
    /// antes o después, el aviso que da el salto dejaría de coincidir con el
    /// instante del que hay que salirse.
    #[test]
    fn el_golpe_del_salto_cae_en_el_punto_mas_alto() {
        let mut enemy = grounded();
        let target = Vector2::new(config::ENEMY_ATTACK_REACH - 1.0, 0.0);
        let mut events = Events::new();
        let dt = 1.0 / 240.0;

        let (mut peak, mut at_contact) = (0.0f32, None);
        // dos ciclos enteros, para pasar también por la espera entre saltos
        let cycle = config::ENEMY_ATTACK_DURATION + config::ENEMY_ATTACK_COOLDOWN;

        for _ in 0..(2.0 * cycle / dt) as usize {
            let landed = enemy.attack(target, dt, &mut events);
            peak = peak.max(enemy.lift());

            if landed {
                at_contact = Some(enemy.lift());
            }
        }

        let at_contact = at_contact.expect("el enemigo nunca llegó a pegar");
        assert!(
            (at_contact - peak).abs() <= config::ENEMY_ATTACK_LIFT * 0.02,
            "el golpe cayó a {} de altura y el pico del salto fue {}",
            at_contact,
            peak
        );
    }

    fn grounded() -> Enemy {
        Enemy {
            pos: Vector2::zero(),
            kind: 'e',
            hits_left: 1,
            speed: 0.0,
            stagger: 0.0,
            flash: 0.0,
            leap: None,
            cooldown: 0.0,
            landed: false,
        }
    }
}
