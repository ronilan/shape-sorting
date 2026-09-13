use incredible::*;
use incredible_elements::Label;
use incredible_helpers_styling::*;

use crate::core::State;

/// Builds the description text shown on the splash screen.
pub(crate) fn build() -> Label<State> {
    let desc = Label::default();
    desc
        .text("Warning: This activity is an utter total waste of time that is not even remotely enjoyable. Like, seriously.")
        .wrap_at(30)
        .color(Some(Color::Ansi(7)))
        .faint(Some(true))
        .focused(false);
    desc
}
