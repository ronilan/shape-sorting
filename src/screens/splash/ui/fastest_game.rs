use incredible::*;
use incredible_elements::Label;
use incredible_helpers_styling::*;

use crate::core::State;
use crate::core::enums::GameType;

const GAME_TYPE_STRINGS: [&str; 3] = ["🔑", "Mix", "🐭"];

/// Builds the "fastest game" text element.
pub(crate) fn build() -> Label<State> {
    let fastest_game_text: Label<State> = Label::default();
    fastest_game_text
        .text("Fastest game: 00:00:00")
        .color(Some(Color::Ansi(10)))
        .focused(false);

    // Format and display the fastest game time for the current mode.
    fastest_game_text.on_state(move |el, state, _ev| {
        let ms = state.stats.fastest_game;
        let game_type_str = match state.game_type {
            GameType::Key => GAME_TYPE_STRINGS[0],
            GameType::Mix => GAME_TYPE_STRINGS[1],
            GameType::Mouse => GAME_TYPE_STRINGS[2],
        };

        if ms == 0 {
            el.text(" Fastest game: Never!   ");
        } else {
            let total_seconds = ms / 1_000;
            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            let seconds = total_seconds % 60;
            el.text(
                format!(
                    "Fastest game: {:02}:{:02}.{:02} {}",
                    hours, minutes, seconds, game_type_str
                )
                .as_str(),
            );
        }
    });

    fastest_game_text
}
