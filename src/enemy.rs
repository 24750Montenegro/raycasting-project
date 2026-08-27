use crate::config;
use crate::events::{Events, GameEvent};
use crate::health::Health;
use crate::maze::{collides, take_spawns, Maze};
use raylib::prelude::*;

pub struct Enemy {
    pub pos: Vector2,
    /// Carácter con el que apareció, que es como se busca su textura.
    pub kind: char,
    origin: Vector2,
    /// Golpes recibidos. Al llegar a ENEMY_HITS deja de contar para todo.
    hits: u32,
    /// Segundos que le quedan de estar frenado por el último golpe.
    stagger: f32,
    /// Segundos que le quedan al destello del último golpe.
    flash: f32,
}

/// Los enemigos del nivel. Aparecen en las celdas listadas en ENEMY_TEXTURES,
/// que se vacían del laberinto al construirlos.
pub struct Enemies {
    enemies: Vec<Enemy>,
}

impl Enemies {
    pub fn spawn(maze: &mut Maze, block_size: usize) -> Self {
        let spawns: Vec<char> = config::ENEMY_TEXTURES.iter().map(|&(cell, _)| cell).collect();

        let enemies = take_spawns(maze, &spawns, block_size)
            .into_iter()
            .map(|(kind, pos)| Enemy {
                pos,
                kind,
                origin: pos,
                hits: 0,
                stagger: 0.0,
                flash: 0.0,
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
        let reach = config::ENEMY_RADIUS + config::PLAYER_RADIUS;

        for enemy in self.enemies.iter_mut() {
            if !enemy.alive() {
                continue;
            }

            enemy.stagger = (enemy.stagger - dt).max(0.0);
            enemy.flash = (enemy.flash - dt).max(0.0);

            if enemy.stagger <= 0.0 {
                enemy.chase(maze, target, reach, block_size, dt);
            }

            // take_hit devuelve false mientras corra la invulnerabilidad, así
            // que el evento sale solo cuando el golpe entró de verdad
            if (target - enemy.pos).length() <= reach && health.take_hit() {
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

    pub fn reset(&mut self) {
        for enemy in self.enemies.iter_mut() {
            enemy.pos = enemy.origin;
            enemy.hits = 0;
            enemy.stagger = 0.0;
            enemy.flash = 0.0;
        }
    }
}

impl Enemy {
    pub fn alive(&self) -> bool {
        self.hits < config::ENEMY_HITS
    }

    /// Cuánto destella ahora mismo, en [0, 1]. Es lo que mezcla el sprite con
    /// ENEMY_HIT_COLOR para que se note que el golpe entró.
    pub fn flash(&self) -> f32 {
        self.flash / config::ENEMY_HIT_FLASH
    }

    fn take_hit(&mut self, maze: &Maze, from: Vector2, block_size: usize) {
        self.hits += 1;
        self.flash = config::ENEMY_HIT_FLASH;
        self.stagger = config::ENEMY_STAGGER;

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

        self.slide(maze, to_target / distance * config::ENEMY_SPEED * dt, block_size);
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
