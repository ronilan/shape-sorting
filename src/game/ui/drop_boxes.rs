use incredible::*;
use incredible_elements::{Frame, FrameKind};
use incredible_helpers_styling::*;

use crate::core::State;
use crate::core::constants::{COLORS, EMPTY_SYMBOLS};

const X: isize = 24;
const Y: isize = 1;

/// Builds the 16 drop box frames (8 color, 8 shape) for the board.
pub(crate) fn build() -> Vec<Frame<State>> {
    let mut frames = Vec::new();

    for i in 0..16 {
        let frame = if i < 8 {
            let color = COLORS[i];
            let f = Frame::new();
            f.x(X + (i as isize * 4))
                .y(Y)
                .width(4)
                .height(4)
                .fill(Some('.'))
                .color(Some(Color::Ansi(color)))
                .focused(false);
            f
        } else {
            let symbol = EMPTY_SYMBOLS[i - 8];
            let f = Frame::new();
            f.x(X + ((i - 8) as isize * 4))
                .y(Y + 18)
                .width(4)
                .height(4)
                .fill(Some(symbol))
                .color(None)
                .focused(false);
            f
        };

        frame.frame_style.focused.kind.set(Some(FrameKind::Double));

        frames.push(frame);
    }

    frames
}
