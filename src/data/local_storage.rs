use crate::core::{GameType, PersistedData, Persistence, Stats};

const STORAGE_KEY: &str = ".shape_sorting";

#[derive(Clone, Copy, Debug)]
pub struct LocalStoragePersistence {
    key: &'static str,
}

impl Default for LocalStoragePersistence {
    fn default() -> Self {
        Self { key: STORAGE_KEY }
    }
}

impl LocalStoragePersistence {
    fn parse_content(content: &str) -> PersistedData {
        let parts: Vec<&str> = content.trim().split(',').collect();
        if parts.len() == 3 {
            let total = parts[0].parse::<u128>().unwrap_or(0);
            let fastest = parts[1].parse::<u128>().unwrap_or(0);
            let game_type = match parts[2] {
                "Mouse" => GameType::Mouse,
                "Key" => GameType::Key,
                "Mix" => GameType::Mix,
                _ => GameType::Mix,
            };

            PersistedData {
                stats: Stats {
                    last_game_time: 0,
                    total_time_wasted: total,
                    fastest_game: fastest,
                },
                game_type,
            }
        } else if parts.len() == 2 {
            let total = parts[0].parse::<u128>().unwrap_or(0);
            let fastest = parts[1].parse::<u128>().unwrap_or(0);
            PersistedData {
                stats: Stats {
                    last_game_time: 0,
                    total_time_wasted: total,
                    fastest_game: fastest,
                },
                game_type: GameType::Mix,
            }
        } else if parts.len() == 1 {
            let total = parts[0].parse::<u128>().unwrap_or(0);
            PersistedData {
                stats: Stats {
                    last_game_time: 0,
                    total_time_wasted: total,
                    fastest_game: 0,
                },
                game_type: GameType::Mix,
            }
        } else {
            PersistedData::default()
        }
    }
}

impl Persistence for LocalStoragePersistence {
    fn load(&self) -> PersistedData {
        web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|s| s.get_item(self.key).ok().flatten())
            .map(|content| Self::parse_content(&content))
            .unwrap_or_default()
    }

    fn save(&self, data: &PersistedData) {
        let game_type_str = match data.game_type {
            GameType::Mouse => "Mouse",
            GameType::Key => "Key",
            GameType::Mix => "Mix",
        };

        let content = format!(
            "{},{},{}",
            data.stats.total_time_wasted, data.stats.fastest_game, game_type_str
        );

        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item(self.key, &content);
        }
    }
}
