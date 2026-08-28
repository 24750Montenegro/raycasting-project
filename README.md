# THE BIRDS

Un raycaster escrito en Rust sobre raylib, sin motor 3D: el mundo se dibuja
píxel por píxel en la CPU y se sube a la placa como una sola textura por cuadro.

Sos el que tiene el bate. Los pájaros están tirados por el laberinto y hay que
sacarlos de ahí antes de que los duolingos te alcancen.

Se juega con teclado y mouse o **con mando**, indistintamente: los dos están
enchufados a la vez y se puede saltar de uno al otro en medio de la partida.

## Cómo se ejecuta

Hace falta **Rust 1.85 o más nuevo** (el proyecto usa la edición 2024) y el
compilador de C del sistema: `raylib-sys` compila raylib desde el fuente la
primera vez. En Windows eso significa las Build Tools de Visual Studio; en Linux,
`gcc`, `cmake` y los headers de X11 o Wayland.

```sh
cargo run
```

La primera compilación tarda un rato largo porque construye raylib entero; las
siguientes son de segundos.

**No hace falta `--release`.** El perfil de desarrollo ya viene con
`opt-level = 3` y sin los chequeos de desbordamiento, que es lo que necesita un
render que es CPU pura: con el perfil dev de fábrica cada cuadro cuesta unas tres
veces y media más.

Hay que ejecutarlo **desde la raíz del repo**: las rutas de los assets y de los
laberintos son relativas al directorio de trabajo.

```sh
cargo test
```

Son 15 tests y ninguno necesita placa de video. Los que tocan sonido abren el
dispositivo de audio y se saltean solos si la máquina no tiene; los demás cargan
los assets de verdad desde el disco.

## Cómo se juega

Arranca en el menú: **JUGAR** empieza y **CONTROLES** muestra la guía. Se navega
con el mouse, con ENTER o con el mando.

### Con teclado y mouse

| tecla | qué hace |
| --- | --- |
| `W` `A` `S` `D` | caminar y desplazarse de costado |
| mouse (o las flechas `←` `→`) | mirar |
| clic izquierdo | pegar con el bate |
| `M` | minimapa chico / minimapa grande |
| `R` | pasar al nivel siguiente, o reiniciar después de perder |
| `ENTER` | confirmar en el menú |

### Con mando

El mando se lee del primer puerto y no hay que activarlo: si está conectado,
anda. El mapeo es el de Xbox, que es el que raylib normaliza para casi cualquier
mando genérico.

| stick / botón | qué hace |
| --- | --- |
| stick izquierdo | caminar y desplazarse de costado |
| stick derecho | mirar |
| **X** | pegar con el bate |
| **Y** | minimapa chico / minimapa grande |
| **A** | seguir al nivel siguiente, reiniciar y confirmar en el menú |

Dos detalles que hacen que se sienta bien y que se ajustan en `config.rs`:

- la **zona muerta del stick izquierdo es radial**, o sea que se mide el módulo
  del vector y no cada eje por su lado, así caminar en diagonal no necesita
  empujar más el stick; lo que queda del recorrido se reescala desde cero para
  que el movimiento no arranque de un salto;
- el **stick derecho tiene respuesta exponencial**, que achica el giro cerca del
  centro y deja la velocidad máxima solo al final del recorrido: es lo que
  permite apuntar fino sin perder el giro rápido.

También se puede invertir el eje horizontal de la cámara y cambiar el puerto, la
zona muerta y la velocidad de giro (las constantes `GAMEPAD_*`).

La leyenda de los carteles cambia sola: con mando conectado nombra el botón en
vez de la tecla.

### La partida

1. **Juntá pájaros.** Están en el piso, animados. Se levantan caminándoles
   encima; el pájaro contento pega un salto en el medio de la pantalla y el
   contador de abajo a la derecha sube. No hay tope: podés cargar todos los que
   juntes.
2. **Entregá en la meta**, que en el minimapa se ve verde. Alcanza con arrimarse.
   Entregar **termina el nivel al instante**: puntúa todo lo que llevabas encima,
   y lo que haya quedado dando vueltas por el mapa se pierde.
3. **El cartel te cuenta lo que dejaste.** El pájaro triste al pie del cartel de
   fin de nivel es cuántos no rescataste.

Ahí está la decisión de todo el nivel: entregás lo que tenés y cerrás seguro, o
volvés a meterte a buscar más y te cruzás otra vez con los bichos.

### Lo que te mata

Los duolingos te persiguen. **No lastiman al rozarte**: pegan saltando encima
tuyo, y ese salto es a la vez el aviso de que el golpe viene y lo que los sube
hasta la línea de tu vista, porque parados te quedan por debajo. Correrse
mientras están en el aire es lo que hace fallar el golpe.

Cada golpe que entra se lleva medio corazón de los cinco que tenés, con un
segundo de invulnerabilidad después. Sin corazones es game over y se vuelve a
empezar desde el primer nivel.

Antes de verlos los vas a escuchar: cuanto más cerca están, más seguido y más
fuerte se hacen oír. Sirve de radar.

### El bate

El clic tira el golpe. El daño **no se aplica al apretar el botón** sino cuando
la animación llega al instante del contacto, así lo que se ve en pantalla y lo
que pasa en el mundo son lo mismo. Llega a 46 píxeles del mundo en un cono de 90
grados hacia adelante, y se lo lleva un solo enemigo por golpe: el más cercano de
los que caigan adentro. El que lo recibe queda empujado y frenado un rato corto,
que es la ventana para sacártelo de encima.

### Los niveles

Son cuatro, cada uno con su mapa y su dificultad. El puntaje se arrastra de uno
al siguiente.

| nivel | mapa | golpes que aguanta cada bicho | velocidad | puntos por pájaro |
| --- | --- | --- | --- | --- |
| 1 | `maze.txt` — pasillos anchos | 5 | 85 | 100 |
| 2 | `maze2.txt` — el peine | 6 | 100 | 150 |
| 3 | `maze3.txt` — cuatro salas | 7 | 115 | 200 |
| 4 | `maze4.txt` — laberinto cerrado | 8 | 130 | 300 |

Ningún bicho llega a los 200 del jugador: correr siempre sigue siendo una opción.

## La lógica del juego

### El bucle

`main.rs` tiene una máquina de estados chica —menú, controles, jugando, nivel
listo, game over, ganaste— y fuera de *jugando* el mundo queda **congelado**: se
sigue dibujando la última escena con un cartel encima. Eso es lo que hace que
cada aviso salga una sola vez y no en cada cuadro del cartel.

Un cuadro es siempre lo mismo:

1. leer la entrada y mover al jugador,
2. correr enemigos, objetos y los relojes de animación,
3. vaciar la cola de eventos hacia el sonido,
4. dibujar mundo, sprites, arma, minimapa y HUD, en ese orden.

### El bus de eventos

Todo lo que pasa y *se nota* —un golpe, un pájaro levantado, una entrega, un
enemigo que se acerca— se anota en una cola durante la actualización y se consume
una sola vez por cuadro (`events.rs`). Quien produce el evento no sabe nada de
quién reacciona: hoy es el sonido, mañana podría ser una sacudida de cámara. Cada
evento viaja con una intensidad en 0..1, que es lo que le deja al aviso del
enemigo sonar más fuerte cuanto más cerca está.

### El mundo

El laberinto es un archivo de texto donde **cada carácter es una celda** de 40
píxeles del mundo (`maze.rs`).

| carácter | qué es |
| --- | --- |
| espacio | piso |
| `p` | dónde empieza el jugador (tiene que haber exactamente una) |
| `c` | bird (moneda) |
| `e` | duolingo malo |
| `g` | la entrega; sigue siendo pared, por eso alcanza con arrimarse |
| `+` `-` `#` y la barra vertical | paredes, cada una con su textura |

Las celdas `c` y `e` **se vacían del mapa** al cargar el nivel: el pájaro o el
bicho nacen ahí y la celda queda libre, así el bucle caliente del raycaster nunca
tiene que saber que existen.

### El render

No hay geometría 3D. `framebuffer.rs` es un buffer de píxeles RGBA en RAM que se
sube a una única textura de GPU por cuadro.

Para cada columna de la pantalla se traza un rayo (`caster.rs`): avanza a pasos
de 0.2 celdas hasta encontrar una pared y ahí hace 12 bisecciones para afinar el
impacto, que queda dentro de dos milésimas de píxel del plano. De la distancia
sale el alto de la columna y de dónde pegó sale la coordenada de la textura. La
textura se muestrea **por columna** y no por píxel: resolver una sola vez la
coordenada horizontal y el envoltorio es lo que mantiene el render en presupuesto.

Esa misma pasada llena un **depth buffer de una entrada por columna**, y eso es
todo lo que hace falta para que las paredes tapen a los sprites. Enemigos y
pájaros se dibujan como billboards —siempre de frente a la cámara— ordenados de
atrás hacia adelante (`render/sprites.rs`).

Encima va una luz que acompaña al jugador y una niebla que cierra el fondo. Las
caras horizontales de las paredes se pintan más oscuras a propósito: esa
diferencia constante es lo que hace que las esquinas se lean como esquinas y no
como un plano continuo.

### Las imágenes

Todas viven en RAM como píxeles (`textures.rs`), reescaladas al cargarlas. Una
pared animada es una hoja con los cuadros uno al lado del otro y un reloj que los
recorre en bucle; ese reloj corre con la simulación y no con el dibujo, así lo
que congela el mundo también las congela. Un video se convierte en esa hoja una
sola vez, fuera del juego:

```sh
python tools/video_to_sheet.py assets/dog-knife.mp4 assets/dog-knife.png
```

Los sprites se recortan por alfa al rectángulo que de verdad ocupan, y los
cuadros de una misma animación comparten un único recorte para que la animación
no salte de un cuadro al siguiente.

### El sonido

Cada evento se ata a un archivo en la tabla `SOUNDS` de `config.rs`. Al cargarlos
se les recorta el silencio del arranque —los efectos bajados de internet vienen
con hasta dos segundos de nada adelante— porque cada disparo reinicia el clip:
sin recortar, el golpe se escucharía tarde o directamente nunca. La música de
fondo va en streaming y en bucle. `assets/sounds/README.md` tiene la tabla
completa.

Si un archivo no está, ese evento queda mudo y el juego arranca igual. Lo mismo
con las texturas: lo que falta cae a un color plano.

## Cómo tocarlo

Todo lo ajustable vive en **`src/config.rs`**, con un comentario por constante:
velocidades, alcances, colores, la luz, la niebla, el tamaño de los sprites, la
duración del golpe, el festejo del pájaro, la tabla de niveles y la de sonidos.

**Para agregar un nivel** alcanza con dejar el archivo del laberinto en la raíz y
sumar una fila a `LEVELS`. El test `todos_los_niveles_se_pueden_jugar` recorre esa
tabla y verifica que el mapa exista, que tenga un solo arranque y que los pájaros
y la entrega queden del lado alcanzable: un tabique de más y el test lo caza antes
de que lo juegues.

## Mapa del código

| archivo | de qué se ocupa |
| --- | --- |
| `main.rs` | el bucle, las fases y el orden del dibujo |
| `config.rs` | todo lo que se quiera tunear |
| `maze.rs` | cargar el mapa, colisiones y celdas de nacimiento |
| `player.rs` | entrada y movimiento del jugador |
| `caster.rs` | trazar un rayo contra el laberinto |
| `framebuffer.rs` | los píxeles y la subida a la GPU |
| `textures.rs` | imágenes, hojas de animación y recortes |
| `enemy.rs` | perseguir, saltar, pegar y aguantar golpes |
| `items.rs` | los pájaros, levantarlos y entregarlos |
| `attack.rs` | el ciclo del golpe del jugador |
| `health.rs` | la vida, contada en medios corazones |
| `stage.rs` | armar un nivel y pasar al siguiente |
| `events.rs` | la cola de eventos del cuadro |
| `audio.rs` | efectos y música |
| `gamepad.rs` | sticks y botones del control |
| `render/` | mundo, sprites, arma, HUD, minimapa, carteles, menú y la fuente de bloques |
