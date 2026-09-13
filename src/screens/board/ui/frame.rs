use incredible_elements::{Frame, FrameKind};
use incredible_helpers_styling::*;

use crate::core::State;

/// Builds the Frame for the board screen.
pub(crate) fn build() -> Frame<State> {
    let frame: Frame<State> = Frame::default();

    frame
        .width(80)
        .height(24)
        .faint(Some(true))
        .kind(Some(FrameKind::Dotted));

    frame
}
