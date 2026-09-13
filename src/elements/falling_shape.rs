use std::cell::Cell;

use incredible::*;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

// -----------------------------
// Options
// -----------------------------
#[derive(Clone, Debug)]
pub struct FallingShapeOptions {
    pub color: u8,
    pub shape: char,
    pub velocity: f32,
}

impl Default for FallingShapeOptions {
    fn default() -> Self {
        Self {
            color: 15,
            shape: '+',
            velocity: 0.0,
        }
    }
}

// -----------------------------
// FallingShape
// -----------------------------
element! {
  pub struct FallingShape<S> {
      options: FallingShapeOptions = FallingShapeOptions::default(),
      y_f: Cell<f32> = Cell::new(0.0),
      velocity: Cell<f32> = Cell::new(0.0),
  }
}

impl<S: Clone + PartialEq> FallingShape<S> {
    /// Constructs a falling shape with default options applied to look/color.
    pub fn default() -> Self {
        let el = Self::blank();

        el.look(Look::from(el.options.shape))
            .color(Some(Color::Ansi(el.options.color)));
        el.velocity.set(el.options.velocity);

        el
    }

    /// Constructs a falling shape with the provided options.
    pub fn new(options: FallingShapeOptions) -> Self {
        let mut el = Self::default();

        el.options.color = options.color;
        el.options.shape = options.shape;
        el.options.velocity = options.velocity;
        el.velocity.set(options.velocity);

        el.look(Look::from(el.options.shape))
            .color(Some(Color::Ansi(el.options.color)));

        el
    }

    /// Advances the falling shape using simple gravity.
    pub fn step(&self, gravity: f32, max_velocity: f32) {
        let v = (self.velocity.get() + gravity).min(max_velocity);
        self.velocity.set(v);

        let next_y = self.y_f.get() + v;
        self.y_f.set(next_y);
        self.y(next_y.floor() as isize);
    }
}
