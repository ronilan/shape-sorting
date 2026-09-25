use incredible::*;
use incredible_elements::Label;
use incredible_helpers_styling::*;

use crate::core::State;

const X: isize = 65;
const Y: isize = 21;

/// Builds the Tab focus hint for the splash screen.
pub(crate) fn build() -> Label<State> {
    let tab_focus = Label::default();
    tab_focus
        .x(X)
        .y(Y)
        .text("Tab focus,\n^D + Esc Exit")
        .color(Some(Color::Ansi(5)))
        .focused(false);

    tab_focus
}
