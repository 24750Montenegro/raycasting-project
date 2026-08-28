//! Parámetros ajustables del motor. Todo lo que se quiera "tunear" vive aquí.

use crate::events::GameEvent;
use raylib::prelude::*;
use std::f32::consts::PI;

// ── Ventana ──────────────────────────────────────────────────────────────
pub const WINDOW_WIDTH: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;
pub const TARGET_FPS: u32 = 60;

// ── Niveles ──────────────────────────────────────────────────────────────
/// Un nivel: qué mapa se juega y con qué dificultad. Todo lo que sube de un
/// nivel al siguiente va acá, así agregar un mapa es agregar una fila.
pub struct Level {
    pub maze_file: &'static str,
    /// Golpes que aguanta cada enemigo antes de caer.
    pub enemy_hits: u32,
    /// Velocidad con la que persiguen, en píxeles del mundo por segundo.
    pub enemy_speed: f32,
    /// Puntos que da cada objeto entregado en la meta.
    pub item_score: u32,
}

/// Los niveles en orden. Se juegan uno tras otro y el puntaje se arrastra de
/// uno al siguiente; agregar un mapa nuevo es agregar una fila acá.
pub const LEVELS: &[Level] = &[Level {
    maze_file: "maze.txt",
    enemy_hits: 5,
    enemy_speed: 85.0,
    item_score: 100,
}];

// ── Mundo ────────────────────────────────────────────────────────────────
pub const BLOCK_SIZE: usize = 40;

// ── Jugador ──────────────────────────────────────────────────────────────
pub const FOV: f32 = PI / 3.0;
pub const MOVE_SPEED: f32 = 200.0;
pub const ROTATION_SPEED: f32 = PI / 1.5;
pub const MOUSE_SENSITIVITY: f32 = 0.002;
pub const PLAYER_RADIUS: f32 = 10.0;

// ── Control (gamepad) ────────────────────────────────────────────────────
/// Puerto del control. 0 es el primero que se conecta.
pub const GAMEPAD_ID: i32 = 0;
/// Zona muerta de los sticks, en fracción del recorrido. Sube si el personaje
/// se mueve solo con el stick en reposo.
pub const GAMEPAD_DEADZONE: f32 = 0.18;
/// Giro del stick derecho a fondo, en radianes por segundo.
pub const GAMEPAD_LOOK_SPEED: f32 = PI;
/// Exponente de la respuesta del stick derecho: >1 achica el giro cerca del
/// centro y deja la velocidad máxima solo al final del recorrido.
pub const GAMEPAD_LOOK_EXPO: f32 = 2.0;
/// Invertir el eje horizontal de la cámara.
pub const GAMEPAD_INVERT_LOOK: bool = false;
/// Botones del control: abrir el minimapa y reiniciar tras el game over.
pub const GAMEPAD_MINIMAP_BUTTON: GamepadButton = GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP;
pub const GAMEPAD_RESTART_BUTTON: GamepadButton = GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN;

// ── Raycaster ────────────────────────────────────────────────────────────
/// Paso del ray march, en celdas. Más chico = menos riesgo de saltarse una pared.
pub const RAY_STEP: f32 = 0.2;
/// Distancia máxima de trazado, en celdas.
pub const RAY_MAX_DEPTH: f32 = 64.0;
/// Bisecciones para afinar el impacto. Cada una parte a la mitad el error del
/// paso: con 12 el impacto queda dentro de ~0.002 px del plano de la pared.
pub const RAY_REFINE_STEPS: u32 = 12;
/// Rayos por columna de pantalla. Más de uno suaviza los bordes verticales a
/// costa de trazar esa cantidad de rayos por píxel de ancho.
pub const SAMPLES_PER_COLUMN: u32 = 1;

// ── Iluminación ──────────────────────────────────────────────────────────
/// Luz mínima: ninguna superficie se pinta más oscura que esto.
pub const LIGHT_AMBIENT: f32 = 0.16;
/// Intensidad de la luz que acompaña al jugador.
pub const LIGHT_INTENSITY: f32 = 1.0;
/// Alcance de esa luz, en celdas.
pub const LIGHT_RANGE: f32 = 12.0;
/// Exponente de la caída: >1 concentra la luz cerca del jugador.
pub const LIGHT_FALLOFF: f32 = 1.7;
/// Brillo relativo de cada cara de las paredes.
pub const LIGHT_VERTICAL_FACE: f32 = 1.0;
pub const LIGHT_HORIZONTAL_FACE: f32 = 0.62;

// ── Texturas ─────────────────────────────────────────────────────────────
/// La textura de un carácter del laberinto. Los caracteres que no tengan
/// entrada se pintan con el color plano de render::cell_color.
pub struct WallTexture {
    pub cell: char,
    /// Una imagen suelta, o la hoja con los cuadros de la animación uno al lado
    /// del otro cuando `frames` es mayor que 1. Un video se convierte en esa
    /// hoja con tools/video_to_sheet.py: el render muestrea texturas que ya
    /// están en RAM, así que no hay nada que decodificar en tiempo real.
    pub path: &'static str,
    /// Cuántas veces se repite dentro de una celda en cada eje: 1.0 deja la
    /// imagen entera cubriendo la cara, 4.0 la mosaica 4x4.
    pub tiles: f32,
    /// Cuántos cuadros hay en la hoja. 1 es una textura quieta.
    pub frames: usize,
    /// A cuántos cuadros por segundo se recorre la hoja, siempre en bucle. En 0
    /// se queda en el primero.
    pub fps: f32,
}

pub const WALL_TEXTURES: &[WallTexture] = &[
    WallTexture {
        cell: '-',
        path: "assets/netherbrick.webp",
        tiles: 4.0,
        frames: 1,
        fps: 0.0,
    },
    WallTexture {
        cell: '+',
        path: "assets/obsidian.jpg",
        tiles: 4.0,
        frames: 1,
        fps: 0.0,
    },
    // el clip original va a 30 fps, pero son tres cuadros: a esa velocidad el
    // bucle entero dura 0.1 s y la pared parpadea en vez de animarse
    WallTexture {
        cell: '#',
        path: "assets/dog-knife.png",
        tiles: 1.0,
        frames: 3,
        fps: 16.0,
    },

    WallTexture {
        cell: '|',
        path: "assets/netherbrick.jpg",
        tiles: 4.0,
        frames: 1,
        fps: 0.0,
    }
];
/// Lado máximo al que se reescalan las texturas al cargarlas. Bajarlo también
/// acelera el render: cuanto más chica la textura, mejor cae en caché.
pub const TEXTURE_MAX_SIZE: u32 = 128;
/// Filtrado bilineal al muestrear. Apagarlo deja los téxeles duros y casi
/// duplica los cuadros por segundo: son 4 lecturas de textura por píxel.
pub const TEXTURE_BILINEAR: bool = true;

// ── Enemigos ─────────────────────────────────────────────────────────────
/// Las dos caras de un enemigo: con cuál camina y con cuál salta encima. Las dos
/// se recortan al mismo rectángulo útil al cargarlas, así que cambiar de una a
/// la otra no lo mueve ni un píxel.
pub struct EnemyTexture {
    /// Carácter en el laberinto. Igual que la del jugador, esta celda es piso:
    /// el enemigo aparece ahí y la celda queda libre.
    pub cell: char,
    /// Cómo se ve mientras persigue. Sin imagen (None) se dibuja con ENEMY_COLOR.
    pub idle: Option<&'static str>,
    /// Cómo se ve durante el salto del ataque. Sin esta salta con la de siempre.
    pub attack: Option<&'static str>,
}

pub const ENEMY_TEXTURES: &[EnemyTexture] = &[EnemyTexture {
    cell: 'e',
    idle: Some("assets/duo.png"),
    attack: Some("assets/duo2.png"),
}];
pub const ENEMY_COLOR: Color = Color::new(196, 38, 38, 255);
/// Radio del enemigo, para chocar con las paredes y con el jugador. Tiene que
/// ser lo bastante chico como para poder cruzarse con el jugador dentro de un
/// pasillo: hace falta que 2 * (ENEMY_RADIUS + PLAYER_RADIUS) < BLOCK_SIZE.
pub const ENEMY_RADIUS: f32 = 6.5;
/// Segundos que el enemigo queda frenado después de cada golpe, que es la
/// ventana para sacárselo de encima.
pub const ENEMY_STAGGER: f32 = 0.22;
/// Destello del sprite al recibir un golpe: cuánto dura y hacia qué color se
/// mezcla en el pico.
pub const ENEMY_HIT_FLASH: f32 = 0.14;
pub const ENEMY_HIT_COLOR: Color = Color::new(255, 240, 232, 255);
/// Alto del sprite como fracción de una celda. El ojo del jugador está a media
/// celda del piso, así que por debajo de 0.5 el enemigo le queda por abajo de la
/// línea de la vista: es de ahí que sale la gracia del salto.
pub const ENEMY_SIZE: f32 = 0.4;
/// ...y ancho como fracción de ese alto: el bicho es más ancho que alto.
pub const ENEMY_ASPECT: f32 = 1.07;
/// Lado máximo al que se reescalan los sprites y las hojas al cargarlos.
pub const SPRITE_MAX_SIZE: u32 = 128;
/// Alfa por debajo del cual un píxel de sprite se considera vacío.
pub const SPRITE_ALPHA_CUTOFF: u8 = 8;
/// Distancia mínima a la cámara para dibujar un sprite, en píxeles del mundo.
/// Más cerca que esto el tamaño se dispara y no queda nada útil en pantalla.
pub const SPRITE_NEAR_PLANE: f32 = 6.0;

// ── Aviso del enemigo ────────────────────────────────────────────────────
// El enemigo se hace oír mientras persigue, y cuanto más cerca está más seguido
// y más fuerte: es lo que avisa que viene uno cuando todavía no se lo ve, y lo
// que deja saber de qué lado está sin mirar el minimapa.
/// Desde qué distancia empieza a escucharse, en píxeles del mundo.
pub const ENEMY_ALERT_RANGE: f32 = 7.0 * BLOCK_SIZE as f32;
/// Cada cuánto se hace oír, en segundos, en el borde de ese alcance y encima
/// del jugador. Entre esos dos extremos va corriendo con la distancia.
pub const ENEMY_ALERT_INTERVAL_FAR: f32 = 2.8;
pub const ENEMY_ALERT_INTERVAL_NEAR: f32 = 0.8;
/// Y con qué volumen, de 0 a 1, en esos mismos dos extremos.
pub const ENEMY_ALERT_VOLUME_FAR: f32 = 0.22;
pub const ENEMY_ALERT_VOLUME_NEAR: f32 = 1.0;

// ── Ataque del enemigo ───────────────────────────────────────────────────
// El enemigo no lastima al rozar: pega saltando encima. El salto es a la vez el
// aviso de que el golpe viene y lo que lo sube hasta la línea de la cámara, que
// parado le queda por debajo.
/// Distancia desde su centro a la que salta, y a la que llega el golpe.
pub const ENEMY_ATTACK_REACH: f32 = 34.0;
/// Cuánto dura el salto entero y en qué fracción de esa duración toca. El
/// contacto es también el punto más alto, así que correrlo corre el pico.
pub const ENEMY_ATTACK_DURATION: f32 = 0.5;
pub const ENEMY_ATTACK_CONTACT: f32 = 0.45;
/// Espera entre el final de un salto y el siguiente.
pub const ENEMY_ATTACK_COOLDOWN: f32 = 0.6;
/// Cuánto se despega del piso en el pico, en fracción de una celda. Sumado a
/// ENEMY_SIZE tiene que pasar de 0.5 para que el salto entre en la cámara.
pub const ENEMY_ATTACK_LIFT: f32 = 0.34;
// las dos mitades del efecto, atadas de una vez: parado tiene que quedar por
// debajo de la línea de la vista —el ojo está a media celda del piso— y el
// salto tiene que pasarla, o el golpe llega desde fuera de la pantalla
const _: () = assert!(
    ENEMY_SIZE < 0.5 && ENEMY_SIZE + ENEMY_ATTACK_LIFT > 0.5,
    "el enemigo tiene que ser más bajo que el ojo del jugador y su salto pasarlo"
);

// ── Sonido ───────────────────────────────────────────────────────────────
/// Un efecto atado a un evento. El archivo que todavía no exista se saltea,
/// así que la tabla ya puede nombrar los sonidos que faltan: apenas aparezca
/// en esa ruta empieza a sonar solo. Un mismo evento admite una sola entrada;
/// el formato lo pone raylib (wav, ogg, mp3, flac).
pub struct SoundClip {
    pub event: GameEvent,
    pub path: &'static str,
    /// Cuánto se usa del clip, en segundos, contados desde donde empieza a
    /// sonar de verdad. En 0 se usa entero. Sirve para los clips que traen de
    /// más: enemy_near son once segundos con varias tandas y solo se quiere la
    /// primera, o al enemigo le llevaría una eternidad terminar de saludar.
    pub seconds: f32,
    /// Cuánto pesa en la mezcla, relativo a SOUND_VOLUME. Los clips vienen de
    /// lados distintos y no están grabados al mismo nivel, así que acá se
    /// emparejan: 1.0 los deja como están, más los levanta y menos los baja.
    pub volume: f32,
}

pub const SOUNDS: &[SoundClip] = &[
    SoundClip {
        event: GameEvent::EnemyNear,
        path: "assets/sounds/enemy_near.mp3",
        seconds: 1.7,
        // suena todo el tiempo mientras haya alguien cerca: es el que menos
        // tiene que taparle el lugar a los demás
        volume: 0.45,
    },
    SoundClip {
        event: GameEvent::EnemyAttack,
        path: "assets/sounds/enemy_attack.mp3",
        seconds: 0.0,
        volume: 1.0,
    },
    // el mismo bonk para el golpe que entra y para el que lo termina de tumbar:
    // son dos entradas y no una porque cada evento se lleva su propia voz, y
    // así el volumen de uno no le pisa el del otro. Viene grabado bajito y es
    // la respuesta al único botón del juego, así que va por encima del resto
    SoundClip {
        event: GameEvent::EnemyHit,
        path: "assets/sounds/enemy_hit.mp3",
        seconds: 0.0,
        volume: 1.6,
    },
    SoundClip {
        event: GameEvent::EnemyDown,
        path: "assets/sounds/enemy_hit.mp3",
        seconds: 0.0,
        volume: 1.6,
    },
    SoundClip {
        event: GameEvent::ItemPickup,
        path: "assets/sounds/pickup.mp3",
        seconds: 0.0,
        volume: 1.0,
    },
    SoundClip {
        event: GameEvent::ItemDeliver,
        path: "assets/sounds/deliver.mp3",
        seconds: 0.0,
        volume: 1.0,
    },
    SoundClip {
        event: GameEvent::PlayerHurt,
        path: "assets/sounds/hurt.mp3",
        // el grito entero son cinco segundos y medio, y el golpe siguiente
        // puede llegar antes: con el arranque alcanza
        seconds: 2.0,
        volume: 1.0,
    },
];
/// Volumen con el que se reproducen, de 0 a 1.
pub const SOUND_VOLUME: f32 = 0.8;
/// Amplitud por debajo de la cual una muestra cuenta como silencio, que es lo
/// que se le recorta a cada clip por delante para que el efecto arranque en el
/// instante en que se dispara.
pub const SOUND_SILENCE: f32 = 0.01;

/// Música de fondo, en bucle de punta a punta y desde que arranca el juego.
/// Va en streaming, así que el archivo puede ser todo lo largo que se quiera.
pub const MUSIC_TRACK: &str = "assets/sounds/lobby.mp3";
/// Bien por debajo de los efectos: es lo que está sonando todo el tiempo.
pub const MUSIC_VOLUME: f32 = 0.35;

// ── Ataque ───────────────────────────────────────────────────────────────
/// Con qué se pega: botón del mouse y botón del control.
pub const ATTACK_MOUSE_BUTTON: MouseButton = MouseButton::MOUSE_BUTTON_LEFT;
pub const GAMEPAD_ATTACK_BUTTON: GamepadButton = GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT;
/// Cuánto dura el golpe entero y en qué fracción de esa duración toca. Subirlo
/// hace el golpe más lento de resolver y más fácil de castigar.
pub const ATTACK_DURATION: f32 = 0.34;
pub const ATTACK_CONTACT: f32 = 0.45;
/// Espera entre el final de un golpe y el siguiente.
pub const ATTACK_COOLDOWN: f32 = 0.12;
/// Alcance desde el centro del jugador, en píxeles del mundo, y abertura del
/// cono al que llega. Solo se lleva el golpe el enemigo más cercano de los que
/// caigan adentro.
pub const ATTACK_RANGE: f32 = 46.0;
pub const ATTACK_ARC: f32 = PI / 2.0;
/// Empujón que se lleva el enemigo golpeado, en píxeles del mundo.
pub const ATTACK_KNOCKBACK: f32 = 14.0;
/// Con qué se pega. Una imagen suelta (un cuadro) o una hoja con los cuadros
/// uno al lado del otro: el primero es el reposo y el resto se reparte a lo
/// largo del golpe. El bate es una sola imagen porque el vaivén no lo dibuja
/// la hoja sino el render, que lo gira y lo empuja contra el frente.
pub const ATTACK_SHEET: Option<&str> = Some("assets/bat.png");
pub const ATTACK_FRAMES: usize = 1;
/// Alto del arma en pantalla como fracción del alto de la ventana; el ancho
/// sale de la proporción de la imagen, así que cambiarla no la deforma.
pub const WEAPON_HEIGHT: f32 = 0.46;
/// Margen contra la esquina de abajo a la derecha, en fracción del alto.
pub const WEAPON_MARGIN: f32 = 0.01;
/// Cuánto se corre el arma durante el golpe, en fracción de su propio tamaño:
/// hacia el centro de la pantalla al pegar y hacia la esquina al tomar envión.
pub const WEAPON_SWING: f32 = 0.26;
/// El punto del bate sobre el que gira, en fracción de la imagen: el puño, que
/// queda abajo a la derecha. Es lo que hace que el mango se quede en la mano y
/// sea la punta la que barre la pantalla.
pub const WEAPON_PIVOT: Vector2 = Vector2::new(0.76, 0.94);
/// Cuánto se levanta hacia atrás para tomar envión y cuánto barre después
/// hacia adelante, en radianes. El barrido va al revés que las agujas del
/// reloj: el bate sale de la esquina y cruza el centro de la pantalla.
pub const WEAPON_WINDUP: f32 = 0.32;
pub const WEAPON_SWING_ARC: f32 = 1.15;
/// Cuánto crece el bate en el momento del contacto, en fracción de su tamaño:
/// es lo que lo acerca a la cámara y hace que el golpe salga hacia el frente y
/// no de costado.
pub const WEAPON_THRUST: f32 = 0.22;

// ── Objetos y entrega ────────────────────────────────────────────────────
/// Un objeto de los que hay que juntar. Igual que los enemigos, estas celdas
/// son piso: el objeto aparece ahí y la celda queda libre.
pub struct ItemTexture {
    pub cell: char,
    /// Cómo se ve esperando en el piso: la hoja con los cuadros de su
    /// animación uno al lado del otro, que se recorren en bucle. Sin imagen
    /// (None) se dibuja con ITEM_COLOR.
    pub idle: Option<&'static str>,
    /// Cuántos cuadros trae esa hoja y a cuántos por segundo se recorren. Un
    /// solo cuadro, o fps en 0, lo dejan quieto.
    pub frames: usize,
    pub fps: f32,
    /// Cómo se ve en el festejo de levantarlo. Sin esta, festeja con la cara
    /// de siempre.
    pub happy: Option<&'static str>,
}

pub const ITEM_TEXTURES: &[ItemTexture] = &[ItemTexture {
    cell: 'c',
    idle: Some("assets/bird-normal-sheet.png"),
    frames: 3,
    fps: 5.0,
    happy: Some("assets/bird-feliz.png"),
}];
pub const ITEM_COLOR: Color = Color::new(240, 196, 62, 255);
/// Alto del sprite como fracción de una celda: el mismo que el enemigo, así el
/// pájaro y el que lo cuida se ven del mismo porte.
pub const ITEM_SIZE: f32 = ENEMY_SIZE;
/// Ancho como fracción de ese alto, para el objeto que no tenga imagen: el que
/// sí la tiene saca el ancho de su propia proporción, que es lo que deja al
/// pájaro abrir las alas en el festejo sin cambiar de altura.
pub const ITEM_ASPECT: f32 = 1.0;
/// Distancia a la que se levanta un objeto, sumada al radio del jugador.
pub const ITEM_PICKUP_RADIUS: f32 = 12.0;
/// Cuántos se pueden llevar a la vez: obliga a volver a la meta a descargar.
pub const ITEM_CARRY_LIMIT: u32 = 3;
/// El festejo de levantar uno: el objeto pega un salto contento abajo de todo
/// de la pantalla, que es lo que avisa que entró. Va en pantalla y no en el
/// mundo porque se levanta pisándolo, y a esa distancia el sprite se proyecta
/// más alto que la ventana: de la única forma que se lo ve entero es así.
/// Cuánto dura el salto, en segundos...
pub const ITEM_CHEER: f32 = 0.5;
/// ...qué alto se dibuja, en fracción del alto de la ventana, y cuánto salta
/// desde el centro de la pantalla, en fracción de su propio alto.
pub const ITEM_CHEER_HEIGHT: f32 = 0.45;
pub const ITEM_CHEER_HOP: f32 = 0.35;

/// Celda donde se entrega lo recolectado, y a qué distancia de su centro cuenta
/// como entregado. La meta sigue siendo pared, así que alcanza con arrimarse:
/// el jugador nunca puede estar a menos de BLOCK_SIZE / 2 + PLAYER_RADIUS.
pub const GOAL_CELL: char = 'g';
pub const GOAL_REACH: f32 = 42.0;
pub const GOAL_COLOR: Color = Color::new(96, 214, 120, 255);

// ── Marcador ─────────────────────────────────────────────────────────────
/// Lado del bloque con el que se dibujan los dígitos del puntaje y del contador
/// de objetos en mano, y margen desde el borde de la pantalla.
pub const SCORE_BLOCK: f32 = 6.0;
pub const SCORE_MARGIN: f32 = 20.0;
pub const SCORE_COLOR: Color = Color::new(238, 232, 220, 255);
/// Número de nivel, arriba a la izquierda.
pub const LEVEL_BLOCK: f32 = 5.0;
pub const LEVEL_COLOR: Color = Color::new(146, 142, 152, 255);
/// Contador de lo que se lleva encima: tamaño del ícono, separación con el
/// número y bloque de sus dígitos.
pub const CARRY_ICON_SIZE: f32 = 18.0;
pub const CARRY_SPACING: f32 = 10.0;
pub const CARRY_BLOCK: f32 = 5.0;

// ── Vida ─────────────────────────────────────────────────────────────────
/// Corazones del HUD. Cada uno aguanta dos golpes.
pub const HEART_SLOTS: u32 = 5;
/// Hoja de corazones y el orden de sus cuadros dentro de la imagen.
pub const HEART_SHEET: &str = "assets/hearts.png";
pub const HEART_FRAMES: usize = 3;
pub const HEART_FRAME_FULL: usize = 0;
pub const HEART_FRAME_EMPTY: usize = 1;
pub const HEART_FRAME_HALF: usize = 2;
/// Lado de cada corazón en pantalla, separación entre ellos y margen desde la
/// esquina inferior izquierda.
pub const HEART_SIZE: f32 = 44.0;
pub const HEART_SPACING: f32 = 8.0;
pub const HEART_MARGIN: f32 = 20.0;
/// Segundos que dura el latido de los corazones al recibir un golpe, y cuánto
/// crecen en el pico de ese latido.
pub const HEART_HIT_ANIMATION: f32 = 0.4;
pub const HEART_HIT_SCALE: f32 = 0.35;
/// Segundos de invulnerabilidad después de cada golpe.
pub const DAMAGE_COOLDOWN: f32 = 1.0;

// ── Game over ──────────────────────────────────────────────
/// Cartel que aparece al quedarse sin vida. Cada carácter distinto de espacio
/// se pinta como un bloque, así que el dibujo se edita acá sin tocar el render.
pub const GAME_OVER_ART: &[&str] = &[
    "####  ##  #   # ####",
    "#    #  # ## ## #   ",
    "# ## #### # # # ### ",
    "#  # #  # #   # #   ",
    "#### #  # #   # ####",
    "                    ",
    " ##  #   # #### ### ",
    "#  # #   # #    #  #",
    "#  # #   # ###  ### ",
    "#  #  # #  #    # # ",
    " ##    #   #### #  #",
];
pub const GAME_OVER_HINT: &[&str] = &[
    "###  #  # #     ###  ##     ### ",
    "#  # #  # #    #    #  #    #  #",
    "###  #  # #     ##  ####    ### ",
    "#    #  # #       # #  #    # # ",
    "#     ##  #### ###  #  #    #  #",
];
/// La misma leyenda para cuando hay un control conectado: cambia la tecla por
/// el botón de GAMEPAD_RESTART_BUTTON.
pub const GAME_OVER_HINT_GAMEPAD: &[&str] = &[
    "###  #  # #     ###  ##      ## ",
    "#  # #  # #    #    #  #    #  #",
    "###  #  # #     ##  ####    ####",
    "#    #  # #       # #  #    #  #",
    "#     ##  #### ###  #  #    #  #",
];
/// Fracción del ancho de la pantalla que ocupan el cartel y su leyenda.
pub const GAME_OVER_FILL: f32 = 0.62;
pub const GAME_OVER_HINT_FILL: f32 = 0.3;
/// Separación entre el cartel y la leyenda, en fracción del alto.
pub const GAME_OVER_GAP: f32 = 0.07;
pub const GAME_OVER_COLOR: Color = Color::new(214, 52, 46, 255);
pub const GAME_OVER_HINT_COLOR: Color = Color::new(168, 164, 172, 255);
/// Velo que se pinta encima de la escena congelada.
pub const GAME_OVER_VEIL: Color = Color::new(10, 8, 12, 200);

/// Cartel de nivel terminado y cartel de partida ganada. Se dibujan igual que
/// el de game over y con la misma leyenda para seguir.
pub const LEVEL_CLEAR_ART: &[&str] = &[
    "#  # ### #  # #### #   ",
    "## #  #  #  # #    #   ",
    "# ##  #  #  # ###  #   ",
    "#  #  #  #  # #    #   ",
    "#  # ###  ##  #### ####",
    "                       ",
    "#    ### #### ### #### ",
    "#     #  #     #  #  # ",
    "#     #  ####  #  #  # ",
    "#     #     #  #  #  # ",
    "#### ### ####  #  #### ",
];
pub const LEVEL_CLEAR_COLOR: Color = Color::new(96, 214, 120, 255);
pub const VICTORY_ART: &[&str] = &[
    "#### #### #  # #### #### ### ####",
    "#    #  # ## # #  # #     #  #   ",
    "# ## #### # ## #### ####  #  ### ",
    "#  # #  # #  # #  #    #  #  #   ",
    "#### #  # #  # #  # ####  #  ####",
];
pub const VICTORY_COLOR: Color = Color::new(240, 196, 62, 255);

// ── Niebla ───────────────────────────────────────────────────────────────
pub const FOG_ENABLED: bool = true;
pub const FOG_COLOR: Color = Color::new(14, 13, 17, 255);
/// Celdas a partir de las cuales la niebla empieza a notarse...
pub const FOG_START: f32 = 1.5;
/// ...y celdas a partir de las cuales lo tapa todo.
pub const FOG_END: f32 = 17.0;
/// Exponente de la mezcla: >1 retrasa la niebla, <1 la adelanta.
pub const FOG_DENSITY: f32 = 1.35;

// ── Minimapa ─────────────────────────────────────────────────────────────
/// Lado de cada celda del minimapa compacto, en píxeles de pantalla.
pub const MINIMAP_COMPACT_CELL: f32 = 9.0;
/// Separación del minimapa compacto respecto de la esquina.
pub const MINIMAP_MARGIN: f32 = 16.0;
/// Fracción de la pantalla que ocupa el minimapa expandido (queda centrado).
pub const MINIMAP_EXPANDED_FILL: f32 = 0.85;
pub const MINIMAP_PADDING: f32 = 6.0;
pub const MINIMAP_BORDER_WIDTH: f32 = 2.0;
/// Opacidad de las paredes del minimapa sobre la vista 3D.
pub const MINIMAP_OPACITY: f32 = 0.85;
pub const MINIMAP_BACKGROUND: Color = Color::new(10, 10, 14, 190);
pub const MINIMAP_BORDER_COLOR: Color = Color::new(120, 122, 140, 220);
pub const MINIMAP_PLAYER_COLOR: Color = Color::new(235, 90, 70, 255);
pub const MINIMAP_RAY_COLOR: Color = Color::new(255, 226, 150, 255);
pub const MINIMAP_RAY_OPACITY: f32 = 0.30;
pub const MINIMAP_RAYS: usize = 48;

// ── Colores base ─────────────────────────────────────────────────────────
pub const SKY_COLOR: Color = Color::new(13, 10, 14, 255);
pub const FLOOR_COLOR: Color = Color::new(24, 22, 22, 255);
