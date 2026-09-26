use incredible::*;
use incredible_elements::{App, AppOptions};
use incredible_helpers_layout::*;

use crate::core::State;
use crate::core::enums::GameState;
use crate::screens::{Board, Splash, Win};

/// Builds the application root and wires global handlers.
pub(crate) fn build() -> App<State> {
    let app = App::new(AppOptions::default());
    // Disabling keyboard exit
    app.pre_exit_combination(Some(KeyCombination::new(Key::Char('d'), &[KeyMod::Ctrl])))
        .exit_combination(Some(KeyCombination::new(Key::Escape, &[])))
        .auto_surface(false);

    // Window events: keep layout centered on resize and pause/resume on focus changes.
    app.on_window(move |el, state: &mut State, event| {
        if event.window == Window::Resize {
            // auto_surface is off - take responsability for app layout and draw.
            el.look(Look::from((event.columns, event.rows)));
            el.elements_snap_center();
            el.draw();
        }
        if event.window == Window::Focus {
            match state.game_state {
                GameState::Running if !event.focus => {
                    state.game_state = GameState::Paused;
                }
                GameState::Paused if event.focus => {
                    state.game_state = GameState::Running;
                }
                _ => {}
            }
        }
    })
    // State changes: redraw when entering Splash so the screen updates immediately.
    .on_state(|el, state, _ev| {
        if state.game_state == GameState::Splash {
            el.draw();
        }
    });

    let board = Board::default();
    let splash = Splash::default();
    let win = Win::default();

    app.add(board);
    app.add(splash);
    app.add(win);

    app
}
