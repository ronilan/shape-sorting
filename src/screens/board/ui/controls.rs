use incredible::*;
use incredible_elements::Label;
use incredible_helpers_styling::*;

use crate::core::State;
use crate::core::enums::GameType;

const X: isize = 55;
const Y: isize = 9;

/// Builds the on-screen controls text and wires it to the selected input mode.
pub(crate) fn build() -> Label<State> {
    let controls_text: Label<State> = Label::default();
    controls_text
        .x(X)
        .y(Y)
        .wrap_at(20)
        .color(Some(Color::Ansi(5)))
        .focused(false);

    // Update the help text to match the selected input mode.
    controls_text.on_state(|el, state, _ev| {
        let text = match state.game_type {
            GameType::Mouse => "
🐭 Mode: Drag & Drop shapes to box",
            GameType::Key => {
                "🔑 Mode: Arrows select, ctrl+x pick, ctrl+v place, esc to reset, space to switch, enter go inside."
            }
            GameType::Mix => {
                "🔑 & 🐭  Mode: Drag & Drop -or- Arrows select, ctrl+x & ctrl+v, esc to reset, space to switch, enter go inside."
            }
        };
        el.text(text);
    });

    controls_text
}
