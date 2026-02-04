//! Document model for Anatsui
//!
//! Represents the document as a tree of nodes with properties.
//! Inspired by Figma's approach: Map<ObjectID, Map<Property, Value>>

mod node;
mod properties;
mod tree;

pub use node::*;
pub use properties::*;
pub use tree::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

/// Unique identifier for objects in the document
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectId {
    client_id: u32,
    sequence: u32,
}

#[wasm_bindgen]
impl ObjectId {
    /// Create a new ObjectId
    pub fn new(client_id: u32, sequence: u32) -> Self {
        Self { client_id, sequence }
    }

    /// Generate a random ObjectId (for standalone use)
    pub fn random() -> Self {
        let uuid = Uuid::new_v4();
        let bytes = uuid.as_bytes();
        Self {
            client_id: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            sequence: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        }
    }

    pub fn client_id(&self) -> u32 {
        self.client_id
    }

    pub fn sequence(&self) -> u32 {
        self.sequence
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.client_id, self.sequence)
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.client_id, self.sequence)
    }
}

/// The main document structure
#[wasm_bindgen]
pub struct Document {
    tree: DocumentTree,
    name: String,
    version: u32,
}

#[wasm_bindgen]
impl Document {
    /// Create a new empty document
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Self {
        let mut tree = DocumentTree::new();
        
        // Create root node
        let root_id = ObjectId::random();
        let root = Node::new(root_id, NodeType::Document);
        tree.insert(root);
        
        // Create first page
        let page_id = ObjectId::random();
        let mut page = Node::new(page_id, NodeType::Page);
        page.set_property(Property::Name, PropertyValue::String("Page 1".into()));
        tree.insert(page);
        tree.set_parent(page_id, root_id);
        
        Self {
            tree,
            name: name.to_string(),
            version: 1,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    /// Get the root node ID
    pub fn root_id(&self) -> ObjectId {
        self.tree.root_id()
    }

    /// Create a new frame on the first page
    pub fn create_frame(&mut self, x: f32, y: f32, width: f32, height: f32) -> ObjectId {
        let frame_id = ObjectId::random();
        let mut frame = Node::new(frame_id, NodeType::Frame);
        
        frame.set_property(Property::X, PropertyValue::Float(x));
        frame.set_property(Property::Y, PropertyValue::Float(y));
        frame.set_property(Property::Width, PropertyValue::Float(width));
        frame.set_property(Property::Height, PropertyValue::Float(height));
        frame.set_property(Property::Name, PropertyValue::String("Frame".into()));
        
        self.tree.insert(frame);
        
        // Get first page and parent to it
        if let Some(first_page) = self.tree.first_page() {
            self.tree.set_parent(frame_id, first_page);
        }
        
        self.version += 1;
        frame_id
    }

    /// Create a rectangle
    pub fn create_rectangle(&mut self, parent_id: ObjectId, x: f32, y: f32, width: f32, height: f32) -> ObjectId {
        let rect_id = ObjectId::random();
        let mut rect = Node::new(rect_id, NodeType::Rectangle);
        
        rect.set_property(Property::X, PropertyValue::Float(x));
        rect.set_property(Property::Y, PropertyValue::Float(y));
        rect.set_property(Property::Width, PropertyValue::Float(width));
        rect.set_property(Property::Height, PropertyValue::Float(height));
        rect.set_property(Property::FillColor, PropertyValue::Color(Color::new(0.8, 0.8, 0.8, 1.0)));
        
        self.tree.insert(rect);
        self.tree.set_parent(rect_id, parent_id);
        
        self.version += 1;
        rect_id
    }

    /// Create an ellipse
    pub fn create_ellipse(&mut self, parent_id: ObjectId, x: f32, y: f32, width: f32, height: f32) -> ObjectId {
        let ellipse_id = ObjectId::random();
        let mut ellipse = Node::new(ellipse_id, NodeType::Ellipse);
        
        ellipse.set_property(Property::X, PropertyValue::Float(x));
        ellipse.set_property(Property::Y, PropertyValue::Float(y));
        ellipse.set_property(Property::Width, PropertyValue::Float(width));
        ellipse.set_property(Property::Height, PropertyValue::Float(height));
        ellipse.set_property(Property::FillColor, PropertyValue::Color(Color::new(0.6, 0.6, 0.9, 1.0)));
        
        self.tree.insert(ellipse);
        self.tree.set_parent(ellipse_id, parent_id);
        
        self.version += 1;
        ellipse_id
    }

    /// Create a text node
    pub fn create_text(&mut self, parent_id: ObjectId, x: f32, y: f32, content: &str) -> ObjectId {
        let text_id = ObjectId::random();
        let mut text = Node::new(text_id, NodeType::Text);
        
        text.set_property(Property::X, PropertyValue::Float(x));
        text.set_property(Property::Y, PropertyValue::Float(y));
        text.set_property(Property::Text, PropertyValue::String(content.into()));
        text.set_property(Property::FontSize, PropertyValue::Float(16.0));
        text.set_property(Property::FillColor, PropertyValue::Color(Color::new(0.0, 0.0, 0.0, 1.0)));
        
        self.tree.insert(text);
        self.tree.set_parent(text_id, parent_id);
        
        self.version += 1;
        text_id
    }

    /// Check if a node exists
    pub fn has_node(&self, id: ObjectId) -> bool {
        self.tree.get(id).is_some()
    }

    /// Get a node by ID (internal use - not exposed to WASM)
    pub(crate) fn get_node(&self, id: ObjectId) -> Option<Node> {
        self.tree.get(id).cloned()
    }

    /// Get node X position
    pub fn get_node_x(&self, id: ObjectId) -> f32 {
        self.tree.get(id).map(|n| n.x()).unwrap_or(0.0)
    }

    /// Get node Y position
    pub fn get_node_y(&self, id: ObjectId) -> f32 {
        self.tree.get(id).map(|n| n.y()).unwrap_or(0.0)
    }

    /// Get node width
    pub fn get_node_width(&self, id: ObjectId) -> f32 {
        self.tree.get(id).map(|n| n.width()).unwrap_or(0.0)
    }

    /// Get node height
    pub fn get_node_height(&self, id: ObjectId) -> f32 {
        self.tree.get(id).map(|n| n.height()).unwrap_or(0.0)
    }

    /// Set node X position
    pub fn set_node_x(&mut self, id: ObjectId, x: f32) {
        if let Some(node) = self.tree.get_mut(id) {
            node.set_property(Property::X, PropertyValue::Float(x));
            self.version += 1;
        }
    }

    /// Set node Y position
    pub fn set_node_y(&mut self, id: ObjectId, y: f32) {
        if let Some(node) = self.tree.get_mut(id) {
            node.set_property(Property::Y, PropertyValue::Float(y));
            self.version += 1;
        }
    }

    /// Set node width
    pub fn set_node_width(&mut self, id: ObjectId, width: f32) {
        if let Some(node) = self.tree.get_mut(id) {
            node.set_property(Property::Width, PropertyValue::Float(width));
            self.version += 1;
        }
    }

    /// Set node height
    pub fn set_node_height(&mut self, id: ObjectId, height: f32) {
        if let Some(node) = self.tree.get_mut(id) {
            node.set_property(Property::Height, PropertyValue::Float(height));
            self.version += 1;
        }
    }

    /// Update a node property (internal use)
    pub(crate) fn set_node_property(&mut self, id: ObjectId, property: Property, value: PropertyValue) {
        if let Some(node) = self.tree.get_mut(id) {
            node.set_property(property, value);
            self.version += 1;
        }
    }

    /// Delete a node
    pub fn delete_node(&mut self, id: ObjectId) {
        self.tree.remove(id);
        self.version += 1;
    }

    /// Get children of a node
    pub fn get_children(&self, parent_id: ObjectId) -> Vec<ObjectId> {
        self.tree.children(parent_id)
    }

    /// Serialize document to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.tree).unwrap_or_default()
    }
}
