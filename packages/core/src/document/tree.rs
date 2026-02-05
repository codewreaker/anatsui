//! # Document Tree Structure
//!
//! This module implements the scene graph for Anatsui's design canvas.
//! It's heavily inspired by Figma's architecture and uses similar concepts.
//!
//! ## Key Concepts
//!
//! ### Tree Structure
//! - Documents contain Pages
//! - Pages contain Frames and Shapes
//! - Frames can contain nested Frames and Shapes
//! - Shapes are leaf nodes (rectangles, ellipses, vectors, text)
//!
//! ```text
//! Document (root)
//!   └─ Page 1
//!       ├─ Frame "Designs"
//!       │   ├─ Frame "Card"
//!       │   │   ├─ Rectangle (background)
//!       │   │   ├─ Text (title)
//!       │   │   └─ Vector (icon)
//!       │   └─ Frame "Button"
//!       │       ├─ Rectangle
//!       │       └─ Text
//!       └─ Ellipse (standalone)
//! ```
//!
//! ### Parent-Child Relationships
//! - Uses **parent pointers** (child stores reference to parent)
//! - Also maintains **children maps** (parent -> [children]) for fast lookup
//! - Bidirectional for efficient traversal in both directions
//!
//! ### Fractional Indexing (Z-Order)
//! - Nodes have a fractional "order_index" (e.g., "0.5", "0.75", "0.625")
//! - This allows inserting nodes *between* existing nodes without reordering everything
//! - Example: To insert between "0.5" and "1.0", use "0.75"
//! - To insert between "0.75" and "1.0", use "0.875"
//!
//! ## Why This Design?
//!
//! ### Why not just Vec<Node>?
//! - A flat vector would require O(n) reparenting operations
//! - HashMap gives O(1) lookup by ID
//! - Parent/children maps give O(1) relationship queries
//!
//! ### Why fractional indexing instead of array indices?
//! - Moving a node doesn't require updating all siblings
//! - Works better with CRDT (conflict-free replicated data) for multiplayer
//! - Can insert between any two nodes without gaps
//!
//! ### Why HashMap instead of a tree of Rc<RefCell<Node>>?
//! - Simpler mental model and ownership rules
//! - No circular references or memory leaks
//! - Better for serialization (can send over network)
//! - Cache-friendly: nodes stored contiguously in memory

use super::{Node, ObjectId, Property, PropertyValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The document tree holding all nodes in the canvas.
///
/// This is the main data structure representing your entire design file.
/// It's essentially a fancy HashMap with parent-child relationship tracking.
///
/// ## Internal Structure
///
/// ```text
/// ┌───────────────────────────────────────┐
/// │         DocumentTree                  │
/// │                                       │
/// │  nodes: HashMap<ID, Node>             │ ← All nodes by ID
/// │    - "abc123" → Rectangle Node        │
/// │    - "def456" → Frame Node            │
/// │    - "ghi789" → Text Node             │
/// │                                       │
/// │  children_map: HashMap<ID, Vec<ID>>   │ ← Parent → Children
/// │    - "frame-1" → ["rect-1", "text-1"] │
/// │                                       │
/// │  parent_map: HashMap<ID, ID>          │ ← Child → Parent
/// │    - "rect-1" → "frame-1"             │
/// │    - "text-1" → "frame-1"             │
/// │                                       │
/// │  root_id: ObjectId                    │ ← Document root
/// └───────────────────────────────────────┘
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTree {
    /// All nodes indexed by their unique ID.
    /// This is the single source of truth for node data.
    nodes: HashMap<ObjectId, Node>,
    
    /// The root node of the document tree.
    /// This is typically a special "Document" node that contains all pages.
    root_id: ObjectId,
    
    /// Maps parent ID to ordered list of children IDs.
    /// Children are sorted by their order_index for consistent rendering order.
    /// Example: {"frame-1" => ["shape-a", "shape-b", "shape-c"]}
    children_map: HashMap<ObjectId, Vec<ObjectId>>,
    
    /// Maps child ID to parent ID for fast upward traversal.
    /// Example: {"shape-a" => "frame-1"}
    parent_map: HashMap<ObjectId, ObjectId>,
}

impl DocumentTree {
    /// Create a new empty document tree with a random root ID.
    ///
    /// The root node is not automatically created - you need to insert it.
    pub fn new() -> Self {
        let root_id = ObjectId::random();
        Self {
            nodes: HashMap::new(),
            root_id,
            children_map: HashMap::new(),
            parent_map: HashMap::new(),
        }
    }

    /// Get the ID of the root node.
    ///
    /// The root is typically a "Document" node that contains all pages.
    pub fn root_id(&self) -> ObjectId {
        self.root_id
    }

    /// Get an immutable reference to a node by ID.
    ///
    /// Returns `None` if the node doesn't exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// if let Some(node) = tree.get(node_id) {
    ///     println!("Node name: {}", node.name());
    /// }
    /// ```
    pub fn get(&self, id: ObjectId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    /// Get a mutable reference to a node by ID.
    ///
    /// This allows you to modify the node's properties.
    ///
    /// # Example
    ///
    /// ```rust
    /// if let Some(node) = tree.get_mut(node_id) {
    ///     node.set_name("New Name".to_string());
    /// }
    /// ```
    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    /// Insert a node into the tree.
    ///
    /// If this is the first node, it becomes the root.
    /// Otherwise, you need to call `set_parent()` to establish relationships.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut node = Node::new_rectangle();
    /// tree.insert(node.clone());
    /// tree.set_parent(node.id(), parent_id);
    /// ```
    pub fn insert(&mut self, node: Node) {
        let id = node.id();
        
        // If tree is empty, this becomes the root
        if self.nodes.is_empty() {
            self.root_id = id;
        }
        
        self.nodes.insert(id, node);
    }

    /// Remove a node and all its descendants from the tree.
    ///
    /// This is a recursive operation that:
    /// 1. Removes the node from its parent's children list
    /// 2. Recursively removes all children
    /// 3. Cleans up parent/children maps
    ///
    /// # Example
    ///
    /// ```rust
    /// // Delete a frame and everything inside it
    /// tree.remove(frame_id);
    /// ```
    pub fn remove(&mut self, id: ObjectId) {
        // Step 1: Remove from parent's children list
        if let Some(parent_id) = self.parent_map.remove(&id) {
            if let Some(children) = self.children_map.get_mut(&parent_id) {
                children.retain(|&child_id| child_id != id);
            }
        }
        
        // Step 2: Recursively remove all descendants
        if let Some(children) = self.children_map.remove(&id) {
            for child_id in children {
                self.remove(child_id); // Recursive call
            }
        }
        
        // Step 3: Remove the node itself
        self.nodes.remove(&id);
    }

    /// Set the parent of a node, moving it to a new location in the tree.
    ///
    /// This handles:
    /// 1. Removing from old parent (if any)
    /// 2. Adding to new parent
    /// 3. Sorting children by order_index (z-order)
    ///
    /// # Example
    ///
    /// ```rust
    /// // Move a shape into a frame
    /// tree.set_parent(shape_id, frame_id);
    /// ```
    pub fn set_parent(&mut self, child_id: ObjectId, parent_id: ObjectId) {
        // Remove from old parent's children list
        if let Some(old_parent_id) = self.parent_map.get(&child_id).cloned() {
            if let Some(children) = self.children_map.get_mut(&old_parent_id) {
                children.retain(|&id| id != child_id);
            }
        }
        
        // Update parent map
        self.parent_map.insert(child_id, parent_id);
        
        // Add to new parent's children list
        self.children_map
            .entry(parent_id)
            .or_insert_with(Vec::new)
            .push(child_id);
        
        // Sort children by fractional index (determines draw order)
        if let Some(children) = self.children_map.get_mut(&parent_id) {
            children.sort_by(|a, b| {
                let a_index = self.nodes.get(a).map(|n| n.order_index()).unwrap_or("0.5");
                let b_index = self.nodes.get(b).map(|n| n.order_index()).unwrap_or("0.5");
                a_index.cmp(b_index)
            });
        }
    }

    /// Get the parent ID of a node.
    ///
    /// Returns `None` if the node is the root or doesn't exist.
    pub fn parent(&self, child_id: ObjectId) -> Option<ObjectId> {
        self.parent_map.get(&child_id).cloned()
    }

    /// Get all children IDs of a node, in order.
    ///
    /// The order is determined by each child's `order_index`.
    /// Returns an empty vector if the node has no children.
    pub fn children(&self, parent_id: ObjectId) -> Vec<ObjectId> {
        self.children_map
            .get(&parent_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Get the first page in the document.
    ///
    /// This is a convenience method that returns the first child of the root.
    /// In most documents, this is "Page 1".
    pub fn first_page(&self) -> Option<ObjectId> {
        self.children(self.root_id).first().cloned()
    }

    /// Move a node before another sibling using fractional indexing.
    ///
    /// This calculates a new order_index that places `node_id` directly before `before_id`.
    /// Both nodes must share the same parent.
    ///
    /// # Example
    ///
    /// ```text
    /// Before: [A(0.3), B(0.5), C(0.7)]
    /// tree.move_before(C, B)
    /// After:  [A(0.3), C(0.4), B(0.5)]
    /// ```
    pub fn move_before(&mut self, node_id: ObjectId, before_id: ObjectId) {
        if let Some(parent_id) = self.parent_map.get(&before_id).cloned() {
            self.set_parent(node_id, parent_id);
            
            // Calculate new fractional index between previous sibling and before_id
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
                    
                    // Midpoint between prev and before
                    let new_index = fractional_midpoint(&prev_index, &before_index);
                    
                    if let Some(node) = self.nodes.get_mut(&node_id) {
                        node.set_order_index(new_index);
                    }
                }
            }
        }
    }

    /// Move a node after another sibling using fractional indexing.
    ///
    /// This calculates a new order_index that places `node_id` directly after `after_id`.
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
                    
                    // Midpoint between after and next
                    let new_index = fractional_midpoint(&after_index, &next_index);
                    
                    if let Some(node) = self.nodes.get_mut(&node_id) {
                        node.set_order_index(new_index);
                    }
                }
            }
        }
    }

    /// Iterate over all nodes in the tree (unordered).
    ///
    /// If you need hierarchical traversal, use `children()` recursively.
    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    /// Get the total count of nodes in the tree.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Check if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl Default for DocumentTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate the midpoint between two fractional indices for z-ordering.
///
/// ## How Fractional Indexing Works
///
/// Instead of using array indices (0, 1, 2, ...), we use fractional values:
/// - Node A: "0.5"
/// - Node B: "0.75"
/// - Node C: "0.875"
///
/// To insert between A and B: `(0.5 + 0.75) / 2 = 0.625`
///
/// ## Why String-Based?
///
/// Using strings instead of f64 gives us:
/// - Arbitrary precision (no floating-point rounding errors)
/// - Consistent behavior across platforms
/// - Better for CRDTs (conflict-free replicated data types)
///
/// ## Production Note
///
/// This is a simplified implementation. In production, you'd want:
/// - Arbitrary-precision string arithmetic
/// - Automatic rebalancing when indices get too long
/// - Fractional-indexing library (e.g., `fractional-index` crate)
///
/// # Arguments
///
/// * `a` - The lower bound fractional index (e.g., "0.5")
/// * `b` - The upper bound fractional index (e.g., "0.75")
///
/// # Returns
///
/// A string representing the midpoint (e.g., "0.625")
fn fractional_midpoint(a: &str, b: &str) -> String {
    // Parse as floats (in production, use arbitrary-precision strings)
    let a_val: f64 = a.parse().unwrap_or(0.0);
    let b_val: f64 = b.parse().unwrap_or(1.0);
    
    // Calculate midpoint
    let mid = (a_val + b_val) / 2.0;
    
    // Format with enough precision to avoid collisions
    // 15 decimal places is usually sufficient
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
