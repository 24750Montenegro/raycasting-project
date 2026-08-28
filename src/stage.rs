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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::{dimensions, is_solid};
    use std::collections::{HashSet, VecDeque};

    /// Todo nivel de la tabla se tiene que poder terminar: el mapa existe,
    /// dice por dónde se empieza, y los pájaros y la entrega quedan del lado
    /// alcanzable del laberinto. Es la red que sostiene agregar mapas: un
    /// tabique de más deja un nivel sin salida y acá se ve enseguida.
    #[test]
    fn todos_los_niveles_se_pueden_jugar() {
        for (index, level) in config::LEVELS.iter().enumerate() {
            let maze = load_maze(level.maze_file);
            let (width, height) = dimensions(&maze);
            let cell = |col: usize, row: usize| maze[row][col];

            let start = find(&maze, |c| c == 'p');
            assert_eq!(
                start.len(),
                1,
                "el nivel {} ({}) tiene {} celdas de arranque",
                index + 1,
                level.maze_file,
                start.len()
            );

            // por dónde se camina: el piso y las celdas que se vacían al nacer
            // los pájaros y los bichos
            let spawns: HashSet<char> = config::ITEM_TEXTURES
                .iter()
                .map(|item| item.cell)
                .chain(config::ENEMY_TEXTURES.iter().map(|enemy| enemy.cell))
                .collect();
            let walkable = |c: char| !is_solid(c) || spawns.contains(&c);

            let mut seen: HashSet<(usize, usize)> = start.iter().copied().collect();
            let mut queue: VecDeque<(usize, usize)> = seen.iter().copied().collect();
            while let Some((col, row)) = queue.pop_front() {
                let neighbours = [
                    (col.wrapping_sub(1), row),
                    (col + 1, row),
                    (col, row.wrapping_sub(1)),
                    (col, row + 1),
                ];
                for (col, row) in neighbours {
                    if col < width && row < height && !seen.contains(&(col, row)) && walkable(cell(col, row))
                    {
                        seen.insert((col, row));
                        queue.push_back((col, row));
                    }
                }
            }

            let birds = find(&maze, |c| config::ITEM_TEXTURES.iter().any(|item| item.cell == c));
            assert!(
                !birds.is_empty(),
                "el nivel {} ({}) no tiene nada que rescatar",
                index + 1,
                level.maze_file
            );
            for bird in &birds {
                assert!(
                    seen.contains(bird),
                    "en el nivel {} ({}) no se llega al pájaro de {:?}",
                    index + 1,
                    level.maze_file,
                    bird
                );
            }

            // la meta sigue siendo pared: alcanza con poder pararse al lado
            let goals = find(&maze, |c| c == config::GOAL_CELL);
            assert!(
                goals.iter().any(|&(col, row)| {
                    [
                        (col.wrapping_sub(1), row),
                        (col + 1, row),
                        (col, row.wrapping_sub(1)),
                        (col, row + 1),
                    ]
                    .iter()
                    .any(|side| seen.contains(side))
                }),
                "en el nivel {} ({}) no hay forma de llegar a entregar",
                index + 1,
                level.maze_file
            );
        }
    }

    /// La partida tiene que recorrer los niveles de la tabla uno tras otro y
    /// recién terminarse en el último: es lo que hace que agregar una fila a
    /// LEVELS agregue un nivel jugable y no uno al que no se llega nunca.
    #[test]
    fn la_partida_recorre_todos_los_niveles() {
        let mut stage = Stage::start(config::BLOCK_SIZE);
        assert_eq!(stage.number(), 1);

        for expected in 2..=config::LEVELS.len() as u32 {
            assert!(
                stage.advance(config::BLOCK_SIZE),
                "no se pudo pasar al nivel {}",
                expected
            );
            assert_eq!(stage.number(), expected, "el contador de nivel se desfasó");
        }

        assert!(
            !stage.advance(config::BLOCK_SIZE),
            "después del último nivel la partida tiene que terminarse"
        );
    }

    /// Las celdas que cumplen `wanted`, como (columna, fila).
    fn find(maze: &Maze, wanted: impl Fn(char) -> bool) -> Vec<(usize, usize)> {
        let mut found = Vec::new();

        for (row, cells) in maze.iter().enumerate() {
            for (col, &cell) in cells.iter().enumerate() {
                if wanted(cell) {
                    found.push((col, row));
                }
            }
        }
        found
    }
}
