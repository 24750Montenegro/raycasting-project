mod framebuffer;
mod maze;
mod render;
mod player;
mod caster;

use framebuffer::Framebuffer;
use raylib::prelude::*;
use maze::{load_maze, find_player_start};
use player::Player;
use std::f32::consts::PI;


fn main() {
    //tamaño de la ventana
    let window_width = 1450;
    let window_height = 800;
    let block_size = 50; // tamaño de cada celda del laberinto
    //iniciar raylib
    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster")
        .build();

    let maze = load_maze("maze.txt");

    //posicion inicial del jugador
    let (px, py) = find_player_start(&maze, block_size);
    let mut player = Player {
        pos: Vector2::new(px, py),
        angle: PI/4.0,
        fov: PI/4.0,
    };


    let mut framebuffer = Framebuffer::new(
        window_width as u32, 
        window_height as u32, 
        Color::new(50, 50, 100, 255)
    );

    window.set_target_fps(60);
    window.disable_cursor();

    let mut mode = "3D";


    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

         //movimiento del jugador
        player::process_events(&mut player, &window, &maze, block_size);

        //limpiar framebuffer
        framebuffer.clear();


        //dibujo del laberinto
        if mode == "2D" {
            render::render_maze(&mut framebuffer, &maze, block_size, &player);
        } else {
            render::render_world(&mut framebuffer, &maze, block_size, &player);
        }


        //mostrar
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
