use incredible::*;
use incredible_elements::Rectangle;

use crate::core::State;

/// Builds a transparent full-screen overlay rectangle.
pub(crate) fn build() -> Rectangle<State> {
    let width = Platform::columns();
    let height = Platform::rows();

    let overlay: Rectangle<State> = Rectangle::new();
    overlay
        .width(width)
        .height(height)
        .fill(None)
        .focused(false);
    overlay
}
