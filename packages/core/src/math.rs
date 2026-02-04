//! Math utilities for Anatsui
//!
//! Re-exports glam types and provides additional math utilities.

pub use glam::{Mat3, Mat4, Vec2, Vec3, Vec4};

use wasm_bindgen::prelude::*;

/// A 2D transformation matrix
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Transform2D {
    matrix: Mat3,
}

#[wasm_bindgen]
impl Transform2D {
    /// Create an identity transform
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            matrix: Mat3::IDENTITY,
        }
    }

    /// Create a translation transform
    pub fn translate(x: f32, y: f32) -> Self {
        Self {
            matrix: Mat3::from_translation(Vec2::new(x, y)),
        }
    }

    /// Create a rotation transform (angle in radians)
    pub fn rotate(angle: f32) -> Self {
        Self {
            matrix: Mat3::from_angle(angle),
        }
    }

    /// Create a scale transform
    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            matrix: Mat3::from_scale(Vec2::new(sx, sy)),
        }
    }

    /// Multiply this transform by another
    pub fn multiply(&self, other: &Transform2D) -> Self {
        Self {
            matrix: self.matrix * other.matrix,
        }
    }

    /// Get the inverse of this transform
    pub fn inverse(&self) -> Self {
        Self {
            matrix: self.matrix.inverse(),
        }
    }

    /// Transform a point
    pub fn transform_point(&self, x: f32, y: f32) -> Vec<f32> {
        let p = self.matrix.transform_point2(Vec2::new(x, y));
        vec![p.x, p.y]
    }

    /// Get the matrix as a flat array for WebGL
    pub fn to_array(&self) -> Vec<f32> {
        self.matrix.to_cols_array().to_vec()
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::new()
    }
}

/// A rectangle with position and size
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[wasm_bindgen]
impl Rect {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    pub fn center_x(&self) -> f32 {
        self.x + self.width / 2.0
    }

    pub fn center_y(&self) -> f32 {
        self.y + self.height / 2.0
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }
}

/// Linear interpolation
#[wasm_bindgen]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Clamp a value between min and max
#[wasm_bindgen]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}
