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
            })
            .collect();

        Enemies { enemies }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Enemy> {
        self.enemies.iter()
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
            enemy.chase(maze, target, reach, block_size, dt);

            // take_hit devuelve false mientras corra la invulnerabilidad, así
            // que el evento sale solo cuando el golpe entró de verdad
            if (target - enemy.pos).length() <= reach && health.take_hit() {
                events.push(GameEvent::PlayerHurt);
            }
        }
    }

    pub fn reset(&mut self) {
        for enemy in self.enemies.iter_mut() {
            enemy.pos = enemy.origin;
        }
    }
}

impl Enemy {
    fn chase(&mut self, maze: &Maze, target: Vector2, reach: f32, block_size: usize, dt: f32) {
        let to_target = target - self.pos;
        let distance = to_target.length();

        // al tocarlo se queda pegado en vez de seguir de largo y atravesarlo
        if distance <= reach {
            return;
        }

        let step = to_target / distance * config::ENEMY_SPEED * dt;
        let radius = config::ENEMY_RADIUS;

        // ejes por separado, igual que el jugador, para que se deslicen por las
        // paredes en vez de quedarse trabados contra una esquina
        if !collides(maze, Vector2::new(self.pos.x + step.x, self.pos.y), radius, block_size) {
            self.pos.x += step.x;
        }
        if !collides(maze, Vector2::new(self.pos.x, self.pos.y + step.y), radius, block_size) {
            self.pos.y += step.y;
        }
    }
}
