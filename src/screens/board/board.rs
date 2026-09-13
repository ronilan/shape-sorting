use std::cell::Cell;

use incredible::*;
use incredible_elements::{Frame, TextButton};
use incredible_macros_decl::element_for;

use crate::core::State;
use crate::core::enums::GameState;
use crate::game::ShapeSorting;
use crate::screens::board::ui;

element_for! {
  pub struct Board for State {
      selected_index: Cell<Option<usize>> = Cell::new(None)
  }
}

impl Board {
    /// Builds the main game board UI and wires all handlers.
    pub fn new() -> Self {
        let el = Self::blank();

        el.look(Look::from((80, 24, ' ')));

        let frame: Frame<State> = ui::frame::build();

        let shape_sorting = ShapeSorting::new();

        let timer = ui::timer::build();
        let controls_text = ui::controls::build();
        let exit_button: TextButton<State> = ui::exit_button::build();
        let reset_button: TextButton<State> = ui::reset_button::build();

        // Element are fixed positioned.
        frame.add(shape_sorting);
        frame.add(timer);
        frame.add(reset_button);
        frame.add(exit_button);
        frame.add(controls_text);

        el.add(frame);

        // Setting showed to false on the fully composed element
        // allows setter to act on sub elements.
        el.showed(false);

        el
            // Tab cycles focus between board-level buttons (Back/Reset).
            .internal_on_key(|el, _state, event| {
                if event.key == Key::Tab {
                    let text_btns = el.elements.dcot::<TextButton<State>>();

                    let total = text_btns.len();
                    if total == 0 {
                        return;
                    }

                    let next = match el.selected_index.get() {
                        Some(idx) => (idx + 1) % total,
                        None => 0,
                    };

                    el.selected_index.set(Some(next));

                    let selected = el.selected_index.get();
                    let mut i = 0;

                    for btn in text_btns {
                        btn.focused(Some(i) == selected);
                        i += 1;
                    }
                }

                el.draw();
            })
            .internal_on_state(|el, state, _ev| {
                el.showed(
                    state.game_state != GameState::Splash
                        && state.game_state != GameState::Completed,
                );
                // TODO - because the timer updates on loop - the board does not need draw override.
                el.draw();
            });

        el
    }
}

impl Default for Board {
    /// Default constructor for the board element.
    fn default() -> Self {
        Self::new()
    }
}
