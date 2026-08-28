//! Sonido del juego: los efectos que cuelgan de los eventos y la música de
//! fondo. Cada entrada de config::SOUNDS ata un GameEvent a un archivo; el que
//! todavía no exista se saltea sin ruido, así que alcanza con dejar el archivo
//! en la ruta que dice la tabla para que ese evento empiece a sonar, sin tocar
//! código.

use crate::config;
use crate::events::GameEvent;
use raylib::error::Error;
use raylib::prelude::*;
use std::collections::HashMap;
use std::path::Path;

/// Un sonido listo para disparar, con el lugar que ocupa en la mezcla ya
/// resuelto: el nivel del clip por el volumen general.
struct Voice<'aud> {
    sound: Sound<'aud>,
    volume: f32,
}

/// Los sonidos cargados en memoria. Viven mientras viva el dispositivo de audio
/// que los creó, de ahí el préstamo.
pub struct Sounds<'aud> {
    sounds: HashMap<GameEvent, Voice<'aud>>,
}

impl<'aud> Sounds<'aud> {
    pub fn load(audio: &'aud RaylibAudio) -> Self {
        let mut sounds = HashMap::new();

        for clip in config::SOUNDS {
            // el asset todavía puede no estar: ese evento queda en silencio
            if !Path::new(clip.path).exists() {
                continue;
            }

            match load_clip(audio, clip) {
                Ok(sound) => {
                    let volume = config::SOUND_VOLUME * clip.volume;
                    sounds.insert(clip.event, Voice { sound, volume });
                }
                Err(e) => eprintln!("Error al cargar el sonido {}: {}", clip.path, e),
            }
        }

        Sounds { sounds }
    }

    /// Suena lo que corresponda al evento, si tiene algo cargado, con el
    /// volumen escalado por la intensidad con la que vino.
    pub fn play(&self, event: GameEvent, intensity: f32) {
        if let Some(voice) = self.sounds.get(&event) {
            voice
                .sound
                .set_volume((voice.volume * intensity.clamp(0.0, 1.0)).clamp(0.0, 1.0));
            voice.sound.play();
        }
    }
}

/// Abre el clip y le saca lo que sobra antes de dejarlo listo para sonar.
fn load_clip<'aud>(
    audio: &'aud RaylibAudio,
    clip: &config::SoundClip,
) -> Result<Sound<'aud>, Error> {
    let mut wave = audio.new_wave(clip.path)?;

    // los efectos bajados de internet vienen con silencio adelante —el de
    // enemy_attack son dos segundos enteros— y con cola de sobra atrás. Sin
    // recortar, el golpe se escucharía después de haber pasado; peor todavía si
    // el evento se repite antes de que el clip llegue a la parte que suena,
    // porque cada disparo lo arranca de cero y nunca se oiría nada.
    trim(&mut wave, clip.seconds);

    audio.new_sound_from_wave(&wave)
}

/// Recorta el silencio con el que arranca el clip y, si `seconds` es mayor que
/// cero, le pone ese techo de duración a lo que queda.
fn trim(wave: &mut Wave, seconds: f32) {
    // el muestreo va a mono primero: mezclado, el largo en muestras y el largo
    // en cuadros son el mismo número, que es lo que espera load_samples. Para
    // un efecto de juego la mezcla no se nota, y encima ocupa la mitad
    wave.format(wave.sample_rate() as i32, 16, 1);

    let samples = wave.load_samples();
    let samples = samples.as_ref();

    let Some(start) = samples
        .iter()
        .position(|sample| sample.abs() > config::SOUND_SILENCE)
    else {
        return; // el clip entero está por debajo del umbral: no hay qué recortar
    };

    let end = if seconds > 0.0 {
        let length = (seconds * wave.sample_rate() as f32) as usize;
        (start + length).min(samples.len())
    } else {
        samples.len()
    };

    if start > 0 || end < samples.len() {
        wave.crop(start as i32, end as i32);
    }
}

/// La música de fondo. raylib la reproduce en streaming en vez de cargarla
/// entera, así que hay que darle de comer una vez por cuadro; a cambio los dos
/// minutos y medio del tema no ocupan RAM. El bucle viene de fábrica: al llegar
/// al final vuelve a empezar sola.
pub struct Backdrop<'aud> {
    music: Music<'aud>,
}

impl<'aud> Backdrop<'aud> {
    /// Arranca el tema de fondo, o None si el archivo no está o no se pudo
    /// abrir: el juego sigue andando, nada más que sin música.
    pub fn start(audio: &'aud RaylibAudio) -> Option<Self> {
        if !Path::new(config::MUSIC_TRACK).exists() {
            return None;
        }

        match audio.new_music(config::MUSIC_TRACK) {
            Ok(music) => {
                music.set_volume(config::MUSIC_VOLUME);
                music.play_stream();
                Some(Backdrop { music })
            }
            Err(e) => {
                eprintln!("Error al cargar la música {}: {}", config::MUSIC_TRACK, e);
                None
            }
        }
    }

    /// Le repone al stream lo que consumió el cuadro. Va afuera de la
    /// simulación: la música sigue sonando también sobre los carteles.
    pub fn feed(&self) {
        self.music.update_stream();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cada clip tiene que quedar arrancando donde de verdad suena. Los que hay
    /// traen silencio adelante —el de enemy_attack, dos segundos enteros— y el
    /// efecto se dispara de nuevo antes de que el clip llegue a esa parte, así
    /// que sin el recorte no se escucharía nunca.
    #[test]
    fn los_clips_arrancan_donde_empieza_el_sonido() {
        let Some(audio) = device() else {
            return; // sin placa de sonido no hay nada que medir
        };

        for clip in config::SOUNDS {
            let mut wave = audio
                .new_wave(clip.path)
                .unwrap_or_else(|e| panic!("no se pudo abrir {}: {}", clip.path, e));
            let original = wave.frame_count();
            trim(&mut wave, clip.seconds);

            let samples = wave.load_samples();
            let first = samples.as_ref()[0].abs();
            assert!(
                first > config::SOUND_SILENCE,
                "{} sigue arrancando en silencio ({} de amplitud)",
                clip.path,
                first
            );
            assert!(
                wave.frame_count() <= original,
                "{} salió más largo de lo que entró",
                clip.path
            );

            if clip.seconds > 0.0 {
                let limit = (clip.seconds * wave.sample_rate() as f32) as u32;
                assert!(
                    wave.frame_count() <= limit,
                    "{} quedó en {} muestras y el techo era {}",
                    clip.path,
                    wave.frame_count(),
                    limit
                );
            }
        }
    }

    /// El dispositivo de audio, o None si esta máquina no tiene: los tests no
    /// pueden depender de que haya placa, igual que no depende el juego.
    fn device() -> Option<RaylibAudio> {
        match RaylibAudio::init_audio_device() {
            Ok(audio) if audio.is_audio_device_ready() => Some(audio),
            _ => None,
        }
    }
}
