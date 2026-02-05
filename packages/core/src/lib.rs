//! # Anatsui Core - Rust/WebAssembly Rendering Engine
//!
//! This is the heart of Anatsui: a high-performance rendering engine written in Rust
//! and compiled to WebAssembly for browser execution. It provides GPU-accelerated
//! rendering using WebGL2 and handles the core document model.
//!
//! ## Architecture Overview
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                    Anatsui Core (Rust)                  │
//! │                                                           │
//! │  ┌─────────────┐  ┌──────────┐  ┌───────────────────┐  │
//! │  │  Document   │  │ Geometry │  │  Renderer         │  │
//! │  │  Tree       │→ │ Network  │→ │  (WebGL2)         │  │
//! │  └─────────────┘  └──────────┘  └───────────────────┘  │
//! │                                                           │
//! │  ┌─────────────┐  ┌──────────┐  ┌───────────────────┐  │
//! │  │  Math       │  │ Tools    │  │  Multiplayer      │  │
//! │  │  Utilities  │  │ System   │  │  Sync (CRDT)      │  │
//! │  └─────────────┘  └──────────┘  └───────────────────┘  │
//! └─────────────────────────────────────────────────────────┘
//!                          ↕ WASM Boundary
//! ┌─────────────────────────────────────────────────────────┐
//! │              React/TypeScript Frontend                   │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Module Organization
//!
//! - **`document`**: Scene graph and object hierarchy (pages, frames, shapes)
//! - **`geometry`**: Bezier paths, vector networks, hit testing
//! - **`renderer`**: WebGL2 context, shaders, batched rendering
//! - **`math`**: 2D transforms, vectors, matrices, bounding boxes
//! - **`tools`**: Drawing tools (pen, rectangle, ellipse, etc.)
//! - **`multiplayer`**: CRDT-based conflict-free collaborative editing
//!
//! ## Key Design Decisions
//!
//! ### Why Rust?
//! - **Performance**: Near-native speed for complex rendering operations
//! - **Safety**: Memory safety without garbage collection pauses
//! - **WebAssembly**: First-class WASM support with minimal overhead
//! - **Type Safety**: Catch bugs at compile time
//!
//! ### Why WebGL2?
//! - **GPU Acceleration**: Offload rendering to the graphics card
//! - **Batching**: Draw thousands of shapes in a single draw call
//! - **Shaders**: Custom rendering effects and filters
//! - **Universal Support**: Available in all modern browsers (2026)
//!
//! ## Usage from JavaScript
//!
//! ```javascript
//! import init, { version } from '@anatsui/wasm';
//!
//! // Initialize the WASM module
//! await init();
//!
//! // Check version
//! console.log('Anatsui Core version:', version());
//! ```
//!
//! ## Building for Production
//!
//! ```bash
//! # Compile to WASM with optimizations
//! wasm-pack build --target web --release
//!
//! # Output goes to ../wasm/pkg/
//! # - anatsui_core_bg.wasm (the binary)
//! # - anatsui_core.js (JS bindings)
//! # - anatsui_core.d.ts (TypeScript types)
//! ```

// Module declarations - these correspond to the folders in src/
pub mod document;   // Document tree: pages, frames, shapes, properties
pub mod geometry;   // Bezier paths, vector networks, hit testing
pub mod math;       // 2D math: Vec2, Transform, Rect, Matrix
pub mod multiplayer; // CRDT-based multiplayer sync
pub mod renderer;   // WebGL2 rendering: shaders, buffers, draw calls
pub mod tools;      // Drawing tools: pen, shape tools, selection

// Re-export commonly used types for convenience
use wasm_bindgen::prelude::*;

/// Set up panic hook for better error messages in the browser console.
/// Without this, Rust panics show as cryptic WASM errors.
/// With this, you get nice stack traces in the devtools.
#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    // This only runs once, even if called multiple times
    console_error_panic_hook::set_once();
}

/// Initialize the Anatsui rendering engine.
///
/// This function MUST be called before using any other functions from this module.
/// It sets up:
/// - Panic hooks for better error messages (in debug builds)
/// - WebGL2 context validation
/// - Global state initialization
///
/// # Example
///
/// ```javascript
/// import init from '@anatsui/wasm';
///
/// // Call this once at app startup
/// await init();
///
/// // Now you can use other functions
/// console.log('Engine ready!');
/// ```
///
/// # Panics
///
/// This function should never panic. If it does, there's a serious bug.
#[wasm_bindgen]
pub fn init() {
    // Enable better error messages in development
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();
    
    // Log to browser console - shows up in DevTools
    web_sys::console::log_1(&"🎨 Anatsui Core initialized".into());
}

/// Get the current version of the Anatsui Core engine.
///
/// The version is read from Cargo.toml at compile time.
/// This is useful for debugging and ensuring the frontend and backend
/// are in sync during development.
///
/// # Returns
///
/// A semantic version string like "0.1.0"
///
/// # Example
///
/// ```javascript
/// import { version } from '@anatsui/wasm';
///
/// console.log(`Using Anatsui Core v${version()}`);
/// // Output: "Using Anatsui Core v0.1.0"
/// ```
#[wasm_bindgen]
pub fn version() -> String {
    // env!() is evaluated at compile time, so no runtime cost
    env!("CARGO_PKG_VERSION").to_string()
}
