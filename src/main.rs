mod caster;
mod config;
mod framebuffer;
mod maze;
mod player;
mod render;
mod textures;

use framebuffer::Framebuffer;
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

    let maze = load_maze(config::MAZE_FILE);
    let mut player = Player::spawn(&maze, config::BLOCK_SIZE);
    let textures = TextureManager::load();

    let mut framebuffer = Framebuffer::new(
        &mut window,
        &raylib_thread,
        config::WINDOW_WIDTH as u32,
        config::WINDOW_HEIGHT as u32,
    );

    let mut minimap = MinimapMode::Compact;

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_M) {
            minimap = minimap.toggled();
        }

        player.update(&window, &maze, config::BLOCK_SIZE);

        render::render_world(&mut framebuffer, &maze, &player, &textures, config::BLOCK_SIZE);
        render::render_minimap(&mut framebuffer, &maze, &player, minimap, config::BLOCK_SIZE);
        framebuffer.present(&mut window, &raylib_thread);
    }
}
