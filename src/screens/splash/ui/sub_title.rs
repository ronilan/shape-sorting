use incredible::*;
use incredible_elements::Label;
use incredible_helpers_styling::*;

use crate::core::State;

/// Builds the subtitle element for the splash screen.
pub(crate) fn build() -> Label<State> {
    let sub_title = Label::default();
    sub_title
        .text(
            "Using 🔑 vs. 🐭 
     Which is faster?",
        )
        .color(Some(Color::Ansi(11)))
        .focused(false);
    sub_title
}
