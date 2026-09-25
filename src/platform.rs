use crate::core::Persistence;
#[cfg(feature = "macos-native")]
use incredible_window_macos::{hex, register_palette, remove_palette};

#[cfg(feature = "windows-native")]
use incredible_window_windows::{hex, register_palette, remove_palette};

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::data::FilePersistence as PlatformPersistence;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::data::LocalStoragePersistence as PlatformPersistence;

pub(crate) fn init() {
    #[cfg(any(feature = "macos-native", feature = "windows-native"))]
    register_palettes();

    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}

#[cfg(any(feature = "macos-native", feature = "windows-native"))]
fn register_palettes() {
    remove_palette("Light");

    register_palette(
        "Solarized Dark",
        hex("#839496"),
        hex("#002b36"),
        [
            hex("#073642"), // 0
            hex("#dc322f"), // 1
            hex("#859900"), // 2
            hex("#b58900"), // 3
            hex("#268bd2"), // 4
            hex("#d33682"), // 5
            hex("#2aa198"), // 6
            hex("#eee8d5"), // 7
            hex("#002b36"), // 8
            hex("#cb4b16"), // 9
            hex("#586e75"), // 10
            hex("#657b83"), // 11
            hex("#839496"), // 12
            hex("#6c71c4"), // 13
            hex("#93a1a1"), // 14
            hex("#fdf6e3"), // 15
        ],
        false,
    );

    register_palette(
        "Solarized Light",
        hex("#657b83"),
        hex("#fdf6e3"),
        [
            hex("#eee8d5"), // 0
            hex("#dc322f"), // 1
            hex("#859900"), // 2
            hex("#b58900"), // 3
            hex("#268bd2"), // 4
            hex("#d33682"), // 5
            hex("#2aa198"), // 6
            hex("#073642"), // 7
            hex("#fdf6e3"), // 8
            hex("#cb4b16"), // 9
            hex("#586e75"), // 10
            hex("#657b83"), // 11
            hex("#839496"), // 12
            hex("#6c71c4"), // 13
            hex("#93a1a1"), // 14
            hex("#002b36"), // 15
        ],
        false,
    );
}

fn persistence() -> PlatformPersistence {
    PlatformPersistence::default()
}

pub(crate) fn load_state() -> crate::core::State {
    let persistence = persistence();
    crate::core::State::from_persisted(persistence.load())
}

pub(crate) fn save_state(final_state: &crate::core::State) {
    let persistence = persistence();
    let mut persisted = final_state.to_persisted();
    persisted.stats = persisted.stats.finalize();
    persistence.save(&persisted);
}
