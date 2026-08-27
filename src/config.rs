//! Parámetros ajustables del motor. Todo lo que se quiera "tunear" vive aquí.

use raylib::prelude::*;
use std::f32::consts::PI;

// ── Ventana ──────────────────────────────────────────────────────────────
pub const WINDOW_WIDTH: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;
pub const TARGET_FPS: u32 = 60;

// ── Mundo ────────────────────────────────────────────────────────────────
pub const MAZE_FILE: &str = "maze.txt";
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
/// Textura de cada carácter del laberinto, con cuántas veces se repite dentro
/// de una celda en cada eje: 1.0 deja la imagen entera cubriendo la cara, 4.0
/// la mosaica 4x4. Los caracteres que no aparezcan se pintan con el color plano
/// de render::cell_color.
pub const WALL_TEXTURES: &[(char, &str, f32)] = &[
    ('-', "assets/netherbrick.webp", 4.0),
    ('+', "assets/netherbrick.jpg", 1.0),
];
/// Lado máximo al que se reescalan las texturas al cargarlas. Bajarlo también
/// acelera el render: cuanto más chica la textura, mejor cae en caché.
pub const TEXTURE_MAX_SIZE: u32 = 128;
/// Filtrado bilineal al muestrear. Apagarlo deja los téxeles duros y casi
/// duplica los cuadros por segundo: son 4 lecturas de textura por píxel.
pub const TEXTURE_BILINEAR: bool = true;

// ── Enemigos ─────────────────────────────────────────────────────────────
/// Enemigos: carácter en el laberinto -> textura del sprite. Igual que la del
/// jugador, estas celdas son piso: el enemigo aparece ahí y la celda queda
/// libre. Sin textura (None) se dibujan con ENEMY_COLOR.
pub const ENEMY_TEXTURES: &[(char, Option<&str>)] = &[('e', None)];
pub const ENEMY_COLOR: Color = Color::new(196, 38, 38, 255);
/// Velocidad con la que persiguen, en píxeles del mundo por segundo.
pub const ENEMY_SPEED: f32 = 85.0;
/// Radio del enemigo, para chocar con las paredes y con el jugador. Tiene que
/// ser lo bastante chico como para poder cruzarse con el jugador dentro de un
/// pasillo: hace falta que 2 * (ENEMY_RADIUS + PLAYER_RADIUS) < BLOCK_SIZE.
pub const ENEMY_RADIUS: f32 = 6.5;
/// Golpes que aguanta antes de caer.
pub const ENEMY_HITS: u32 = 5;
/// Segundos que el enemigo queda frenado después de cada golpe, que es la
/// ventana para sacárselo de encima.
pub const ENEMY_STAGGER: f32 = 0.22;
/// Destello del sprite al recibir un golpe: cuánto dura y hacia qué color se
/// mezcla en el pico.
pub const ENEMY_HIT_FLASH: f32 = 0.14;
pub const ENEMY_HIT_COLOR: Color = Color::new(255, 240, 232, 255);
/// Alto del sprite como fracción de una celda...
pub const ENEMY_SIZE: f32 = 0.75;
/// ...y ancho como fracción de ese alto: <1 deja una figura angosta, acorde al
/// poco lugar que ocupa el enemigo en el mundo.
pub const ENEMY_ASPECT: f32 = 0.42;
/// Lado máximo al que se reescalan los sprites y las hojas al cargarlos.
pub const SPRITE_MAX_SIZE: u32 = 128;
/// Alfa por debajo del cual un píxel de sprite se considera vacío.
pub const SPRITE_ALPHA_CUTOFF: u8 = 8;
/// Distancia mínima a la cámara para dibujar un sprite, en píxeles del mundo.
/// Más cerca que esto el tamaño se dispara y no queda nada útil en pantalla.
pub const SPRITE_NEAR_PLANE: f32 = 6.0;

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
/// Hoja de la animación del golpe, con el primer cuadro en reposo y el resto
/// repartido a lo largo del golpe. Sin hoja (None) se dibuja el puño de
/// bloques de render::weapon.
pub const ATTACK_SHEET: Option<&str> = None;
pub const ATTACK_FRAMES: usize = 4;
/// Alto del arma en pantalla como fracción del alto de la ventana, ancho como
/// fracción de ese alto, margen contra la esquina y cuánto se desplaza durante
/// el golpe (en fracción de su propio tamaño).
pub const WEAPON_HEIGHT: f32 = 0.42;
pub const WEAPON_ASPECT: f32 = 0.34;
pub const WEAPON_MARGIN: f32 = 0.01;
pub const WEAPON_SWING: f32 = 0.22;
/// Colores del puño de reemplazo, mientras no haya hoja.
pub const WEAPON_COLOR: Color = Color::new(206, 158, 122, 255);
pub const WEAPON_ARM_COLOR: Color = Color::new(74, 62, 84, 255);

// ── Objetos y entrega ────────────────────────────────────────────────────
/// Objetos que se juntan: carácter en el laberinto -> textura del sprite.
/// Igual que los enemigos, estas celdas son piso: el objeto aparece ahí y la
/// celda queda libre. Sin textura (None) se dibujan con ITEM_COLOR.
pub const ITEM_TEXTURES: &[(char, Option<&str>)] = &[('c', None)];
pub const ITEM_COLOR: Color = Color::new(240, 196, 62, 255);
/// Alto del sprite como fracción de una celda y ancho como fracción del alto.
pub const ITEM_SIZE: f32 = 0.3;
pub const ITEM_ASPECT: f32 = 1.0;
/// Distancia a la que se levanta un objeto, sumada al radio del jugador.
pub const ITEM_PICKUP_RADIUS: f32 = 12.0;
/// Cuántos se pueden llevar a la vez: obliga a volver a la meta a descargar.
pub const ITEM_CARRY_LIMIT: u32 = 3;
/// Puntos que da cada objeto al entregarlo.
pub const ITEM_SCORE: u32 = 100;
/// Flote del sprite: altura sobre el piso en fracción de celda, amplitud y
/// velocidad del vaivén, y desfase entre un objeto y el siguiente.
pub const ITEM_LIFT: f32 = 0.34;
pub const ITEM_BOB_AMPLITUDE: f32 = 0.05;
pub const ITEM_BOB_SPEED: f32 = 2.6;
pub const ITEM_BOB_OFFSET: f32 = 1.1;

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
