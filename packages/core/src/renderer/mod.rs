//! WebGL-based rendering engine for Anatsui
//!
//! Implements a custom 2D renderer using WebGL2, inspired by Figma's approach.

mod context;
mod shaders;
mod shapes;
mod viewport;

pub use context::*;
pub use shaders::*;
pub use shapes::*;
pub use viewport::*;

use crate::document::{Color, Document, Node, NodeType, ObjectId};
use crate::math::{Rect, Transform2D};
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext as GL, HtmlCanvasElement};

/// The main renderer for Anatsui
#[wasm_bindgen]
pub struct Renderer {
    context: RenderContext,
    viewport: Viewport,
    background_color: Color,
}

#[wasm_bindgen]
impl Renderer {
    /// Create a new renderer attached to a canvas element
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<Renderer, JsValue> {
        let context = RenderContext::new(canvas)?;
        let viewport = Viewport::new(0.0, 0.0, 1.0);
        
        Ok(Self {
            context,
            viewport,
            background_color: Color::from_hex("#F5F5F5"),
        })
    }

    /// Set the background color
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /// Resize the canvas
    pub fn resize(&mut self, width: u32, height: u32) {
        self.context.resize(width, height);
    }

    /// Get the viewport
    pub fn viewport(&self) -> Viewport {
        self.viewport.clone()
    }

    /// Set viewport position
    pub fn set_viewport_position(&mut self, x: f32, y: f32) {
        self.viewport.x = x;
        self.viewport.y = y;
    }

    /// Set viewport zoom
    pub fn set_viewport_zoom(&mut self, zoom: f32) {
        self.viewport.zoom = zoom.max(0.01).min(256.0);
    }

    /// Pan the viewport
    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.viewport.x += dx / self.viewport.zoom;
        self.viewport.y += dy / self.viewport.zoom;
    }

    /// Zoom at a point
    pub fn zoom_at(&mut self, x: f32, y: f32, delta: f32) {
        let old_zoom = self.viewport.zoom;
        let new_zoom = (old_zoom * (1.0 + delta * 0.1)).max(0.01).min(256.0);
        
        // Adjust position to keep the point under cursor
        let scale_change = new_zoom / old_zoom;
        self.viewport.x = x - (x - self.viewport.x) * scale_change;
        self.viewport.y = y - (y - self.viewport.y) * scale_change;
        self.viewport.zoom = new_zoom;
    }

    /// Convert screen coordinates to canvas coordinates
    pub fn screen_to_canvas(&self, screen_x: f32, screen_y: f32) -> Vec<f32> {
        let canvas_x = (screen_x - self.viewport.x) / self.viewport.zoom;
        let canvas_y = (screen_y - self.viewport.y) / self.viewport.zoom;
        vec![canvas_x, canvas_y]
    }

    /// Convert canvas coordinates to screen coordinates
    pub fn canvas_to_screen(&self, canvas_x: f32, canvas_y: f32) -> Vec<f32> {
        let screen_x = canvas_x * self.viewport.zoom + self.viewport.x;
        let screen_y = canvas_y * self.viewport.zoom + self.viewport.y;
        vec![screen_x, screen_y]
    }

    /// Clear the canvas
    pub fn clear(&self) {
        self.context.clear(self.background_color);
    }

    /// Render a document
    pub fn render_document(&mut self, document: &Document) {
        self.clear();
        
        let root_id = document.root_id();
        self.render_node_recursive(document, root_id);
    }

    fn render_node_recursive(&mut self, document: &Document, node_id: ObjectId) {
        if let Some(node) = document.get_node(node_id) {
            if !node.visible() {
                return;
            }
            
            // Render this node
            self.render_node(&node);
            
            // Render children
            for child_id in document.get_children(node_id) {
                self.render_node_recursive(document, child_id);
            }
        }
    }

    /// Render a single node
    pub fn render_node(&mut self, node: &Node) {
        let x = node.x();
        let y = node.y();
        let width = node.width();
        let height = node.height();
        
        match node.node_type() {
            NodeType::Rectangle => {
                self.draw_rectangle(x, y, width, height, node.fill_color(), node.corner_radius());
                if node.stroke_width() > 0.0 {
                    self.draw_rectangle_stroke(x, y, width, height, node.stroke_color(), node.stroke_width());
                }
            }
            NodeType::Ellipse => {
                self.draw_ellipse(x, y, width, height, node.fill_color());
            }
            NodeType::Frame => {
                // Frames have a background
                self.draw_rectangle(x, y, width, height, Color::white(), 0.0);
                // And a subtle border
                self.draw_rectangle_stroke(x, y, width, height, Color::from_hex("#E0E0E0"), 1.0);
            }
            NodeType::Text => {
                // Text rendering is handled separately
                // For now, draw a placeholder
                let text_color = node.fill_color();
                self.draw_text_placeholder(x, y, width, 20.0, text_color);
            }
            NodeType::Line => {
                // Draw a line
                let stroke_color = node.stroke_color();
                let stroke_width = node.stroke_width().max(1.0);
                self.draw_line(x, y, x + width, y + height, stroke_color, stroke_width);
            }
            _ => {
                // Default: draw as rectangle
                self.draw_rectangle(x, y, width, height, node.fill_color(), 0.0);
            }
        }
    }

    /// Draw a filled rectangle
    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color, corner_radius: f32) {
        let rect = Rect::new(x, y, width, height);
        self.context.draw_rect(rect, color, &self.viewport, corner_radius);
    }

    /// Draw a rectangle stroke
    pub fn draw_rectangle_stroke(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color, stroke_width: f32) {
        let rect = Rect::new(x, y, width, height);
        self.context.draw_rect_stroke(rect, color, &self.viewport, stroke_width);
    }

    /// Draw a filled ellipse
    pub fn draw_ellipse(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        self.context.draw_ellipse(x, y, width, height, color, &self.viewport);
    }

    /// Draw a line
    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color, width: f32) {
        self.context.draw_line(x1, y1, x2, y2, color, &self.viewport, width);
    }

    /// Draw text placeholder (actual text rendering TBD)
    pub fn draw_text_placeholder(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        // For now, just draw a small colored rectangle as placeholder
        self.context.draw_rect(Rect::new(x, y, width, height), color, &self.viewport, 0.0);
    }

    /// Draw selection handles around a node
    pub fn draw_selection(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let handle_size = 8.0 / self.viewport.zoom;
        let half_handle = handle_size / 2.0;
        
        // Draw selection border
        self.draw_rectangle_stroke(x, y, width, height, Color::from_hex("#0D99FF"), 2.0 / self.viewport.zoom);
        
        // Draw corner handles
        let corners = [
            (x - half_handle, y - half_handle),
            (x + width - half_handle, y - half_handle),
            (x - half_handle, y + height - half_handle),
            (x + width - half_handle, y + height - half_handle),
        ];
        
        for (hx, hy) in corners {
            self.draw_rectangle(hx, hy, handle_size, handle_size, Color::white(), 0.0);
            self.draw_rectangle_stroke(hx, hy, handle_size, handle_size, Color::from_hex("#0D99FF"), 1.0 / self.viewport.zoom);
        }
        
        // Draw edge handles
        let edges = [
            (x + width / 2.0 - half_handle, y - half_handle),
            (x + width / 2.0 - half_handle, y + height - half_handle),
            (x - half_handle, y + height / 2.0 - half_handle),
            (x + width - half_handle, y + height / 2.0 - half_handle),
        ];
        
        for (hx, hy) in edges {
            self.draw_rectangle(hx, hy, handle_size, handle_size, Color::white(), 0.0);
            self.draw_rectangle_stroke(hx, hy, handle_size, handle_size, Color::from_hex("#0D99FF"), 1.0 / self.viewport.zoom);
        }
    }

    /// Draw a selection rectangle (for marquee selection)
    pub fn draw_selection_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let fill_color = Color::new(0.05, 0.6, 1.0, 0.1);
        let stroke_color = Color::from_hex("#0D99FF");
        
        self.draw_rectangle(x, y, width, height, fill_color, 0.0);
        self.draw_rectangle_stroke(x, y, width, height, stroke_color, 1.0);
    }

    /// Begin a frame (for animation)
    pub fn begin_frame(&mut self) {
        self.clear();
    }

    /// End a frame
    pub fn end_frame(&self) {
        self.context.flush();
    }

    /// Get performance stats
    pub fn get_stats(&self) -> String {
        format!(
            "Viewport: ({:.1}, {:.1}) @ {:.2}x",
            self.viewport.x, self.viewport.y, self.viewport.zoom
        )
    }
}
