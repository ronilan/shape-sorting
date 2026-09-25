use incredible::*;

use crate::core::State;
use crate::core::enums::GameState;
use crate::elements::Timer;

const X: isize = 5;
const Y: isize = 10;

/// Builds the timer element and wires it to game state transitions.
pub(crate) fn build() -> Timer<State> {
    let timer: Timer<State> = Timer::default();
    timer.x(X).y(Y).color(Some(Color::Ansi(5)));

    // Stats only need whole-second precision. Mirror the timer into app state
    // only when that value changes to avoid a board redraw every other loop.
    timer.on_loop(|el, state, _event| {
        let elapsed_time_ms = el.get_elapsed_ms() / 1_000 * 1_000;
        if elapsed_time_ms != state.stats.last_game_time {
            state.stats.last_game_time = elapsed_time_ms;
        }
    });

    // Start/stop/reset the timer based on the game lifecycle.
    timer.on_state(|el, state, _ev| match state.game_state {
        GameState::Splash | GameState::Ready => {
            el.reset();
        }
        GameState::Running => {
            el.start();
        }
        GameState::Paused | GameState::Completed => {
            el.stop();
        }
    });

    timer
}
