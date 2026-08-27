mod caster;
mod config;
mod enemy;
mod events;
mod framebuffer;
mod gamepad;
mod health;
mod items;
mod maze;
mod player;
mod render;
mod textures;

use enemy::Enemies;
use events::{Events, GameEvent};
use framebuffer::Framebuffer;
use health::Health;
use items::Items;
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

    // spawn vacía del laberinto las celdas de enemigos y objetos, así que van
    // antes que cualquier cosa que lea el mapa
    let mut maze = load_maze(config::MAZE_FILE);
    let mut enemies = Enemies::spawn(&mut maze, config::BLOCK_SIZE);
    let mut items = Items::spawn(&mut maze, config::BLOCK_SIZE);
    let mut player = Player::spawn(&maze, config::BLOCK_SIZE);
    let mut health = Health::full();
    let mut events = Events::new();

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

        if window.is_key_pressed(KeyboardKey::KEY_M)
            || gamepad::button_pressed(&window, config::GAMEPAD_MINIMAP_BUTTON)
        {
            minimap = minimap.toggled();
        }

        // sin vida el mundo queda congelado: se sigue dibujando la ultima
        // escena, pero ya nada se mueve hasta reiniciar
        if health.is_empty() {
            if window.is_key_pressed(KeyboardKey::KEY_R)
                || gamepad::button_pressed(&window, config::GAMEPAD_RESTART_BUTTON)
            {
                player = Player::spawn(&maze, config::BLOCK_SIZE);
                enemies.reset();
                items.reset();
                health = Health::full();
            }
        } else {
            player.update(&window, &maze, config::BLOCK_SIZE);
            health.tick(dt);
            enemies.update(
                &maze,
                player.pos,
                &mut health,
                &mut events,
                config::BLOCK_SIZE,
                dt,
            );
            items.update(player.pos, dt, &mut events);

            // la rama corre solo mientras quedaba vida, así que el aviso de
            // caída sale una vez y no en cada cuadro del game over
            if health.is_empty() {
                events.push(GameEvent::PlayerDown);
            }
        }

        // todavía nadie reacciona a los eventos: se vacían para que la cola no
        // crezca hasta que el sonido los use
        for _event in events.drain() {}

        render::render_world(
            &mut framebuffer,
            &maze,
            &player,
            &textures,
            config::BLOCK_SIZE,
            &mut depth_buffer,
        );
        render::render_sprites(
            &mut framebuffer,
            &enemies,
            &items,
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
            &items,
            minimap,
            config::BLOCK_SIZE,
        );
        render::render_health(&mut framebuffer, &health, &textures);
        render::render_score(&mut framebuffer, &items, &textures);

        if health.is_empty() {
            render::render_game_over(&mut framebuffer, gamepad::connected(&window));
        }

        framebuffer.present(&mut window, &raylib_thread);
    }
}
