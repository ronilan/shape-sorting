use incredible::*;
use incredible_elements::TextButton;
use incredible_helpers_styling::*;

use crate::core::State;
use crate::core::enums::GameState;

const X: isize = 74;
const Y: isize = 1;

/// Builds the Back button (returns to Splash and records time).
pub(crate) fn build() -> TextButton<State> {
    let exit_button: TextButton<State> = TextButton::default();
    exit_button.text("Back").color(Some(Color::Ansi(6)));
    exit_button.x(X).y(Y).focused(false);

    // Back button: return to Splash and record time.
    exit_button
        .on_key(|_el, state: &mut State, event| {
            if event.key == Key::Enter {
                state.stats.total_time_wasted += state.stats.last_game_time;
                state.stats.last_game_time = 0;
                state.game_state = GameState::Splash;
            }
        })
        // Same action for mouse click.
        .on_mouse(|_el, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                state.stats.total_time_wasted += state.stats.last_game_time;
                state.stats.last_game_time = 0;
                state.game_state = GameState::Splash;
            }
        });

    exit_button
}
