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

pub fn is_wall(maze: &Maze, x: f32, y: f32, block_size: usize) -> bool {
    solid_at(maze, Vector2::new(x, y), block_size).is_some()
}

pub fn find_player_start(maze: &Maze, block_size: usize) -> Vector2 {
    let block = block_size as f32;

    for (j, row) in maze.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            if cell == 'p' {
                return Vector2::new(
                    i as f32 * block + block / 2.0,
                    j as f32 * block + block / 2.0,
                );
            }
        }
    }
    Vector2::new(block * 1.5, block * 1.5) // posición por defecto si no se encuentra 'p'
}
