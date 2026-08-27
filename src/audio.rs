//! Sonido de los eventos del juego. Cada entrada de config::SOUNDS ata un
//! GameEvent a un archivo; el que todavía no exista se saltea sin ruido, así
//! que alcanza con dejar el archivo en la ruta que dice la tabla para que ese
//! evento empiece a sonar, sin tocar código.

use crate::config;
use crate::events::GameEvent;
use raylib::prelude::*;
use std::collections::HashMap;
use std::path::Path;

/// Los sonidos cargados en memoria. Viven mientras viva el dispositivo de audio
/// que los creó, de ahí el préstamo.
pub struct Sounds<'aud> {
    sounds: HashMap<GameEvent, Sound<'aud>>,
}

impl<'aud> Sounds<'aud> {
    pub fn load(audio: &'aud RaylibAudio) -> Self {
        let mut sounds = HashMap::new();

        for &(event, path) in config::SOUNDS {
            // el asset todavía puede no estar: ese evento queda en silencio
            if !Path::new(path).exists() {
                continue;
            }

            match audio.new_sound(path) {
                Ok(sound) => {
                    sound.set_volume(config::SOUND_VOLUME);
                    sounds.insert(event, sound);
                }
                Err(e) => eprintln!("Error al cargar el sonido {}: {}", path, e),
            }
        }

        Sounds { sounds }
    }

    /// Suena lo que corresponda al evento, si tiene algo cargado.
    pub fn play(&self, event: GameEvent) {
        if let Some(sound) = self.sounds.get(&event) {
            sound.play();
        }
    }
}
