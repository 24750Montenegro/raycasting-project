mod framebuffer;
mod maze;
mod render;
mod player;

use framebuffer::Framebuffer;
use raylib::prelude::*;
use maze::load_maze;
use player::Player;


fn main() {
    //tamaño de la ventana
    let window_width = 1450;
    let window_height = 350;
    let block_size = 50; // tamaño de cada celda del laberinto
    //iniciar raylib
    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster")
        .build();

    let maze = load_maze("maze.txt");

    //posicion inicial del jugador
    let (px, py) = maze::find_player_start(&maze, block_size);
    let player = Player {
        pos: Vector2::new(px, py),
        angle: 0.0,
        fov: 60.0 * (std::f32::consts::PI / 180.0), // Convertir a radianes
    };
    let mut framebuffer = Framebuffer::new(
        window_width as u32, 
        window_height as u32, 
        Color::new(50, 50, 100, 255)
    );

    window.set_target_fps(60);

    while !window.window_should_close() {
        //limpiar framebuffer
        framebuffer.clear();

        //dibujo del laberinto
        render::render_maze(&mut framebuffer, &maze, block_size, &player);


        //mostrar
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
