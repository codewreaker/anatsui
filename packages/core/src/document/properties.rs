//! Property types and values

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Properties that can be set on nodes
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Property {
    // Transform
    X,
    Y,
    Width,
    Height,
    Rotation,
    
    // Appearance
    Opacity,
    Visible,
    Locked,
    
    // Fill
    FillColor,
    FillOpacity,
    
    // Stroke
    StrokeColor,
    StrokeWidth,
    StrokeOpacity,
    StrokeAlign,
    StrokeCap,
    StrokeJoin,
    
    // Corner
    CornerRadius,
    
    // Text
    Text,
    FontFamily,
    FontSize,
    FontWeight,
    FontStyle,
    TextAlign,
    LineHeight,
    LetterSpacing,
    
    // Effects
    BlurRadius,
    ShadowColor,
    ShadowOffsetX,
    ShadowOffsetY,
    ShadowBlur,
    ShadowSpread,
    
    // Layout
    LayoutMode,
    LayoutDirection,
    LayoutGap,
    LayoutPadding,
    LayoutAlign,
    
    // Metadata
    Name,
    Description,
    
    // Parent relationship (for tree structure)
    ParentId,
}

/// Values that properties can hold
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    Float(f32),
    Int(i32),
    Bool(bool),
    String(String),
    Color(Color),
    Vec2(f32, f32),
    Vec4(f32, f32, f32, f32),
}

/// RGBA color
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[wasm_bindgen]
impl Color {
    #[wasm_bindgen(constructor)]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let len = hex.len();
        
        if len == 6 || len == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
            let a = if len == 8 {
                u8::from_str_radix(&hex[6..8], 16).unwrap_or(255) as f32 / 255.0
            } else {
                1.0
            };
            Self { r, g, b, a }
        } else {
            Self::default()
        }
    }

    pub fn to_hex(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8
        )
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn transparent() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0, 1.0)
    }

    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }

    /// Convert to array for WebGL uniforms
    pub fn to_array(&self) -> Vec<f32> {
        vec![self.r, self.g, self.b, self.a]
    }

    /// Interpolate between two colors
    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
}
