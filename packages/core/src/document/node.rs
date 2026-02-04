//! Node types and node structure

use super::{Color, Property, PropertyValue};
use crate::document::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// Types of nodes in the document
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Document,
    Page,
    Frame,
    Group,
    Rectangle,
    Ellipse,
    Line,
    Vector,
    Text,
    Image,
    Component,
    Instance,
}

/// A node in the document tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    id: ObjectId,
    node_type: NodeType,
    properties: HashMap<Property, PropertyValue>,
    /// Fractional index for ordering among siblings
    order_index: String,
}

impl Node {
    /// Create a new node
    pub fn new(id: ObjectId, node_type: NodeType) -> Self {
        Self {
            id,
            node_type,
            properties: HashMap::new(),
            order_index: "0.5".to_string(),
        }
    }

    pub fn id(&self) -> ObjectId {
        self.id
    }

    pub fn node_type(&self) -> NodeType {
        self.node_type
    }

    pub fn order_index(&self) -> &str {
        &self.order_index
    }

    pub fn set_order_index(&mut self, index: String) {
        self.order_index = index;
    }

    /// Get a property value
    pub fn get_property(&self, property: Property) -> Option<&PropertyValue> {
        self.properties.get(&property)
    }

    /// Set a property value
    pub fn set_property(&mut self, property: Property, value: PropertyValue) {
        self.properties.insert(property, value);
    }

    /// Remove a property
    pub fn remove_property(&mut self, property: Property) {
        self.properties.remove(&property);
    }

    /// Get all properties
    pub fn properties(&self) -> &HashMap<Property, PropertyValue> {
        &self.properties
    }

    // Convenience getters for common properties
    
    pub fn x(&self) -> f32 {
        match self.get_property(Property::X) {
            Some(PropertyValue::Float(v)) => *v,
            _ => 0.0,
        }
    }

    pub fn y(&self) -> f32 {
        match self.get_property(Property::Y) {
            Some(PropertyValue::Float(v)) => *v,
            _ => 0.0,
        }
    }

    pub fn width(&self) -> f32 {
        match self.get_property(Property::Width) {
            Some(PropertyValue::Float(v)) => *v,
            _ => 100.0,
        }
    }

    pub fn height(&self) -> f32 {
        match self.get_property(Property::Height) {
            Some(PropertyValue::Float(v)) => *v,
            _ => 100.0,
        }
    }

    pub fn rotation(&self) -> f32 {
        match self.get_property(Property::Rotation) {
            Some(PropertyValue::Float(v)) => *v,
            _ => 0.0,
        }
    }

    pub fn opacity(&self) -> f32 {
        match self.get_property(Property::Opacity) {
            Some(PropertyValue::Float(v)) => *v,
            _ => 1.0,
        }
    }

    pub fn fill_color(&self) -> Color {
        match self.get_property(Property::FillColor) {
            Some(PropertyValue::Color(c)) => *c,
            _ => Color::default(),
        }
    }

    pub fn stroke_color(&self) -> Color {
        match self.get_property(Property::StrokeColor) {
            Some(PropertyValue::Color(c)) => *c,
            _ => Color::transparent(),
        }
    }

    pub fn stroke_width(&self) -> f32 {
        match self.get_property(Property::StrokeWidth) {
            Some(PropertyValue::Float(v)) => *v,
            _ => 0.0,
        }
    }

    pub fn corner_radius(&self) -> f32 {
        match self.get_property(Property::CornerRadius) {
            Some(PropertyValue::Float(v)) => *v,
            _ => 0.0,
        }
    }

    pub fn name(&self) -> String {
        match self.get_property(Property::Name) {
            Some(PropertyValue::String(s)) => s.clone(),
            _ => format!("{:?}", self.node_type),
        }
    }

    pub fn visible(&self) -> bool {
        match self.get_property(Property::Visible) {
            Some(PropertyValue::Bool(v)) => *v,
            _ => true,
        }
    }

    pub fn locked(&self) -> bool {
        match self.get_property(Property::Locked) {
            Some(PropertyValue::Bool(v)) => *v,
            _ => false,
        }
    }
}
