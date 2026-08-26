mod caster;
mod config;
mod enemy;
mod framebuffer;
mod health;
mod maze;
mod player;
mod render;
mod textures;

use enemy::Enemies;
use framebuffer::Framebuffer;
use health::Health;
use maze::load_maze;
use player::Player;
use raylib::prelude::*;
use render::MinimapMode;
use textures::TextureManager;

fn main() {
    let (mut window, raylib_thread) = raylib::init()
        .size(config::WINDOW_WIDTH, config::WINDOW_HEIGHT)
        .title("Raycaster")
        .build();

    window.set_target_fps(config::TARGET_FPS);
    window.disable_cursor();

    // spawn vacía del laberinto las celdas de los enemigos, así que va primero
    let mut maze = load_maze(config::MAZE_FILE);
    let mut enemies = Enemies::spawn(&mut maze, config::BLOCK_SIZE);
    let mut player = Player::spawn(&maze, config::BLOCK_SIZE);
    let mut health = Health::full();

    let textures = TextureManager::load();
    let mut framebuffer = Framebuffer::new(
        &mut window,
        &raylib_thread,
        config::WINDOW_WIDTH as u32,
        config::WINDOW_HEIGHT as u32,
    );
    let mut depth_buffer = vec![f32::INFINITY; config::WINDOW_WIDTH as usize];

    let mut minimap = MinimapMode::Compact;

    while !window.window_should_close() {
        let dt = window.get_frame_time();

        if window.is_key_pressed(KeyboardKey::KEY_M) {
            minimap = minimap.toggled();
        }

        player.update(&window, &maze, config::BLOCK_SIZE);
        health.tick(dt);
        enemies.update(&maze, player.pos, &mut health, config::BLOCK_SIZE, dt);

        if health.is_empty() {
            player = Player::spawn(&maze, config::BLOCK_SIZE);
            enemies.reset();
            health = Health::full();
        }

        render::render_world(
            &mut framebuffer,
            &maze,
            &player,
            &textures,
            config::BLOCK_SIZE,
            &mut depth_buffer,
        );
        render::render_enemies(
            &mut framebuffer,
            &enemies,
            &player,
            &textures,
            &depth_buffer,
            config::BLOCK_SIZE,
        );
        render::render_minimap(
            &mut framebuffer,
            &maze,
            &player,
            &enemies,
            minimap,
            config::BLOCK_SIZE,
        );
        render::render_health(&mut framebuffer, &health, &textures);

        framebuffer.present(&mut window, &raylib_thread);
    }
}
