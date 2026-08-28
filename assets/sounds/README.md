# Sonidos

Cada archivo se engancha a un evento en `SOUNDS` (src/config.rs). El que no
este todavia se saltea: el juego arranca igual y ese evento queda mudo.

| archivo | cuando suena |
| --- | --- |
| `enemy_near.mp3` | un enemigo esta cerca; mas seguido y mas fuerte cuanto mas se acerca |
| `enemy_attack.mp3` | un enemigo salta encima del jugador |
| `enemy_hit.mp3` | el golpe entra en un enemigo, lo deje en pie o lo tumbe |
| `pickup.mp3` | se levanta un objeto |
| `deliver.mp3` | se entrega lo recolectado en la meta |
| `hurt.mp3` | el jugador recibe dano |
| `lobby.mp3` | musica de fondo, en bucle y de punta a punta de la partida |

Los nombres van sin espacios ni acentos a proposito: raylib abre los archivos
por la ruta cruda, y en Windows un nombre con acentos no lo encuentra.

A cada efecto se le recorta el silencio del principio al cargarlo, asi que no
hace falta editar el clip para que el golpe suene en el momento en que pasa.
Lo que sobra por atras se corta con `seconds` en la tabla de `SOUNDS`.
