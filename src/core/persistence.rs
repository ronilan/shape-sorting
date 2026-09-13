use crate::core::{GameType, Stats};

#[derive(Clone, Debug)]
pub struct PersistedData {
    pub stats: Stats,
    pub game_type: GameType,
}

impl Default for PersistedData {
    fn default() -> Self {
        Self {
            stats: Stats::default(),
            game_type: GameType::Mix,
        }
    }
}

pub trait Persistence {
    fn load(&self) -> PersistedData;
    fn save(&self, data: &PersistedData);
}
