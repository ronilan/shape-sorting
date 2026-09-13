use std::cell::Cell;

use incredible::*;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

// -----------------------------
// Options
// -----------------------------
#[derive(Clone, Debug)]
pub struct ColoredShapeOptions {
    pub color: u8,
    pub shape: char,
    pub draggable: bool,
}

impl Default for ColoredShapeOptions {
    fn default() -> Self {
        Self {
            color: 0,
            shape: '+',
            draggable: false,
        }
    }
}

// -----------------------------
// State
// -----------------------------
#[derive(Clone, Debug, Default)]
pub struct ColoredShapeState {
    pub draggable: Cell<bool>,
}

// -----------------------------
// ColoredShape
// -----------------------------
element! {
  pub struct ColoredShape<S> {
      internal_state: ColoredShapeState = ColoredShapeState::default(),
      options: ColoredShapeOptions = ColoredShapeOptions::default(),
  }
}

impl<S: Clone + PartialEq> ColoredShape<S> {
    /// Constructs a default colored shape with default options applied to look/color.
    pub fn default() -> Self {
        let el = Self::blank();

        el.look(Look::from(el.options.shape))
            .color(Some(Color::Ansi(el.options.color)));
        el.internal_state.draggable.set(el.options.draggable);

        el.internal_on_mouse(|el, _, event| {
            if event.mouse == Mouse::Drag
                && el.status.dragged.get()
                && el.internal_state.draggable.get()
            {
                el.x(event.x - el.status.drag_offset_x.get());
                el.y(event.y - el.status.drag_offset_y.get());
            }
        });

        el.decorate();

        el
    }

    /// Constructs a colored shape with the provided options.
    pub fn new(options: ColoredShapeOptions) -> Self {
        let mut el = Self::default();

        el.options.color = options.color;
        el.options.shape = options.shape;
        el.options.draggable = options.draggable;
        el.internal_state.draggable.set(options.draggable);

        el.look(Look::from(el.options.shape))
            .color(Some(Color::Ansi(el.options.color)));

        el.decorate();

        el
    }

    /// Enables or disables mouse dragging for this shape.
    pub fn draggable(&self, enabled: bool) -> &Self {
        self.internal_state.draggable.set(enabled);
        self
    }
}
