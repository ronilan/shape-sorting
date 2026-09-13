#[derive(Clone, PartialEq, Debug, Default)]
pub struct Stats {
    pub last_game_time: u128,
    pub total_time_wasted: u128,
    pub fastest_game: u128,
}

impl Stats {
    /// Returns a finalized snapshot with last_game_time added into total_time_wasted.
    pub fn finalize(mut self) -> Self {
        self.total_time_wasted += self.last_game_time;
        self
    }
}
