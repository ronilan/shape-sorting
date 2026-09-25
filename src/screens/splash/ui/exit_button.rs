use incredible::*;
use incredible_elements::TextButton;
use incredible_helpers_styling::*;

use crate::core::State;

const X: isize = 74;
const Y: isize = 1;

/// Builds the Exit button for the splash screen.
pub(crate) fn build() -> TextButton<State> {
    let exit_button: TextButton<State> = TextButton::default();

    exit_button
        .x(X)
        .y(Y)
        .text("Exit")
        .color(Some(Color::Ansi(6)))
        .focused(false);

    // Exit the app from the splash screen.
    exit_button
        .on_key(|_el, _state: &mut State, event| {
            if event.key == Key::Enter {
                exit(0);
            }
        })
        // Same action for mouse click.
        .on_mouse(|_el, _state: &mut State, event| {
            if event.mouse == Mouse::Click {
                exit(0);
            }
        });

    exit_button
}
