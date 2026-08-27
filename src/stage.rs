//! El nivel en juego. Junta todo lo que se rearma de cero al cambiar de mapa
//! —laberinto, enemigos, objetos y jugador— para que pasar de nivel sea cargar
//! otro Stage y nada más. La dificultad no vive acá: sale de la entrada del
//! nivel en config::LEVELS, así que agregar un mapa es agregar una fila.

use crate::config;
use crate::enemy::Enemies;
use crate::items::Items;
use crate::maze::{load_maze, Maze};
use crate::player::Player;

pub struct Stage {
    pub maze: Maze,
    pub enemies: Enemies,
    pub items: Items,
    pub player: Player,
    index: usize,
    /// Puntaje de los niveles ya terminados: el del nivel en curso lo lleva
    /// Items, que es quien sabe qué se entregó.
    banked: u32,
}

impl Stage {
    /// Empieza una partida nueva en el primer nivel.
    pub fn start(block_size: usize) -> Self {
        Stage::build(0, 0, block_size)
    }

    /// Número de nivel para mostrar, empezando en 1.
    pub fn number(&self) -> u32 {
        self.index as u32 + 1
    }

    pub fn score(&self) -> u32 {
        self.banked + self.items.score()
    }

    /// Vuelve al primer nivel con el marcador en cero.
    pub fn restart(&mut self, block_size: usize) {
        *self = Stage::start(block_size);
    }

    /// Pasa al siguiente nivel guardando lo puntuado hasta acá. Devuelve false
    /// si el que se acaba de terminar era el último.
    pub fn advance(&mut self, block_size: usize) -> bool {
        if self.index + 1 >= config::LEVELS.len() {
            return false;
        }

        *self = Stage::build(self.index + 1, self.score(), block_size);
        true
    }

    fn build(index: usize, banked: u32, block_size: usize) -> Self {
        let level = level_at(index);

        // spawn vacía del laberinto las celdas de enemigos y objetos, así que
        // van antes que cualquier cosa que lea el mapa
        let mut maze = load_maze(level.maze_file);
        let enemies = Enemies::spawn(&mut maze, level, block_size);
        let items = Items::spawn(&mut maze, level, block_size);
        let player = Player::spawn(&maze, block_size);

        Stage {
            maze,
            enemies,
            items,
            player,
            index,
            banked,
        }
    }
}

/// El nivel `index`, o el último si la tabla se queda corta.
fn level_at(index: usize) -> &'static config::Level {
    config::LEVELS
        .get(index)
        .or_else(|| config::LEVELS.last())
        .expect("config::LEVELS no puede estar vacía")
}
