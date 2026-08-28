//! Bus de eventos del juego. Todo lo que pasa y "se nota" (un golpe, una
//! moneda, una entrega) se anota acá durante la actualización y se consume una
//! sola vez por cuadro. Sirve para colgar reacciones —sonido, sacudidas de
//! cámara, mensajes— sin que quien produce el evento sepa nada de ellas.

/// Algo que acaba de pasar en el mundo, en el cuadro que se está actualizando.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameEvent {
    /// El jugador tiró un golpe, haya acertado o no.
    Attack,
    /// Un enemigo está lo bastante cerca como para hacerse oír.
    EnemyNear,
    /// Un enemigo saltó encima del jugador, le vaya a pegar o no.
    EnemyAttack,
    /// Un golpe entró en un enemigo que sigue vivo.
    EnemyHit,
    /// Un enemigo se quedó sin aguante.
    EnemyDown,
    /// El jugador levantó un objeto.
    ItemPickup,
    /// El jugador dejó lo que llevaba en el punto de entrega.
    ItemDeliver,
    /// El jugador recibió daño.
    PlayerHurt,
    /// El jugador se quedó sin vida.
    PlayerDown,
    /// Se entregaron todos los objetos del nivel.
    LevelClear,
}

/// Cola de eventos del cuadro. Cada uno viene con cuánto se tiene que hacer
/// notar, en [0, 1]: el sonido lo usa de volumen, y lo mismo podría escalar una
/// sacudida de cámara. Casi todo pasa o no pasa —y va con intensidad 1—, pero
/// lo que depende de una distancia, como el enemigo que se acerca, necesita
/// decir además cuánto.
#[derive(Default)]
pub struct Events {
    queue: Vec<(GameEvent, f32)>,
}

impl Events {
    pub fn new() -> Self {
        Events::default()
    }

    /// Anota un evento a intensidad plena.
    pub fn push(&mut self, event: GameEvent) {
        self.push_with(event, 1.0);
    }

    pub fn push_with(&mut self, event: GameEvent, intensity: f32) {
        self.queue.push((event, intensity.clamp(0.0, 1.0)));
    }

    /// Vacía la cola y devuelve lo que había, en orden.
    pub fn drain(&mut self) -> std::vec::Drain<'_, (GameEvent, f32)> {
        self.queue.drain(..)
    }
}
