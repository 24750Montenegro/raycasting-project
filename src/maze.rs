use raylib::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

//vector de vectores
pub type Maze = Vec<Vec<char>>;

/// Carácter con el que se trata todo lo que queda fuera del laberinto.
pub const OUT_OF_BOUNDS: char = '#';

pub fn load_maze(filename: &str) -> Maze {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error al abrir el archivo {}: {}", filename, e);
            return vec![vec![OUT_OF_BOUNDS; 10]; 10];
        }
    };

    let mut maze: Maze = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.trim_end_matches(['\r', '\n'])
                .chars()
                .collect::<Vec<char>>()
        })
        .filter(|row: &Vec<char>| !row.is_empty())
        .collect();

    let width = maze.iter().map(|r| r.len()).max().unwrap_or(0);
    for row in maze.iter_mut() {
        row.resize(width, ' ');
    }
    maze
}

pub fn dimensions(maze: &Maze) -> (usize, usize) {
    (maze.first().map_or(0, |row| row.len()), maze.len())
}

pub fn is_solid(c: char) -> bool {
    !matches!(c, ' ' | 'p')
}

/// Devuelve el carácter de la pared que ocupa `point`, o `None` si el punto está libre.
pub fn solid_at(maze: &Maze, point: Vector2, block_size: usize) -> Option<char> {
    if point.x < 0.0 || point.y < 0.0 {
        return Some(OUT_OF_BOUNDS);
    }

    let i = point.x as usize / block_size; //columna
    let j = point.y as usize / block_size; //fila

    match maze.get(j).and_then(|row| row.get(i)) {
        Some(&cell) if is_solid(cell) => Some(cell),
        Some(_) => None,
        None => Some(OUT_OF_BOUNDS),
    }
}

/// Un círculo de radio `radius` centrado en `pos` toca alguna pared. Lo usan
/// tanto el jugador como los enemigos.
pub fn collides(maze: &Maze, pos: Vector2, radius: f32, block_size: usize) -> bool {
    [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .any(|&(sx, sy)| {
            let corner = Vector2::new(pos.x + sx * radius, pos.y + sy * radius);
            solid_at(maze, corner, block_size).is_some()
        })
}

/// Centro en el mundo de la celda (col, row).
pub fn cell_center(col: usize, row: usize, block_size: usize) -> Vector2 {
    let block = block_size as f32;
    Vector2::new(
        col as f32 * block + block / 2.0,
        row as f32 * block + block / 2.0,
    )
}

pub fn find_player_start(maze: &Maze, block_size: usize) -> Vector2 {
    for (row, cells) in maze.iter().enumerate() {
        for (col, &cell) in cells.iter().enumerate() {
            if cell == 'p' {
                return cell_center(col, row, block_size);
            }
        }
    }
    cell_center(1, 1, block_size) // posición por defecto si no se encuentra 'p'
}

/// Saca del laberinto las celdas que aparecen en `spawns` y devuelve dónde
/// estaban. Se vacían para que el raycaster no las vea nunca como pared: así el
/// bucle caliente no tiene que saber nada de enemigos.
pub fn take_spawns(maze: &mut Maze, spawns: &[char], block_size: usize) -> Vec<(char, Vector2)> {
    let mut found = Vec::new();

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            let cell = maze[row][col];
            if spawns.contains(&cell) {
                found.push((cell, cell_center(col, row, block_size)));
                maze[row][col] = ' ';
            }
        }
    }
    found
}
