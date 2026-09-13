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

    // Mirror timer's elapsed milliseconds into app state for stats.
    timer.on_loop(|el, state, event| {
        // TODO - don't really need "sub second stats"
        // currently it drives board over rendering
        if event.loop_count % 2 == 0 {
            state.stats.last_game_time = el.get_elapsed_ms();
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
