//! Multiplayer collaboration module
//!
//! Implements real-time collaboration inspired by Figma's approach:
//! - Property-level syncing (last-writer-wins)
//! - Fractional indexing for ordering
//! - Client-server architecture

mod message;
mod sync;

pub use message::*;
pub use sync::*;

use wasm_bindgen::prelude::*;
use crate::document::ObjectId;

/// Unique identifier for a connected client
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientId(u32);

#[wasm_bindgen]
impl ClientId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

/// Cursor position of another user
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct UserCursor {
    client_id: ClientId,
    name: String,
    color: String,
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl UserCursor {
    #[wasm_bindgen(constructor)]
    pub fn new(client_id: ClientId, name: &str, color: &str) -> Self {
        Self {
            client_id,
            name: name.to_string(),
            color: color.to_string(),
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn client_id(&self) -> ClientId {
        self.client_id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn color(&self) -> String {
        self.color.clone()
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

/// Colors for different users
const USER_COLORS: [&str; 8] = [
    "#F24E1E", // Red
    "#A259FF", // Purple
    "#1ABCFE", // Blue
    "#0ACF83", // Green
    "#FF7262", // Orange
    "#FFC700", // Yellow
    "#00C2FF", // Cyan
    "#C7B9FF", // Lavender
];

/// Get a color for a user based on their client ID
#[wasm_bindgen]
pub fn get_user_color(client_id: ClientId) -> String {
    USER_COLORS[client_id.0 as usize % USER_COLORS.len()].to_string()
}
