//! Multiplayer messages

use crate::document::{ObjectId, Property, PropertyValue};
use crate::multiplayer::ClientId;
use serde::{Deserialize, Serialize};

/// Messages sent between client and server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    /// Client joining a document
    Join {
        document_id: String,
        client_name: String,
    },
    /// Server acknowledging join with client ID
    JoinAck {
        client_id: u32,
        document_state: String, // JSON-serialized document
    },
    /// Client leaving
    Leave {
        client_id: u32,
    },
    /// Cursor position update
    CursorMove {
        client_id: u32,
        x: f32,
        y: f32,
    },
    /// Property change
    PropertyChange {
        client_id: u32,
        object_id: ObjectId,
        property: Property,
        value: String, // JSON-serialized PropertyValue
        sequence: u64,
    },
    /// Create a new object
    CreateObject {
        client_id: u32,
        object_id: ObjectId,
        object_type: String,
        parent_id: ObjectId,
        order_index: String, // Fractional index
        sequence: u64,
    },
    /// Delete an object
    DeleteObject {
        client_id: u32,
        object_id: ObjectId,
        sequence: u64,
    },
    /// Move an object (reparent)
    MoveObject {
        client_id: u32,
        object_id: ObjectId,
        new_parent_id: ObjectId,
        order_index: String,
        sequence: u64,
    },
    /// Server acknowledging a change
    Ack {
        sequence: u64,
    },
    /// Selection change
    SelectionChange {
        client_id: u32,
        selected_ids: Vec<ObjectId>,
    },
    /// Error message
    Error {
        code: u32,
        message: String,
    },
    /// Heartbeat/ping
    Ping,
    /// Heartbeat response
    Pong,
}

impl Message {
    /// Serialize to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
}
