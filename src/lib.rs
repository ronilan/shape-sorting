mod core;
mod data;
mod elements;
mod game;
mod platform;
mod runtime;
mod screens;
mod ui;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    runtime::run();
}
