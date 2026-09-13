use std::cell::Cell;

use incredible::*;
use incredible_elements::{Button, Frame, TextButton};
use incredible_helpers_layout::*;
use incredible_macros_decl::element_for;

use crate::core::State;
use crate::core::enums::GameState;
use crate::screens::splash::ui;

element_for! {
  pub struct Splash for State {
      selected_index: Cell<Option<usize>> = Cell::new(None)
  }
}

impl Splash {
    /// Builds the splash screen UI and wires all handlers.
    pub fn new() -> Self {
        let el = Self::blank();

        el.look(Look::from((80, 24, ' ')));

        let frame: Frame<State> = ui::frame::build();

        let name = ui::title::build();
        let sub_title = ui::sub_title::build();
        let selector = ui::selector::build();
        let fastest_game_text = ui::fastest_game::build();
        let desc = ui::desc::build();
        let total_time_wasted_text = ui::total_time_wasted::build();

        // Elements are auto positioned according to order
        frame.add(name);
        frame.add(sub_title);
        frame.add(selector);
        frame.add(fastest_game_text);
        frame.add(desc);
        frame.add(total_time_wasted_text);

        frame
            .elements_flow_down(1) // Align is Start by default
            .elements_snap_center_x() // Equivalent to setting Align Center
            .elements_to_center_y(); // All to center

        // Element is fixed positioned.
        let exit_button = ui::exit_button::build();
        frame.add(exit_button);

        el.add(frame);

        el
            // Tab cycles focus between the mode buttons and the Exit button.
            .internal_on_key(|el, _state, event| {
                if event.key == Key::Tab {
                    // get selector (Rectangle)

                    let btns = el.elements.dcot::<Button<State>>();
                    let text_btns = el.elements.dcot::<TextButton<State>>();

                    let total = btns.len() + text_btns.len();
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

                    for btn in btns {
                        btn.focused(Some(i) == selected);
                        i += 1;
                    }

                    for btn in text_btns {
                        btn.focused(Some(i) == selected);
                        i += 1;
                    }
                }

                el.draw();
            })
            // Only show splash elements while in Splash state.
            .internal_on_state(|el, state, _ev| {
                el.showed(state.game_state == GameState::Splash);
                el.draw();
            });

        el
    }
}

impl Default for Splash {
    /// Default constructor for the splash element.
    fn default() -> Self {
        Self::new()
    }
}
