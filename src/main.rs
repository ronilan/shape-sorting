mod app;
mod core;
mod data;
mod elements;
mod game;
mod platform;
mod runtime;
mod screens;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "macos-native")))]
fn main() {
    runtime::run();
}

#[cfg(all(not(target_arch = "wasm32"), feature = "macos-native"))]
fn main() {
    eprintln!(
        "The 'shape_sorting' binary is for Terminal. Use 'shape_sorting_macos' for the macOS version, or build without the 'macos-native' feature."
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {}
