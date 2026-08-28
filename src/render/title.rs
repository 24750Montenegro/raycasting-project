//! La pantalla de inicio: el menú con el nombre del juego y, detrás de una de
//! sus opciones, la guía de controles. Las dos se dibujan encima del primer
//! nivel ya cargado, que es lo que se ve detrás del velo, así que no hace falta
//! una escena aparte para el menú.
//!
//! Los rectángulos de los botones salen de `button_at`, que es la misma cuenta
//! que usa el bucle para saber a cuál le pegó el clic: la geometría vive en un
//! solo lado, y mover un botón no puede desincronizar lo que se ve de lo que se
//! puede apretar.

use super::font;
use crate::config;
use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

/// Cuál de las dos pantallas del menú se está mostrando.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    /// El nombre y las opciones.
    Main,
    /// La guía de controles, con la vuelta al menú.
    Controls,
}

/// Lo que hace cada botón. Quien lo reciba decide: acá solo se dibujan y se
/// dice cuál está debajo del mouse.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Play,
    Controls,
    Back,
}

impl Menu {
    /// Los botones de esta pantalla, de arriba hacia abajo. El primero es el
    /// que hace lo principal, y es el que se pinta fuerte.
    fn buttons(self) -> &'static [Button] {
        match self {
            Menu::Main => &[Button::Play, Button::Controls],
            Menu::Controls => &[Button::Back],
        }
    }

    /// El cartel de arriba: el nombre del juego en el menú y el encabezado en
    /// la guía, cada uno con su tamaño.
    fn heading(self) -> (&'static str, f32) {
        match self {
            Menu::Main => (config::TITLE_NAME, config::TITLE_BLOCK),
            Menu::Controls => (config::TITLE_HEADING, config::TITLE_HEADING_BLOCK),
        }
    }
}

impl Button {
    fn label(self) -> &'static str {
        match self {
            Button::Play => config::TITLE_PLAY,
            Button::Controls => config::TITLE_CONTROLS_LABEL,
            Button::Back => config::TITLE_BACK,
        }
    }

    fn size(self) -> Vector2 {
        let label = Vector2::new(
            font::text_width(self.label(), config::TITLE_PLAY_BLOCK),
            font::number_height(config::TITLE_PLAY_BLOCK),
        );

        label + config::TITLE_PLAY_PADDING * 2.0
    }
}

pub fn render_title(framebuffer: &mut Framebuffer, menu: Menu, hovered: Option<Button>) {
    let screen = framebuffer.size();
    framebuffer.blend_rect(0, 0, screen.x as i32, screen.y as i32, config::TITLE_VEIL);

    let (heading, block) = menu.heading();
    let mut top = layout(screen, menu).top;

    font::draw_text(
        framebuffer,
        heading,
        Vector2::new((screen.x - font::text_width(heading, block)) / 2.0, top),
        block,
        config::TITLE_COLOR,
    );
    top += font::number_height(block) + config::TITLE_GAP;

    if menu == Menu::Controls {
        draw_controls(framebuffer, screen, top);
    }

    // cada botón se ubica solo, con la misma cuenta con la que se lo apunta
    for (index, &button) in menu.buttons().iter().enumerate() {
        draw_button(framebuffer, screen, menu, button, index == 0, hovered == Some(button));
    }
}

/// El botón que está debajo de `point`, si hay alguno.
pub fn button_at(screen: Vector2, menu: Menu, point: Vector2) -> Option<Button> {
    menu.buttons().iter().copied().find(|&button| {
        let rect = button_rect(screen, menu, button);

        point.x >= rect.x
            && point.x <= rect.x + rect.width
            && point.y >= rect.y
            && point.y <= rect.y + rect.height
    })
}

/// La guía de controles, fila por fila.
fn draw_controls(framebuffer: &mut Framebuffer, screen: Vector2, mut top: f32) {
    // las dos columnas arrancan del mismo borde izquierdo, así la tecla y lo
    // que hace quedan alineadas fila a fila
    let left = (screen.x - controls_width()) / 2.0;
    let line = font::number_height(config::TITLE_CONTROL_BLOCK) + config::TITLE_CONTROL_SPACING;

    for (key, action) in config::TITLE_CONTROLS {
        font::draw_text(
            framebuffer,
            key,
            Vector2::new(left, top),
            config::TITLE_CONTROL_BLOCK,
            config::TITLE_KEY_COLOR,
        );
        font::draw_text(
            framebuffer,
            action,
            Vector2::new(left + config::TITLE_KEY_COLUMN, top),
            config::TITLE_CONTROL_BLOCK,
            config::TITLE_CONTROL_COLOR,
        );
        top += line;
    }
}

fn draw_button(
    framebuffer: &mut Framebuffer,
    screen: Vector2,
    menu: Menu,
    button: Button,
    main: bool,
    hovered: bool,
) {
    let rect = button_rect(screen, menu, button);
    let color = match (main, hovered) {
        (true, false) => config::TITLE_BUTTON_COLOR,
        (true, true) => config::TITLE_BUTTON_HOVER,
        (false, false) => config::TITLE_BUTTON_SECOND,
        (false, true) => config::TITLE_BUTTON_SECOND_HOVER,
    };

    framebuffer.fill_rect(
        rect.x as i32,
        rect.y as i32,
        rect.width as i32,
        rect.height as i32,
        color,
    );
    font::draw_text(
        framebuffer,
        button.label(),
        Vector2::new(
            rect.x + config::TITLE_PLAY_PADDING.x,
            rect.y + config::TITLE_PLAY_PADDING.y,
        ),
        config::TITLE_PLAY_BLOCK,
        config::TITLE_PLAY_COLOR,
    );
}

/// Dónde cae un botón: debajo del cartel, de la guía si la hay, y de los
/// botones que van antes que él.
fn button_rect(screen: Vector2, menu: Menu, button: Button) -> Rectangle {
    let placed = layout(screen, menu);
    let mut top = placed.top + placed.height - buttons_height(menu);

    for &current in menu.buttons() {
        let size = current.size();
        if current == button {
            return Rectangle::new((screen.x - size.x) / 2.0, top, size.x, size.y);
        }
        top += size.y + config::TITLE_BUTTON_SPACING;
    }

    Rectangle::new(0.0, 0.0, 0.0, 0.0) // ese botón no es de esta pantalla
}

/// Dónde arranca y cuánto ocupa todo el cartel. Sale de una sola cuenta para
/// que el conjunto quede centrado por más que se agreguen líneas u opciones.
struct Layout {
    top: f32,
    height: f32,
}

fn layout(screen: Vector2, menu: Menu) -> Layout {
    let (_, block) = menu.heading();
    let guide = if menu == Menu::Controls {
        controls_height() + config::TITLE_GAP
    } else {
        0.0
    };

    let height =
        font::number_height(block) + config::TITLE_GAP + guide + buttons_height(menu);

    Layout {
        top: (screen.y - height) / 2.0,
        height,
    }
}

fn buttons_height(menu: Menu) -> f32 {
    let buttons = menu.buttons();
    let spacing = (buttons.len() as f32 - 1.0).max(0.0) * config::TITLE_BUTTON_SPACING;

    buttons.iter().map(|button| button.size().y).sum::<f32>() + spacing
}

fn controls_height() -> f32 {
    let lines = config::TITLE_CONTROLS.len() as f32;

    lines * font::number_height(config::TITLE_CONTROL_BLOCK)
        + (lines - 1.0).max(0.0) * config::TITLE_CONTROL_SPACING
}

/// Lo que ocupa de ancho la guía: la columna de las teclas más la acción más
/// larga, que es lo que hay que centrar.
fn controls_width() -> f32 {
    let action = config::TITLE_CONTROLS
        .iter()
        .map(|(_, action)| font::text_width(action, config::TITLE_CONTROL_BLOCK))
        .fold(0.0f32, f32::max);

    config::TITLE_KEY_COLUMN + action
}
