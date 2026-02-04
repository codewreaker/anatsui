//! Vector geometry and vector networks
//!
//! Implements vector networks as described in Figma's blog post.
//! Unlike traditional paths (sequences of points), vector networks allow
//! any point to connect to any other point.

mod network;
mod point;
mod segment;

pub use network::*;
pub use point::*;
pub use segment::*;

use wasm_bindgen::prelude::*;
