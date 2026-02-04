//! Vector network segment (edge between two points)

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// A segment connecting two points in a vector network
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VectorSegment {
    /// Index of start point
    pub start: u32,
    /// Index of end point
    pub end: u32,
    /// Whether this segment is selected
    selected: bool,
}

#[wasm_bindgen]
impl VectorSegment {
    #[wasm_bindgen(constructor)]
    pub fn new(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
            selected: false,
        }
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    /// Check if this segment connects to a point
    pub fn connects_to(&self, point_index: u32) -> bool {
        self.start == point_index || self.end == point_index
    }

    /// Get the other point of this segment
    pub fn other_point(&self, point_index: u32) -> Option<u32> {
        if self.start == point_index {
            Some(self.end)
        } else if self.end == point_index {
            Some(self.start)
        } else {
            None
        }
    }
}

impl PartialEq for VectorSegment {
    fn eq(&self, other: &Self) -> bool {
        // Segments are equal regardless of direction
        (self.start == other.start && self.end == other.end) ||
        (self.start == other.end && self.end == other.start)
    }
}

impl Eq for VectorSegment {}

impl std::hash::Hash for VectorSegment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash in a way that's independent of direction
        let (a, b) = if self.start <= self.end {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        };
        a.hash(state);
        b.hash(state);
    }
}
