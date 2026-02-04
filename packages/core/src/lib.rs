//! # Anatsui Core
//!
//! High-performance Rust rendering engine for the Anatsui design tool.
//! Compiled to WebAssembly for browser execution.

pub mod document;
pub mod geometry;
pub mod math;
pub mod multiplayer;
pub mod renderer;
pub mod tools;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Initialize the Anatsui engine
#[wasm_bindgen]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();
    
    web_sys::console::log_1(&"🎨 Anatsui Core initialized".into());
}

/// Get the version of the Anatsui Core
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
