//! Vector network point (vertex)

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// A point in a vector network
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VectorPoint {
    pub x: f32,
    pub y: f32,
    /// Handle for incoming curve (relative to point)
    pub handle_in_x: f32,
    pub handle_in_y: f32,
    /// Handle for outgoing curve (relative to point)
    pub handle_out_x: f32,
    pub handle_out_y: f32,
    /// Point type affects how handles behave
    point_type: PointType,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PointType {
    /// Handles are independent
    Corner,
    /// Handles are aligned (smooth curve)
    Smooth,
    /// Handles are aligned and equal length
    Symmetric,
}

#[wasm_bindgen]
impl VectorPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            handle_in_x: 0.0,
            handle_in_y: 0.0,
            handle_out_x: 0.0,
            handle_out_y: 0.0,
            point_type: PointType::Corner,
        }
    }

    pub fn with_handles(x: f32, y: f32, in_x: f32, in_y: f32, out_x: f32, out_y: f32) -> Self {
        Self {
            x,
            y,
            handle_in_x: in_x,
            handle_in_y: in_y,
            handle_out_x: out_x,
            handle_out_y: out_y,
            point_type: PointType::Corner,
        }
    }

    pub fn point_type(&self) -> PointType {
        self.point_type
    }

    pub fn set_point_type(&mut self, point_type: PointType) {
        self.point_type = point_type;
        self.adjust_handles_for_type();
    }

    /// Move the point
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    /// Set the incoming handle
    pub fn set_handle_in(&mut self, dx: f32, dy: f32) {
        self.handle_in_x = dx;
        self.handle_in_y = dy;
        self.adjust_handles_for_type();
    }

    /// Set the outgoing handle
    pub fn set_handle_out(&mut self, dx: f32, dy: f32) {
        self.handle_out_x = dx;
        self.handle_out_y = dy;
        self.adjust_handles_for_type();
    }

    /// Get absolute position of incoming handle
    pub fn handle_in_absolute(&self) -> (f32, f32) {
        (self.x + self.handle_in_x, self.y + self.handle_in_y)
    }

    /// Get absolute position of outgoing handle
    pub fn handle_out_absolute(&self) -> (f32, f32) {
        (self.x + self.handle_out_x, self.y + self.handle_out_y)
    }

    /// Check if this point has handles (is curved)
    pub fn has_handles(&self) -> bool {
        self.handle_in_x != 0.0 || self.handle_in_y != 0.0 ||
        self.handle_out_x != 0.0 || self.handle_out_y != 0.0
    }

    /// Distance to another point
    pub fn distance_to(&self, other: &VectorPoint) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Distance to a coordinate
    pub fn distance_to_coord(&self, px: f32, py: f32) -> f32 {
        let dx = px - self.x;
        let dy = py - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn adjust_handles_for_type(&mut self) {
        match self.point_type {
            PointType::Corner => {
                // Handles are independent, nothing to do
            }
            PointType::Smooth => {
                // Align handles (opposite directions)
                let len_in = (self.handle_in_x * self.handle_in_x + self.handle_in_y * self.handle_in_y).sqrt();
                let len_out = (self.handle_out_x * self.handle_out_x + self.handle_out_y * self.handle_out_y).sqrt();
                
                if len_out > 0.0 {
                    let dx = -self.handle_out_x / len_out;
                    let dy = -self.handle_out_y / len_out;
                    self.handle_in_x = dx * len_in;
                    self.handle_in_y = dy * len_in;
                }
            }
            PointType::Symmetric => {
                // Handles are opposite with same length
                self.handle_in_x = -self.handle_out_x;
                self.handle_in_y = -self.handle_out_y;
            }
        }
    }
}

impl Default for VectorPoint {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}
