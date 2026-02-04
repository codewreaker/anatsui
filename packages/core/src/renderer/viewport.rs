//! Viewport management

use wasm_bindgen::prelude::*;

/// Viewport state for the canvas
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

#[wasm_bindgen]
impl Viewport {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, zoom: f32) -> Self {
        Self { x, y, zoom }
    }

    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.zoom = 1.0;
    }

    /// Center on a point
    pub fn center_on(&mut self, canvas_x: f32, canvas_y: f32, screen_width: f32, screen_height: f32) {
        self.x = screen_width / 2.0 - canvas_x * self.zoom;
        self.y = screen_height / 2.0 - canvas_y * self.zoom;
    }

    /// Fit a rectangle in the viewport
    pub fn fit_rect(&mut self, x: f32, y: f32, width: f32, height: f32, screen_width: f32, screen_height: f32, padding: f32) {
        let zoom_x = (screen_width - padding * 2.0) / width;
        let zoom_y = (screen_height - padding * 2.0) / height;
        self.zoom = zoom_x.min(zoom_y).max(0.01).min(256.0);
        
        let center_x = x + width / 2.0;
        let center_y = y + height / 2.0;
        self.x = screen_width / 2.0 - center_x * self.zoom;
        self.y = screen_height / 2.0 - center_y * self.zoom;
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
}
