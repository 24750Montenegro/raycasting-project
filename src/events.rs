//! Bus de eventos del juego. Todo lo que pasa y "se nota" (un golpe, una
//! moneda, una entrega) se anota acá durante la actualización y se consume una
//! sola vez por cuadro. Sirve para colgar reacciones —sonido, sacudidas de
//! cámara, mensajes— sin que quien produce el evento sepa nada de ellas.

/// Algo que acaba de pasar en el mundo, en el cuadro que se está actualizando.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameEvent {
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

/// Cola de eventos del cuadro.
#[derive(Default)]
pub struct Events {
    queue: Vec<GameEvent>,
}

impl Events {
    pub fn new() -> Self {
        Events::default()
    }

    pub fn push(&mut self, event: GameEvent) {
        self.queue.push(event);
    }

    /// Vacía la cola y devuelve lo que había, en orden.
    pub fn drain(&mut self) -> std::vec::Drain<'_, GameEvent> {
        self.queue.drain(..)
    }
}
