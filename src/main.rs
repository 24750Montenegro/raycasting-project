mod attack;
mod audio;
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
mod stage;
mod textures;

use attack::Attack;
use audio::{Backdrop, Sounds};
use events::{Events, GameEvent};
use framebuffer::Framebuffer;
use health::Health;
use raylib::prelude::*;
use render::MinimapMode;
use stage::Stage;
use textures::{ItemFace, TextureManager};

/// En qué está la partida. Fuera de Playing el mundo queda congelado: se sigue
/// dibujando la última escena con un cartel encima, y la tecla de reinicio es
/// lo único que hace algo.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    /// La pantalla de inicio, antes de empezar.
    Title,
    /// La guía de controles, a la que se llega desde el menú.
    Controls,
    Playing,
    /// Se entregó todo lo del nivel.
    Cleared,
    /// Se acabó la vida.
    Over,
    /// Se terminó el último nivel.
    Won,
}

fn main() {
    let (mut window, raylib_thread) = raylib::init()
        .size(config::WINDOW_WIDTH, config::WINDOW_HEIGHT)
        .title(config::TITLE_NAME)
        .build();

    window.set_target_fps(config::TARGET_FPS);
    // el menú se maneja con el mouse; el juego se lo queda al empezar
    window.enable_cursor();

    let mut stage = Stage::start(config::BLOCK_SIZE);
    let mut phase = Phase::Title;
    let mut health = Health::full();
    let mut attack = Attack::new();
    let mut events = Events::new();

    // sin dispositivo de audio el juego sigue andando, mudo. init_audio_device
    // devuelve Ok aunque el backend no haya podido abrir la placa, asi que hace
    // falta preguntarle al dispositivo si de verdad quedo listo
    let audio = match RaylibAudio::init_audio_device() {
        Ok(audio) if audio.is_audio_device_ready() => Some(audio),
        Ok(_) => {
            eprintln!("Sin sonido: el dispositivo de audio no arranco");
            None
        }
        Err(e) => {
            eprintln!("Sin sonido: {}", e);
            None
        }
    };
    let sounds = audio.as_ref().map(Sounds::load);
    let backdrop = audio.as_ref().and_then(Backdrop::start);

    let mut textures = TextureManager::load();
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

        // la música va afuera de la simulación: sigue de fondo también sobre
        // los carteles, que es cuando el mundo está congelado
        if let Some(backdrop) = &backdrop {
            backdrop.feed();
        }

        if window.is_key_pressed(KeyboardKey::KEY_M)
            || gamepad::button_pressed(&window, config::GAMEPAD_MINIMAP_BUTTON)
        {
            minimap = minimap.toggled();
        }

        if let Some(menu) = menu_of(phase) {
            match menu_choice(&window, framebuffer.size(), menu) {
                Some(render::Button::Play) => {
                    phase = Phase::Playing;
                    window.disable_cursor(); // de acá en más el mouse es la mirada
                }
                Some(render::Button::Controls) => phase = Phase::Controls,
                Some(render::Button::Back) => phase = Phase::Title,
                None => {}
            }
        } else if phase == Phase::Playing {
            stage.player.update(&window, &stage.maze, config::BLOCK_SIZE);
            health.tick(dt);

            // el golpe se resuelve en el instante de contacto de la animación,
            // no al apretar el botón
            if attack.update(&window, dt, &mut events) {
                stage.enemies.strike(
                    &stage.maze,
                    stage.player.pos,
                    stage.player.direction(),
                    &mut events,
                    config::BLOCK_SIZE,
                );
            }

            stage.enemies.update(
                &stage.maze,
                stage.player.pos,
                &mut health,
                &mut events,
                config::BLOCK_SIZE,
                dt,
            );
            stage.items.update(stage.player.pos, dt, &mut events);
            textures.animate(dt);

            // la rama corre solo mientras se estaba jugando, así que cada aviso
            // sale una vez y no en cada cuadro del cartel
            if health.is_empty() {
                events.push(GameEvent::PlayerDown);
                phase = Phase::Over;
            } else if stage.items.cleared() {
                phase = Phase::Cleared;
            }
        } else if restart_pressed(&window) {
            phase = next_run(&mut stage, phase, &mut health, &mut attack);
        }

        for (event, intensity) in events.drain() {
            if let Some(sounds) = &sounds {
                sounds.play(event, intensity);
            }
        }

        render::render_world(
            &mut framebuffer,
            &stage.maze,
            &stage.player,
            &textures,
            config::BLOCK_SIZE,
            &mut depth_buffer,
        );
        render::render_sprites(
            &mut framebuffer,
            &stage.enemies,
            &stage.items,
            &stage.player,
            &textures,
            &depth_buffer,
            config::BLOCK_SIZE,
        );
        // en el menú se ve la escena y nada más: el HUD es de la partida, y
        // todavía no empezó
        if let Some(menu) = menu_of(phase) {
            let hovered = render::button_at(framebuffer.size(), menu, window.get_mouse_position());
            render::render_title(&mut framebuffer, menu, hovered);
        } else {
            render::render_weapon(&mut framebuffer, &attack, &textures);
            render::render_minimap(
                &mut framebuffer,
                &stage.maze,
                &stage.player,
                &stage.enemies,
                &stage.items,
                minimap,
                config::BLOCK_SIZE,
            );
            render::render_health(&mut framebuffer, &health, &textures);
            render::render_pickup(&mut framebuffer, &stage.items, &textures);
            render::render_score(
                &mut framebuffer,
                stage.number(),
                stage.score(),
                stage.items.carried(),
                &textures,
            );

            if let Some((art, color)) = banner(phase) {
                // los que quedaron sin rescatar solo al terminar un nivel: en
                // el game over no se perdió ninguno, se perdió la partida
                let left_behind = matches!(phase, Phase::Cleared | Phase::Won)
                    .then(|| stage.items.left_behind())
                    .filter(|&count| count > 0)
                    .zip(textures.item_face(ItemFace::Sad));

                render::render_banner(
                    &mut framebuffer,
                    art,
                    color,
                    gamepad::connected(&window),
                    left_behind,
                );
            }
        }

        framebuffer.present(&mut window, &raylib_thread);
    }
}

/// Qué pantalla del menú corresponde a esta fase, o None si ya se está jugando.
fn menu_of(phase: Phase) -> Option<render::Menu> {
    match phase {
        Phase::Title => Some(render::Menu::Main),
        Phase::Controls => Some(render::Menu::Controls),
        _ => None,
    }
}

/// Qué eligió el jugador en el menú, o None si no tocó nada. Se elige con el
/// clic sobre un botón, y también con la tecla o el control, que van siempre a
/// la opción principal de esa pantalla: empezar en el menú y volver en la guía.
fn menu_choice(window: &RaylibHandle, screen: Vector2, menu: render::Menu) -> Option<render::Button> {
    if window.is_mouse_button_pressed(config::ATTACK_MOUSE_BUTTON) {
        if let Some(button) = render::button_at(screen, menu, window.get_mouse_position()) {
            return Some(button);
        }
    }

    let shortcut = window.is_key_pressed(config::TITLE_START_KEY)
        || gamepad::button_pressed(window, config::GAMEPAD_RESTART_BUTTON);

    shortcut.then(|| match menu {
        render::Menu::Main => render::Button::Play,
        render::Menu::Controls => render::Button::Back,
    })
}

fn restart_pressed(window: &RaylibHandle) -> bool {
    window.is_key_pressed(KeyboardKey::KEY_R)
        || gamepad::button_pressed(window, config::GAMEPAD_RESTART_BUTTON)
}

/// Qué sigue después de un cartel: el nivel siguiente si se terminó este, y una
/// partida nueva desde el primero si se perdió o si ya no quedan niveles.
fn next_run(stage: &mut Stage, phase: Phase, health: &mut Health, attack: &mut Attack) -> Phase {
    let advanced = phase == Phase::Cleared && stage.advance(config::BLOCK_SIZE);

    if phase == Phase::Cleared && !advanced {
        return Phase::Won; // era el último nivel
    }
    if !advanced {
        stage.restart(config::BLOCK_SIZE);
    }

    *health = Health::full();
    attack.cancel();
    Phase::Playing
}

/// Cartel que va encima de la escena congelada, o None mientras se juega. El
/// de la pantalla de inicio no sale de acá: ese se dibuja aparte.
fn banner(phase: Phase) -> Option<(&'static [&'static str], Color)> {
    match phase {
        Phase::Title | Phase::Controls | Phase::Playing => None,
        Phase::Cleared => Some((config::LEVEL_CLEAR_ART, config::LEVEL_CLEAR_COLOR)),
        Phase::Over => Some((config::GAME_OVER_ART, config::GAME_OVER_COLOR)),
        Phase::Won => Some((config::VICTORY_ART, config::VICTORY_COLOR)),
    }
}
