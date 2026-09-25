use incredible::*;
use incredible_elements::{Frame, FrameKind};
use incredible_helpers_styling::*;

use crate::core::State;
use crate::core::enums::{GameState, GameType};

const X: isize = 31;
const Y: isize = 9;

/// Builds the shape box frame where draggable shapes start.
pub(crate) fn build() -> Frame<State> {
    let shape_box: Frame<State> = Frame::new();
    shape_box
        .x(X)
        .y(Y)
        .width(18)
        .height(6)
        .fill(Some('.'))
        .color(None)
        .focused(false);

    shape_box
        .frame_style
        .focused
        .kind
        .set(Some(FrameKind::Double));

    // Mouse click inside the shape box starts the game timer.
    shape_box.on_mouse(|_el, state: &mut State, event| {
        if event.mouse == Mouse::Down
            && state.game_state == GameState::Ready
            && state.game_type != GameType::Key
        {
            state.game_state = GameState::Running;
        }
    });

    shape_box
}
