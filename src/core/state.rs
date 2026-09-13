use crate::core::{GameState, GameType, PersistedData, Stats};

#[derive(Clone, PartialEq, Debug)]
pub struct State {
    pub game_state: GameState,
    pub game_type: GameType,
    pub stats: Stats,
}

impl Default for State {
    /// Builds initial state with in-memory defaults only.
    /// Persistence loading is injected at app startup.
    fn default() -> Self {
        Self {
            game_state: GameState::Splash,
            stats: Stats::default(),
            game_type: GameType::Mix,
        }
    }
}

impl State {
    pub fn from_persisted(data: PersistedData) -> Self {
        Self {
            game_state: GameState::Splash,
            stats: data.stats,
            game_type: data.game_type,
        }
    }

    pub fn to_persisted(&self) -> PersistedData {
        PersistedData {
            stats: self.stats.clone(),
            game_type: self.game_type,
        }
    }
}
