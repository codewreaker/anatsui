//! Document tree structure
//!
//! Implements the tree using parent pointers on children (like Figma)
//! with fractional indexing for ordering.

use super::{Node, ObjectId, Property, PropertyValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The document tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTree {
    nodes: HashMap<ObjectId, Node>,
    root_id: ObjectId,
    /// Maps parent ID to ordered list of children IDs
    children_map: HashMap<ObjectId, Vec<ObjectId>>,
    /// Maps child ID to parent ID
    parent_map: HashMap<ObjectId, ObjectId>,
}

impl DocumentTree {
    pub fn new() -> Self {
        let root_id = ObjectId::random();
        Self {
            nodes: HashMap::new(),
            root_id,
            children_map: HashMap::new(),
            parent_map: HashMap::new(),
        }
    }

    pub fn root_id(&self) -> ObjectId {
        self.root_id
    }

    pub fn get(&self, id: ObjectId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    pub fn insert(&mut self, node: Node) {
        let id = node.id();
        if self.nodes.is_empty() {
            self.root_id = id;
        }
        self.nodes.insert(id, node);
    }

    pub fn remove(&mut self, id: ObjectId) {
        // Remove from parent's children
        if let Some(parent_id) = self.parent_map.remove(&id) {
            if let Some(children) = self.children_map.get_mut(&parent_id) {
                children.retain(|&child_id| child_id != id);
            }
        }
        
        // Remove children recursively
        if let Some(children) = self.children_map.remove(&id) {
            for child_id in children {
                self.remove(child_id);
            }
        }
        
        self.nodes.remove(&id);
    }

    /// Set the parent of a node
    pub fn set_parent(&mut self, child_id: ObjectId, parent_id: ObjectId) {
        // Remove from old parent
        if let Some(old_parent_id) = self.parent_map.get(&child_id).cloned() {
            if let Some(children) = self.children_map.get_mut(&old_parent_id) {
                children.retain(|&id| id != child_id);
            }
        }
        
        // Add to new parent
        self.parent_map.insert(child_id, parent_id);
        self.children_map
            .entry(parent_id)
            .or_insert_with(Vec::new)
            .push(child_id);
        
        // Sort children by fractional index
        if let Some(children) = self.children_map.get_mut(&parent_id) {
            children.sort_by(|a, b| {
                let a_index = self.nodes.get(a).map(|n| n.order_index()).unwrap_or("0.5");
                let b_index = self.nodes.get(b).map(|n| n.order_index()).unwrap_or("0.5");
                a_index.cmp(b_index)
            });
        }
    }

    /// Get the parent of a node
    pub fn parent(&self, child_id: ObjectId) -> Option<ObjectId> {
        self.parent_map.get(&child_id).cloned()
    }

    /// Get children of a node
    pub fn children(&self, parent_id: ObjectId) -> Vec<ObjectId> {
        self.children_map
            .get(&parent_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Get the first page in the document
    pub fn first_page(&self) -> Option<ObjectId> {
        self.children(self.root_id).first().cloned()
    }

    /// Move a node before another sibling
    pub fn move_before(&mut self, node_id: ObjectId, before_id: ObjectId) {
        if let Some(parent_id) = self.parent_map.get(&before_id).cloned() {
            self.set_parent(node_id, parent_id);
            
            // Calculate new fractional index
            if let Some(children) = self.children_map.get(&parent_id) {
                if let Some(before_idx) = children.iter().position(|&id| id == before_id) {
                    let before_index = self.nodes.get(&before_id)
                        .map(|n| n.order_index().to_string())
                        .unwrap_or_else(|| "0.5".to_string());
                    
                    let prev_index = if before_idx > 0 {
                        children.get(before_idx - 1)
                            .and_then(|&id| self.nodes.get(&id))
                            .map(|n| n.order_index().to_string())
                            .unwrap_or_else(|| "0".to_string())
                    } else {
                        "0".to_string()
                    };
                    
                    let new_index = fractional_midpoint(&prev_index, &before_index);
                    
                    if let Some(node) = self.nodes.get_mut(&node_id) {
                        node.set_order_index(new_index);
                    }
                }
            }
        }
    }

    /// Move a node after another sibling
    pub fn move_after(&mut self, node_id: ObjectId, after_id: ObjectId) {
        if let Some(parent_id) = self.parent_map.get(&after_id).cloned() {
            self.set_parent(node_id, parent_id);
            
            if let Some(children) = self.children_map.get(&parent_id) {
                if let Some(after_idx) = children.iter().position(|&id| id == after_id) {
                    let after_index = self.nodes.get(&after_id)
                        .map(|n| n.order_index().to_string())
                        .unwrap_or_else(|| "0.5".to_string());
                    
                    let next_index = children.get(after_idx + 1)
                        .and_then(|&id| self.nodes.get(&id))
                        .map(|n| n.order_index().to_string())
                        .unwrap_or_else(|| "1".to_string());
                    
                    let new_index = fractional_midpoint(&after_index, &next_index);
                    
                    if let Some(node) = self.nodes.get_mut(&node_id) {
                        node.set_order_index(new_index);
                    }
                }
            }
        }
    }

    /// Iterate over all nodes
    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    /// Get count of nodes
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl Default for DocumentTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate the midpoint between two fractional indices
/// Uses string-based arithmetic to maintain precision
fn fractional_midpoint(a: &str, b: &str) -> String {
    // Simple implementation: parse as f64, average, format
    // In production, use arbitrary-precision string arithmetic
    let a_val: f64 = a.parse().unwrap_or(0.0);
    let b_val: f64 = b.parse().unwrap_or(1.0);
    let mid = (a_val + b_val) / 2.0;
    format!("{:.15}", mid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::NodeType;

    #[test]
    fn test_tree_operations() {
        let mut tree = DocumentTree::new();
        
        let parent_id = ObjectId::random();
        let parent = Node::new(parent_id, NodeType::Frame);
        tree.insert(parent);
        
        let child_id = ObjectId::random();
        let child = Node::new(child_id, NodeType::Rectangle);
        tree.insert(child);
        tree.set_parent(child_id, parent_id);
        
        assert_eq!(tree.children(parent_id), vec![child_id]);
        assert_eq!(tree.parent(child_id), Some(parent_id));
    }
}
