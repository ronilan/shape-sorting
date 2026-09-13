use crate::{app, core::State, platform};

pub(crate) fn run() -> incredible::tui::DeferredValue<State> {
    platform::init();

    let state = platform::load_state();
    let app = app::build();

    let deferred = app.run(state);

    deferred.on_set(|final_state| {
        platform::save_state(&final_state);
    });

    deferred
}
