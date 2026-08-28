//! Objetos que hay que juntar por el laberinto y llevar hasta el punto de
//! entrega ('g' en el mapa). Lo que se lleva encima no se pierde al caminar,
//! pero solo puntúa cuando se llega a la meta, así que la gracia está en
//! decidir cuándo volver.

use crate::config;
use crate::config::Level;
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
    /// Segundos que le quedan al festejo de haberlo levantado, o None si no
    /// está festejando: en el piso todavía, o hace rato que se lo llevaron.
    cheer: Option<f32>,
}

impl Item {
    /// Avance del festejo en [0, 1], o None si este no está festejando.
    fn cheer(&self) -> Option<f32> {
        self.cheer
            .map(|left| 1.0 - (left / config::ITEM_CHEER).clamp(0.0, 1.0))
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
    /// Puntos por objeto entregado, que los pone el nivel.
    value: u32,
}

impl Items {
    pub fn spawn(maze: &mut Maze, level: &Level, block_size: usize) -> Self {
        let spawns: Vec<char> = config::ITEM_TEXTURES.iter().map(|item| item.cell).collect();

        let items = take_spawns(maze, &spawns, block_size)
            .into_iter()
            .map(|(kind, pos)| Item {
                pos,
                kind,
                state: ItemState::Loose,
                cheer: None,
            })
            .collect();

        Items {
            items,
            goals: find_cells(maze, config::GOAL_CELL, block_size),
            carried: 0,
            delivered: 0,
            score: 0,
            value: level.item_score,
        }
    }

    /// Los que siguen esperando en el piso, que es lo que falta juntar.
    pub fn loose(&self) -> impl Iterator<Item = &Item> {
        self.items
            .iter()
            .filter(|item| item.state == ItemState::Loose)
    }

    /// Qué se acaba de levantar y cuánto lleva el festejo, en [0, 1], o None si
    /// no hay ninguno en curso. El que va más atrás manda: levantar otro
    /// mientras salta el anterior vuelve a empezar el festejo.
    pub fn cheer(&self) -> Option<(char, f32)> {
        self.items
            .iter()
            .filter_map(|item| Some((item.kind, item.cheer()?)))
            .min_by(|a, b| a.1.total_cmp(&b.1))
    }

    pub fn carried(&self) -> u32 {
        self.carried
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    /// Los que se quedaron sin rescatar: el nivel termina en cuanto se entrega,
    /// así que todo lo que no se llevó a la meta se pierde ahí.
    pub fn left_behind(&self) -> u32 {
        self.items.len() as u32 - self.delivered
    }

    /// Si ya se entregó, que es lo que termina el nivel. Llegar a la meta con
    /// las manos vacías no cuenta: hay que rescatar aunque sea uno.
    pub fn cleared(&self) -> bool {
        self.delivered > 0
    }

    /// Dónde hay que entregar, para marcarlo en el minimapa.
    pub fn goals(&self) -> &[Vector2] {
        &self.goals
    }

    pub fn update(&mut self, player_pos: Vector2, dt: f32, events: &mut Events) {
        for item in self.items.iter_mut() {
            item.cheer = item.cheer.map(|left| left - dt).filter(|left| *left > 0.0);
        }

        self.pick_up(player_pos, events);
        self.deliver(player_pos, events);
    }

    /// Levanta todo lo que se pise. No hay tope: lo único que decide cuándo
    /// volver a la meta es el riesgo de seguir dando vueltas, porque entregar
    /// termina el nivel y deja atrás lo que no se juntó.
    fn pick_up(&mut self, player_pos: Vector2, events: &mut Events) {
        let reach = config::ITEM_PICKUP_RADIUS + config::PLAYER_RADIUS;

        for item in self.items.iter_mut() {
            if item.state != ItemState::Loose || (player_pos - item.pos).length() > reach {
                continue;
            }

            item.state = ItemState::Carried;
            item.cheer = Some(config::ITEM_CHEER);
            self.carried += 1;
            events.push(GameEvent::ItemPickup);
        }
    }

    /// Deja en la meta todo lo que se lleve encima, y con eso da el nivel por
    /// terminado: lo que haya quedado dando vueltas por el mapa se pierde. De
    /// ahí sale la decisión de todo el nivel, porque juntarlos a todos es
    /// cruzarse con todos los bichos.
    fn deliver(&mut self, player_pos: Vector2, events: &mut Events) {
        if self.carried == 0 || !self.at_goal(player_pos) {
            return;
        }

        for item in self.items.iter_mut() {
            if item.state == ItemState::Carried {
                item.state = ItemState::Delivered;
            }
        }

        self.score += self.carried * self.value;
        self.delivered += self.carried;
        self.carried = 0;
        events.push(GameEvent::ItemDeliver);
        events.push(GameEvent::LevelClear);
    }

    fn at_goal(&self, player_pos: Vector2) -> bool {
        self.goals
            .iter()
            .any(|&goal| (player_pos - goal).length() <= config::GOAL_REACH)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Levantar algo tiene que dejar el festejo corriendo de punta a punta, y
    /// terminarlo: es lo único que avisa que entró, así que cortarlo antes de
    /// tiempo es no avisar, y no cortarlo nunca es dejarlo tapando la pantalla.
    #[test]
    fn levantar_un_objeto_larga_el_festejo_y_lo_termina() {
        let mut maze: Maze = vec![vec![' '; 4]; 4];
        maze[1][1] = config::ITEM_TEXTURES[0].cell;

        let block = config::BLOCK_SIZE;
        let mut items = Items::spawn(&mut maze, &level(), block);
        let mut events = Events::new();
        // encima del objeto, que es donde se lo levanta
        let on_top = Vector2::new(1.5 * block as f32, 1.5 * block as f32);

        items.update(on_top, 0.0, &mut events);
        assert_eq!(items.carried(), 1, "no lo levantó");
        assert_eq!(
            items.cheer().map(|(_, progress)| progress),
            Some(0.0),
            "el festejo no arrancó al levantarlo"
        );

        // justo antes de que se cumpla el tiempo todavía tiene que estar
        let dt = 1.0 / 60.0;
        for _ in 0..(config::ITEM_CHEER / dt) as usize - 1 {
            items.update(on_top, dt, &mut events);
        }
        let (_, progress) = items.cheer().expect("el festejo se cortó antes");
        assert!(progress > 0.9, "el festejo va en {} y ya casi termina", progress);

        for _ in 0..3 {
            items.update(on_top, dt, &mut events);
        }
        assert!(items.cheer().is_none(), "el festejo no terminó nunca");
    }

    /// Entregar termina el nivel aunque queden pájaros dando vueltas, y los
    /// que quedaron se cuentan: de ahí sale toda la decisión del nivel, porque
    /// volver a buscar más es volver a cruzarse con los bichos.
    #[test]
    fn entregar_termina_el_nivel_y_cuenta_los_que_quedaron() {
        let mut maze: Maze = vec![vec!['+'; 4], vec!['+'; 4], vec!['+'; 4], vec!['+'; 4]];
        let bird = config::ITEM_TEXTURES[0].cell;
        maze[1][1] = bird;
        maze[1][2] = config::GOAL_CELL;
        maze[2][1] = bird;

        let block = config::BLOCK_SIZE;
        let level = level();
        let mut items = Items::spawn(&mut maze, &level, block);
        let mut events = Events::new();

        // encima de un pájaro y al lado de la entrega: lo levanta y lo deja
        let on_item = Vector2::new(1.5 * block as f32, 1.5 * block as f32);
        items.update(on_item, 0.0, &mut events);

        assert!(items.cleared(), "entregar no terminó el nivel");
        assert_eq!(items.score(), level.item_score, "no puntuó lo entregado");
        assert_eq!(
            items.left_behind(),
            1,
            "el que se quedó en el piso tiene que contar como no rescatado"
        );
    }

    /// En la mano entran todos: no hay tope. Lo único que decide cuándo volver
    /// a la meta es el riesgo, porque entregar termina el nivel.
    #[test]
    fn se_pueden_llevar_todos_los_que_se_junten() {
        let bird = config::ITEM_TEXTURES[0].cell;
        let mut maze: Maze = vec![
            vec!['+'; 7],
            vec!['+', bird, bird, bird, bird, bird, '+'],
            vec!['+'; 7],
        ];

        let block = config::BLOCK_SIZE;
        let mut items = Items::spawn(&mut maze, &level(), block);
        let mut events = Events::new();

        // se los va pisando uno por uno, como al caminar por el pasillo
        for col in 1..=5 {
            let on_item = Vector2::new((col as f32 + 0.5) * block as f32, 1.5 * block as f32);
            items.update(on_item, 1.0 / 60.0, &mut events);
        }

        assert_eq!(items.carried(), 5, "se quedó con menos de los que pisó");
        assert!(!items.cleared(), "sin pasar por la meta el nivel sigue");
    }

    fn level() -> Level {
        Level {
            maze_file: "",
            enemy_hits: 1,
            enemy_speed: 0.0,
            item_score: 10,
        }
    }
}
