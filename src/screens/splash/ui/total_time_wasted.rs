use incredible::*;
use incredible_elements::Label;
use incredible_helpers_styling::*;

use crate::core::State;

/// Builds the "total wasted time" text element.
pub(crate) fn build() -> Label<State> {
    let total_time_wasted_text: Label<State> = Label::default();
    total_time_wasted_text
        .text("Total wasted time: 00:00:00")
        .color(Some(Color::Ansi(9)))
        .faint(Some(true))
        .focused(false);

    // Format and display the total time across all games.
    total_time_wasted_text.on_state(|el, state, _ev| {
        let ms = state.stats.total_time_wasted;
        let total_seconds = ms / 1_000;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        el.text(
            format!(
                "Total wasted time: {:02}:{:02}:{:02}",
                hours, minutes, seconds
            )
            .as_str(),
        );
    });

    total_time_wasted_text
}
