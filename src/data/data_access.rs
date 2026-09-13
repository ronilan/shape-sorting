use crate::core::{GameType, PersistedData, Persistence, Stats};
use std::fs;
use std::path::PathBuf;

const DATA_DIR: &str = "shape_sorting";
const DATA_FILE: &str = ".shape_sorting";

/// Per-user data file location, following platform conventions
/// (macOS: ~/Library/Application Support, Linux: $XDG_DATA_HOME,
/// Windows: %APPDATA%). `None` when no base data directory can be
/// determined (e.g. HOME is unset).
#[cfg(not(target_arch = "wasm32"))]
fn data_file_path() -> Option<PathBuf> {
    Some(dirs::data_dir()?.join(DATA_DIR).join(DATA_FILE))
}

/// Native file persistence does not exist on the web — the WASM build uses
/// `LocalStoragePersistence` — so there is never a data file path.
#[cfg(target_arch = "wasm32")]
fn data_file_path() -> Option<PathBuf> {
    None
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FilePersistence;

impl FilePersistence {
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
            // Backward compatibility: old format had only total and fastest.
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
            // Backward compatibility: very old format had only total.
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

impl Persistence for FilePersistence {
    fn load(&self) -> PersistedData {
        match data_file_path().and_then(|path| fs::read_to_string(path).ok()) {
            Some(content) => Self::parse_content(&content),
            None => PersistedData::default(),
        }
    }

    fn save(&self, data: &PersistedData) {
        let Some(path) = data_file_path() else { return };
        let game_type_str = match data.game_type {
            GameType::Mouse => "Mouse",
            GameType::Key => "Key",
            GameType::Mix => "Mix",
        };
        let content = format!(
            "{},{},{}",
            data.stats.total_time_wasted, data.stats.fastest_game, game_type_str
        );
        if let Some(parent) = path.parent() {
            if fs::create_dir_all(parent).is_err() {
                return;
            }
        }
        let _ = fs::write(&path, content);
    }
}
