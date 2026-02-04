//! Sync engine for multiplayer collaboration

use crate::document::{Document, ObjectId, Property, PropertyValue};
use crate::multiplayer::{ClientId, Message, UserCursor};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// Pending change waiting for acknowledgement
#[derive(Debug, Clone)]
struct PendingChange {
    object_id: ObjectId,
    property: Property,
    value: PropertyValue,
    sequence: u64,
}

/// Sync engine manages multiplayer state
#[wasm_bindgen]
pub struct SyncEngine {
    client_id: Option<ClientId>,
    sequence: u64,
    pending_changes: Vec<PendingChange>,
    cursors: HashMap<u32, UserCursor>,
    connected: bool,
}

#[wasm_bindgen]
impl SyncEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            client_id: None,
            sequence: 0,
            pending_changes: Vec::new(),
            cursors: HashMap::new(),
            connected: false,
        }
    }

    pub fn client_id(&self) -> Option<u32> {
        self.client_id.map(|c| c.value())
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn set_connected(&mut self, connected: bool) {
        self.connected = connected;
        if !connected {
            self.client_id = None;
        }
    }

    /// Create a join message
    pub fn create_join_message(&self, document_id: &str, client_name: &str) -> String {
        Message::Join {
            document_id: document_id.to_string(),
            client_name: client_name.to_string(),
        }.to_json()
    }

    /// Create a cursor move message
    pub fn create_cursor_message(&self, x: f32, y: f32) -> Option<String> {
        self.client_id.map(|id| {
            Message::CursorMove {
                client_id: id.value(),
                x,
                y,
            }.to_json()
        })
    }

    /// Create a property change message
    pub fn create_property_change_message(&mut self, object_id: ObjectId, property: Property, value: &str) -> Option<String> {
        self.client_id.map(|id| {
            self.sequence += 1;
            Message::PropertyChange {
                client_id: id.value(),
                object_id,
                property,
                value: value.to_string(),
                sequence: self.sequence,
            }.to_json()
        })
    }

    /// Process an incoming message
    pub fn process_message(&mut self, json: &str, document: &mut Document) -> Option<String> {
        let message = Message::from_json(json)?;
        
        match message {
            Message::JoinAck { client_id, document_state: _ } => {
                self.client_id = Some(ClientId::new(client_id));
                self.connected = true;
                None
            }
            Message::CursorMove { client_id, x, y } => {
                if let Some(cursor) = self.cursors.get_mut(&client_id) {
                    cursor.set_position(x, y);
                } else {
                    let mut cursor = UserCursor::new(
                        ClientId::new(client_id),
                        &format!("User {}", client_id),
                        &crate::multiplayer::get_user_color(ClientId::new(client_id)),
                    );
                    cursor.set_position(x, y);
                    self.cursors.insert(client_id, cursor);
                }
                None
            }
            Message::PropertyChange { client_id: _, object_id, property, value, sequence: _ } => {
                // Apply the change if it doesn't conflict with pending changes
                if !self.has_pending_change(object_id, property) {
                    if let Ok(prop_value) = serde_json::from_str::<PropertyValue>(&value) {
                        document.set_node_property(object_id, property, prop_value);
                    }
                }
                None
            }
            Message::Ack { sequence } => {
                // Remove acknowledged changes
                self.pending_changes.retain(|c| c.sequence != sequence);
                None
            }
            Message::Leave { client_id } => {
                self.cursors.remove(&client_id);
                None
            }
            Message::Ping => {
                Some(Message::Pong.to_json())
            }
            _ => None,
        }
    }

    /// Check if we have a pending change for this property
    fn has_pending_change(&self, object_id: ObjectId, property: Property) -> bool {
        self.pending_changes.iter().any(|c| c.object_id == object_id && c.property == property)
    }

    /// Get cursor count
    pub fn cursor_count(&self) -> usize {
        self.cursors.len()
    }

    /// Get pending change count
    pub fn pending_count(&self) -> usize {
        self.pending_changes.len()
    }
}

impl Default for SyncEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncEngine {
    /// Get all cursors (for rendering)
    pub fn cursors(&self) -> impl Iterator<Item = &UserCursor> {
        self.cursors.values()
    }

    /// Add a pending change
    pub fn add_pending_change(&mut self, object_id: ObjectId, property: Property, value: PropertyValue) {
        self.sequence += 1;
        self.pending_changes.push(PendingChange {
            object_id,
            property,
            value,
            sequence: self.sequence,
        });
    }
}
