use std::cell::Cell;

use incredible::*;
use incredible_elements_text_fonts::FancyStr;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

// -----------------------------
// Options
// -----------------------------
#[derive(Clone, Debug, Default)]
pub struct TimerOptions {
    pub show_ms: bool,
}

// -----------------------------
// State (user-facing data)
// -----------------------------
#[derive(Clone, Debug, Default)]
pub struct TimerState {
    pub elapsed_ms: Cell<u128>,
}

// -----------------------------
// Timer Element
// -----------------------------
element! {
  pub struct Timer<S> {
      internal_state: TimerState = TimerState::default(),
      options: TimerOptions = TimerOptions::default(),

      // Implementation details
      start: Cell<Option<f64>> = Cell::new(None),
      accumulated_ms: Cell<u128> = Cell::new(0),
      running: Cell<bool> = Cell::new(false),
  }
}

impl<S: Clone + PartialEq> Timer<S> {
    /// Constructs a timer element with the provided display options.
    pub fn new(options: TimerOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        let display = FancyStr::default();
        display
            .text(Self::format_time(0, el.options.show_ms).as_str())
            .focused(false);

        el.look(Look::from((display.get_width(), display.get_height())));

        // Refresh the on-screen timer every loop tick.
        // TODO - this triggers collection across the board
        el.internal_on_loop(|el, _, _| {
            el.update_display();
        });

        el.add(display);

        el
    }

    /// Updates internal elapsed time and refreshes the display text.
    pub fn update_display(&self) {
        let elapsed = if self.running.get() {
            let start = self.start.get().unwrap_or_else(Globals::now);
            let delta_ms = (Globals::now() - start) as u128;
            self.accumulated_ms.get() + delta_ms
        } else {
            self.accumulated_ms.get()
        };

        // Update state
        self.internal_state.elapsed_ms.set(elapsed);

        if let Some(display) = self.elements.cot::<FancyStr<S>>().first() {
            display.text(Self::format_time(elapsed, self.options.show_ms).as_str());
        }
        // TODO - don't really need to draw if not showing ms
        self.draw();
    }

    /// Sets the display color for the timer text.
    pub fn color(&self, value: Option<Color>) -> &Self {
        if let Some(display) = self.elements.cot::<FancyStr<S>>().first() {
            display.color(value);
        }

        self
    }

    /// Starts the timer if it is not already running.
    pub fn start(&self) {
        if !self.running.get() {
            self.start.set(Some(Globals::now()));
            self.running.set(true);
        }
    }

    /// Stops the timer and freezes the elapsed time.
    pub fn stop(&self) {
        if self.running.get() {
            let start = self.start.get().unwrap_or_else(Globals::now);
            let delta_ms = (Globals::now() - start) as u128;
            let elapsed = self.accumulated_ms.get() + delta_ms;
            self.accumulated_ms.set(elapsed);
            self.running.set(false);

            // Update state
            self.internal_state.elapsed_ms.set(elapsed);

            if let Some(display) = self.elements.cot::<FancyStr<S>>().first() {
                display.text(Self::format_time(elapsed, self.options.show_ms).as_str());
            }

            self.draw();
        }
    }

    /// Resets the timer back to zero.
    pub fn reset(&self) {
        self.accumulated_ms.set(0);
        self.running.set(false);
        self.start.set(None);

        // Update state
        self.internal_state.elapsed_ms.set(0);

        if let Some(display) = self.elements.cot::<FancyStr<S>>().first() {
            display.text(Self::format_time(0, self.options.show_ms).as_str());
        }

        self.draw();
    }

    /// Sets the elapsed time explicitly and refreshes the display.
    pub fn set_elapsed_ms(&self, ms: u128) -> &Self {
        self.accumulated_ms.set(ms);
        self.running.set(false);
        self.start.set(None);

        // Update state
        self.internal_state.elapsed_ms.set(ms);

        if let Some(display) = self.elements.cot::<FancyStr<S>>().first() {
            display.text(Self::format_time(ms, self.options.show_ms).as_str());
        }

        self.draw();

        self
    }

    /// Formats milliseconds into `MM:SS` or `MM:SS.cc` depending on options.
    fn format_time(ms: u128, show_ms: bool) -> String {
        let minutes = ms / 60_000;
        let seconds = (ms % 60_000) / 1_000;

        if show_ms {
            let centiseconds = (ms % 1_000) / 10;
            format!("{:02}:{:02}.{:02}", minutes, seconds, centiseconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }

    // Getter for TimerState field

    /// Gets the elapsed time in milliseconds.
    pub fn get_elapsed_ms(&self) -> u128 {
        self.internal_state.elapsed_ms.get()
    }
}

impl<S: Clone + PartialEq> Default for Timer<S> {
    /// Default constructor for the timer element.
    fn default() -> Self {
        Self::new(TimerOptions::default())
    }
}
