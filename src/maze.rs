use std::fs::File;
use std::io::{BufRead, BufReader};

//vector de vectores
pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str) -> Maze {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error al abrir el archivo {}: {}", filename, e);
            return vec![vec!['#';10];10]; // Retorna un laberinto vacío de 10x10 en caso de error
        }
    };

    let mut maze: Maze = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|line| line.trim_end_matches(['\r', '\n']).chars().collect::<Vec<char>>())
        .filter(|row: &Vec<char>| !row.is_empty())
        .collect();

    let width = maze.iter().map(|r| r.len()).max().unwrap_or(0);
    for row in maze.iter_mut() {
        row.resize(width, ' ');
    }
    maze
}

pub fn is_solid(c: char) -> bool {
    !matches!(c, ' ' | 'p')
}

pub fn is_wall(maze: &Maze, x: f32, y: f32, block_size: usize) -> bool {
    if x < 0.0 || y < 0.0 {
        return true;
    }

    let i = x as usize /block_size; //columna
    let j = y as usize /block_size; //fila

    match maze.get(j).and_then(|row| row.get(i)) {
        Some(&cell) => is_solid(cell),
        None => true, // fuera de los límites del laberinto
    }
}

pub fn find_player_start(maze: &Maze, block_size: usize) -> (f32, f32) {
    for (j, row) in maze.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            if cell == 'p' {
                let x = i as f32 * block_size as f32 + block_size as f32 / 2.0;
                let y = j as f32 * block_size as f32 + block_size as f32 / 2.0;
                return (x, y);
            }
        }
    }
    (block_size as f32 * 1.5, block_size as f32 * 1.5) // posición por defecto si no se encuentra 'p'
}