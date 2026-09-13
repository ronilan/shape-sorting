use incredible::*;
use incredible_elements::TextButton;
use incredible_helpers_styling::*;

use crate::core::State;
use crate::core::enums::GameState;

const X: isize = 73;
const Y: isize = 22;

/// Builds the Reset button (restarts the current game).
pub(crate) fn build() -> TextButton<State> {
    let reset_button: TextButton<State> = TextButton::default();
    reset_button.text("Reset").color(Some(Color::Ansi(6)));
    reset_button.x(X).y(Y).showed(false).focused(false);

    // Reset button: restart the game without leaving the board.
    // TODO: the action of both key and mouse is identical - set the same to ready however
    // containing element also has on_key that sets game to running.
    reset_button
        .on_key(|_el, state, event| {
            if event.key == Key::Enter {
                state.stats.total_time_wasted += state.stats.last_game_time;
                state.stats.last_game_time = 0;
                state.game_state = GameState::Ready;
            }
        })
        // Same action for mouse click.
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.stats.total_time_wasted += state.stats.last_game_time;
                state.stats.last_game_time = 0;
                state.game_state = GameState::Ready;
            }
        })
        // Only show the reset button during active or completed play.
        .on_state(|el, state, _ev| {
            el.showed(
                state.game_state == GameState::Running || state.game_state == GameState::Completed,
            );
        });

    reset_button
}
