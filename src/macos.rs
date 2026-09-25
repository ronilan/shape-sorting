#[cfg(feature = "macos-native")]
mod core;
#[cfg(feature = "macos-native")]
mod data;
#[cfg(feature = "macos-native")]
mod elements;
#[cfg(feature = "macos-native")]
mod game;
#[cfg(feature = "macos-native")]
mod platform;
#[cfg(feature = "macos-native")]
mod runtime;
#[cfg(feature = "macos-native")]
mod screens;
#[cfg(feature = "macos-native")]
mod ui;

#[cfg(feature = "macos-native")]
fn main() {
    incredible_window_macos::set_window_title(option_env!("APP_NAME").unwrap_or("Shape Sorting"));
    platform::init();
    incredible_window_macos::run_app(runtime::run);
}

#[cfg(not(feature = "macos-native"))]
fn main() {
    eprintln!("This binary is only available for macOS targets.");
}
