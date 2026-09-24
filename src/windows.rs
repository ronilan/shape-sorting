#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(all(target_os = "windows", feature = "windows-native"))]
mod core;
#[cfg(all(target_os = "windows", feature = "windows-native"))]
mod data;
#[cfg(all(target_os = "windows", feature = "windows-native"))]
mod elements;
#[cfg(all(target_os = "windows", feature = "windows-native"))]
mod game;
#[cfg(all(target_os = "windows", feature = "windows-native"))]
mod platform;
#[cfg(all(target_os = "windows", feature = "windows-native"))]
mod runtime;
#[cfg(all(target_os = "windows", feature = "windows-native"))]
mod screens;
#[cfg(all(target_os = "windows", feature = "windows-native"))]
mod ui;

#[cfg(all(target_os = "windows", feature = "windows-native"))]
fn main() {
    incredible_window_windows::set_window_title(option_env!("APP_NAME").unwrap_or("Shape Sorting"));
    platform::init();
    incredible_window_windows::run_app(runtime::run);
}

#[cfg(any(not(target_os = "windows"), not(feature = "windows-native")))]
fn main() {
    eprintln!(
        "This binary is only available for windows targets with the 'windows-native' feature enabled."
    );
}
