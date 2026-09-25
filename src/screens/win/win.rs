use rand::{Rng, rng};
use std::cell::Cell;

use incredible::*;
use incredible_elements::{Label, Rectangle};
use incredible_helpers_layout::*;
use incredible_macros_decl::element_for;

use crate::core::State;
use crate::core::constants::{COLORS, SYMBOLS};
use crate::core::enums::GameState;
use crate::elements::{FallingShape, FallingShapeOptions, Timer};
use crate::screens::win::ui;

const MAX_FALLING_SHAPES: usize = 600;
const SPAWN_CHANCE: u32 = 2; // 1 in N per tick
const FALL_STEP_TICKS: usize = 2; // update every N loops
const SPAWN_BATCH: usize = 6;
const GRAVITY: f32 = 0.04;
const MAX_VELOCITY: f32 = 1.4;

element_for! {
  pub struct Win for State {
      active: Cell<bool> = Cell::new(false),
      tick: Cell<u32> = Cell::new(0),
      width: Cell<usize> = Cell::new(0),
      height: Cell<usize> = Cell::new(0)
  }
}

impl Win {
    /// Builds the win overlay and wires animation/input handlers.
    pub fn new() -> Self {
        let el = Self::blank();

        let width = Platform::columns();
        let height = Platform::rows();

        el.look(Look::from((width, height))).fused(true);

        let overlay = ui::overlay::build();
        let label = ui::text::build();
        let timer = ui::timer::build();

        overlay.add(label);
        overlay.add(timer);
        overlay.elements_flow_down(1).elements_to_center();

        el.add(overlay);

        // Setting showed to false on the fully composed element
        // allows setter to act on sub elements.
        el.showed(false);

        el.width.set(width);
        el.height.set(height);

        el
            // Any key ends the win screen and returns to Splash.
            .internal_on_key(|el, state, _event| {
                if state.game_state == GameState::Completed {
                    state.stats.total_time_wasted += state.stats.last_game_time;
                    state.stats.last_game_time = 0;
                    state.game_state = GameState::Splash;
                    el.showed(false);
                    el.active.set(false);
                }
            })
            // Any click ends the win screen and returns to Splash.
            .internal_on_mouse(|el, state, event| {
                if state.game_state == GameState::Completed && event.mouse == Mouse::Click {
                    state.stats.total_time_wasted += state.stats.last_game_time;
                    state.stats.last_game_time = 0;
                    state.game_state = GameState::Splash;
                    el.showed(false);
                    el.active.set(false);
                }
            })
            // Keep overlay and centered text aligned to terminal size.
            .internal_on_window(|el, _state, event| {
                if event.window == Window::Resize {
                    el.resize(event.columns, event.rows);
                }
            })
            // Animate falling shapes on a fixed tick cadence.
            .internal_on_loop(|el, _state, event| {
                if !el.active.get() {
                    return;
                }

                let tick = el.tick.get().wrapping_add(1);
                el.tick.set(tick);

                if event.loop_count % FALL_STEP_TICKS == 0 {
                    el.advance_shapes();
                    el.spawn_shapes();
                    el.draw();
                }
            })
            .internal_on_state(|el, state, _ev| {
                let is_active = state.game_state == GameState::Completed;
                if is_active != el.active.get() {
                    el.active.set(is_active);
                    el.clear_shapes();
                    el.tick.set(0);
                }

                if is_active {
                    if let Some(overlay) = el.elements.cot::<Rectangle<State>>().first() {
                        if let Some(timer) = overlay.elements.cot::<Timer<State>>().first() {
                            timer.set_elapsed_ms(state.stats.last_game_time);
                        }
                    }
                }

                el.showed(is_active).focused(is_active);
            });

        el
    }

    /// Resizes the overlay and keeps the text centered.
    fn resize(&self, width: usize, height: usize) {
        if width == 0 || height == 0 {
            return;
        }

        self.width.set(width);
        self.height.set(height);
        self.look(Look::from((width, height)));

        if let Some(overlay) = self.elements.cot::<Rectangle<State>>().first() {
            overlay.width(width).height(height);
        }

        if let Some(overlay) = self.elements.cot::<Rectangle<State>>().first() {
            overlay.elements_flow_down(1).elements_to_center();
        }
    }

    /// Removes all falling shapes from the overlay.
    fn clear_shapes(&self) {
        let _ = self.elements.saot_w::<FallingShape<State>, _>(|_s| true);
    }

    /// Steps all falling shapes and removes those that leave the screen.
    fn advance_shapes(&self) {
        let shapes = self.elements.cot::<FallingShape<State>>();
        for shape in &shapes {
            shape.step(GRAVITY, MAX_VELOCITY);
        }

        let height_limit = self.height.get() as isize + 1;
        let _ = self
            .elements
            .saot_w::<FallingShape<State>, _>(|s| s.get_y() > height_limit);
    }

    /// Randomly spawns new falling shapes near the top of the screen.
    fn spawn_shapes(&self) {
        let shape_count = self.elements.cot::<FallingShape<State>>().len();
        if shape_count >= MAX_FALLING_SHAPES {
            return;
        }

        let mut rng = rng();
        if rng.random_range(0..SPAWN_CHANCE) != 0 {
            return;
        }

        let width = self.width.get().max(1);
        let spawn_count = SPAWN_BATCH.min(MAX_FALLING_SHAPES.saturating_sub(shape_count));

        for _ in 0..spawn_count {
            let x = rng.random_range(0..width) as isize;
            let symbol = SYMBOLS[rng.random_range(0..SYMBOLS.len())];
            let color = COLORS[rng.random_range(0..COLORS.len())];
            let velocity = rng.random_range(0.0..0.5);

            let shape = FallingShape::new(FallingShapeOptions {
                color,
                shape: symbol,
                velocity,
            });
            shape.x(x).y(0).focused(false);

            if let Some((label_index, _)) = self.elements.coti::<Label<State>>().first().cloned() {
                let _ = self.elements.push_to(label_index, shape);
            } else {
                self.add(shape);
            }
        }
    }
}

impl Default for Win {
    /// Default constructor for the win overlay element.
    fn default() -> Self {
        Self::new()
    }
}
