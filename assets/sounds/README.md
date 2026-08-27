# Sonidos

Cada archivo se engancha a un evento en `SOUNDS` (src/config.rs). El que no
este todavia se saltea: el juego arranca igual y ese evento queda mudo.

| archivo | cuando suena |
| --- | --- |
| `attack.wav` | el jugador tira un golpe |
| `enemy_hit.wav` | el golpe entra en un enemigo que sigue en pie |
| `enemy_down.wav` | un enemigo se queda sin aguante |
| `pickup.wav` | se levanta un objeto |
| `deliver.wav` | se entrega lo recolectado en la meta |
| `hurt.wav` | el jugador recibe dano |
| `game_over.wav` | el jugador se queda sin vida |
| `level_clear.wav` | se entregaron todos los objetos del nivel |
