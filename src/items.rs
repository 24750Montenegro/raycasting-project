//! Objetos que hay que juntar por el laberinto y llevar hasta el punto de
//! entrega ('g' en el mapa). Lo que se lleva encima no se pierde al caminar,
//! pero solo puntúa cuando se llega a la meta, así que la gracia está en
//! decidir cuándo volver.

use crate::config;
use crate::events::{Events, GameEvent};
use crate::maze::{find_cells, take_spawns, Maze};
use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemState {
    /// Tirado en el piso, esperando que lo levanten.
    Loose,
    /// Encima del jugador.
    Carried,
    /// Ya entregado en la meta.
    Delivered,
}

pub struct Item {
    pub pos: Vector2,
    /// Carácter con el que apareció, que es como se busca su textura.
    pub kind: char,
    pub state: ItemState,
    /// Desfase del flote, para que no suban y bajen todos a la vez.
    phase: f32,
}

impl Item {
    /// Cuánto se despega del piso ahora mismo, en fracción de una celda.
    pub fn lift(&self, clock: f32) -> f32 {
        let wave = (clock * config::ITEM_BOB_SPEED + self.phase).sin();
        config::ITEM_LIFT + wave * config::ITEM_BOB_AMPLITUDE
    }
}

/// Los objetos del nivel más el marcador. Aparecen en las celdas listadas en
/// ITEM_TEXTURES, que se vacían del laberinto al construirlos.
pub struct Items {
    items: Vec<Item>,
    /// Celdas 'g' donde se entrega. Se buscan una vez y no se tocan.
    goals: Vec<Vector2>,
    carried: u32,
    delivered: u32,
    score: u32,
    /// Segundos desde que empezó el nivel, para el flote de los sprites.
    clock: f32,
}

impl Items {
    pub fn spawn(maze: &mut Maze, block_size: usize) -> Self {
        let spawns: Vec<char> = config::ITEM_TEXTURES.iter().map(|&(cell, _)| cell).collect();

        let items = take_spawns(maze, &spawns, block_size)
            .into_iter()
            .enumerate()
            .map(|(index, (kind, pos))| Item {
                pos,
                kind,
                state: ItemState::Loose,
                phase: index as f32 * config::ITEM_BOB_OFFSET,
            })
            .collect();

        Items {
            items,
            goals: find_cells(maze, config::GOAL_CELL, block_size),
            carried: 0,
            delivered: 0,
            score: 0,
            clock: 0.0,
        }
    }

    /// Los que están en el piso, que son los únicos que se dibujan.
    pub fn loose(&self) -> impl Iterator<Item = &Item> {
        self.items
            .iter()
            .filter(|item| item.state == ItemState::Loose)
    }

    pub fn clock(&self) -> f32 {
        self.clock
    }

    pub fn carried(&self) -> u32 {
        self.carried
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    /// Los que todavía no llegaron a la meta, lleve el jugador o no.
    pub fn pending(&self) -> u32 {
        self.items.len() as u32 - self.delivered
    }

    pub fn all_delivered(&self) -> bool {
        !self.items.is_empty() && self.pending() == 0
    }

    /// Dónde hay que entregar, para marcarlo en el minimapa.
    pub fn goals(&self) -> &[Vector2] {
        &self.goals
    }

    pub fn update(&mut self, player_pos: Vector2, dt: f32, events: &mut Events) {
        self.clock += dt;

        self.pick_up(player_pos, events);
        self.deliver(player_pos, events);
    }

    /// Levanta lo que se pise, hasta llenar las manos.
    fn pick_up(&mut self, player_pos: Vector2, events: &mut Events) {
        let reach = config::ITEM_PICKUP_RADIUS + config::PLAYER_RADIUS;

        for item in self.items.iter_mut() {
            if self.carried >= config::ITEM_CARRY_LIMIT {
                return; // no entra nada más hasta descargar
            }
            if item.state != ItemState::Loose || (player_pos - item.pos).length() > reach {
                continue;
            }

            item.state = ItemState::Carried;
            self.carried += 1;
            events.push(GameEvent::ItemPickup);
        }
    }

    /// Deja en la meta todo lo que se lleve encima.
    fn deliver(&mut self, player_pos: Vector2, events: &mut Events) {
        if self.carried == 0 || !self.at_goal(player_pos) {
            return;
        }

        for item in self.items.iter_mut() {
            if item.state == ItemState::Carried {
                item.state = ItemState::Delivered;
            }
        }

        self.score += self.carried * config::ITEM_SCORE;
        self.delivered += self.carried;
        self.carried = 0;
        events.push(GameEvent::ItemDeliver);

        if self.all_delivered() {
            events.push(GameEvent::LevelClear);
        }
    }

    fn at_goal(&self, player_pos: Vector2) -> bool {
        self.goals
            .iter()
            .any(|&goal| (player_pos - goal).length() <= config::GOAL_REACH)
    }

    /// Vuelve a dejar el nivel como al empezar, marcador incluido.
    pub fn reset(&mut self) {
        for item in self.items.iter_mut() {
            item.state = ItemState::Loose;
        }
        self.carried = 0;
        self.delivered = 0;
        self.score = 0;
        self.clock = 0.0;
    }
}
