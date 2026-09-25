use incredible::*;
use incredible_elements::{Button, Rectangle};
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;

use crate::core::State;
use crate::core::enums::{GameState, GameType};

const GAME_TYPE_STRINGS: [&str; 3] = ["🔑", "🔑 & 🐭", "🐭"];

/// Builds the input mode selector (Key/Mix/Mouse) as a row of buttons.
pub(crate) fn build() -> Rectangle<State> {
    let selector = Rectangle::new();
    selector.width("🔑🔑 & 🐭🐭".len() + 6).height(3);

    for (i, btn_text) in GAME_TYPE_STRINGS.iter().enumerate() {
        let btn: Button<State> = Button::default();
        btn.width(btn_text.len() + 2)
            .text(btn_text)
            .color(Some(Color::Ansi(6)))
            .focused(false);

        let mode = match i {
            0 => GameType::Key,
            1 => GameType::Mix,
            2 => GameType::Mouse,
            _ => GameType::Mix,
        };

        // Enter selects the input mode and starts the game.
        btn.on_key(move |_el, state, event| {
            if event.key == Key::Enter {
                state.game_state = GameState::Ready;
                state.game_type = mode;
            }
        })
        // Same action for mouse click.
        .on_mouse(move |_el, state, event| {
            if event.mouse == Mouse::Click {
                state.game_state = GameState::Ready;
                state.game_type = mode;
            }
        });

        selector.add(btn);
    }

    selector.elements_flow_right(0);

    selector
}
