use incredible::*;
use incredible_elements::Label;
use incredible_helpers_styling::*;

use crate::core::State;

/// Builds the "Completed!" text element.
pub(crate) fn build() -> Label<State> {
    let label: Label<State> = Label::default();
    label
        .text("Completed!")
        .color(Some(Color::Ansi(148)))
        .focused(false);
    label
}
