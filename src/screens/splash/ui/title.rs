use incredible::*;
use incredible_elements_text_fonts::FancyStr;
use incredible_helpers_styling::*;

use crate::core::State;

/// Builds the main title element for the splash screen.
pub(crate) fn build() -> FancyStr<State> {
    let name = FancyStr::default();
    name.text("Shape Sorting").color(Some(Color::Ansi(2)));

    name
}
