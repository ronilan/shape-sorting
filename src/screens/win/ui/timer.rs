use incredible::*;

use crate::core::State;
use crate::elements::{Timer, TimerOptions};

/// Builds the win-screen timer element.
pub(crate) fn build() -> Timer<State> {
    let timer = Timer::new(TimerOptions { show_ms: false });
    timer.focused(false);
    timer
}
