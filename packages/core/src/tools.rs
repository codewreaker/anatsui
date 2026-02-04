//! Design tools for Anatsui
//!
//! Tools for selecting, drawing, and editing elements.

use wasm_bindgen::prelude::*;
use crate::document::ObjectId;

/// Tool types available in Anatsui
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolType {
    /// Selection/move tool (V)
    Select,
    /// Frame tool (F)
    Frame,
    /// Rectangle tool (R)
    Rectangle,
    /// Ellipse tool (O)
    Ellipse,
    /// Line tool (L)
    Line,
    /// Pen tool for vector editing (P)
    Pen,
    /// Text tool (T)
    Text,
    /// Hand/pan tool (H)
    Hand,
    /// Zoom tool (Z)
    Zoom,
}

/// State of the current tool
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ToolState {
    tool: ToolType,
    /// Is the tool currently being used (mouse down)
    active: bool,
    /// Start position of the action
    start_x: f32,
    start_y: f32,
    /// Current position
    current_x: f32,
    current_y: f32,
    /// Selected objects
    selection: Vec<ObjectId>,
}

#[wasm_bindgen]
impl ToolState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            tool: ToolType::Select,
            active: false,
            start_x: 0.0,
            start_y: 0.0,
            current_x: 0.0,
            current_y: 0.0,
            selection: Vec::new(),
        }
    }

    pub fn tool(&self) -> ToolType {
        self.tool
    }

    pub fn set_tool(&mut self, tool: ToolType) {
        self.tool = tool;
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn start_x(&self) -> f32 {
        self.start_x
    }

    pub fn start_y(&self) -> f32 {
        self.start_y
    }

    pub fn current_x(&self) -> f32 {
        self.current_x
    }

    pub fn current_y(&self) -> f32 {
        self.current_y
    }

    /// Begin a tool action
    pub fn begin(&mut self, x: f32, y: f32) {
        self.active = true;
        self.start_x = x;
        self.start_y = y;
        self.current_x = x;
        self.current_y = y;
    }

    /// Update the current position during an action
    pub fn update(&mut self, x: f32, y: f32) {
        self.current_x = x;
        self.current_y = y;
    }

    /// End a tool action
    pub fn end(&mut self, x: f32, y: f32) {
        self.current_x = x;
        self.current_y = y;
        self.active = false;
    }

    /// Cancel a tool action
    pub fn cancel(&mut self) {
        self.active = false;
    }

    /// Get the drag delta
    pub fn delta_x(&self) -> f32 {
        self.current_x - self.start_x
    }

    pub fn delta_y(&self) -> f32 {
        self.current_y - self.start_y
    }

    /// Get the bounding box of the current drag
    pub fn drag_bounds(&self) -> (f32, f32, f32, f32) {
        let x = self.start_x.min(self.current_x);
        let y = self.start_y.min(self.current_y);
        let width = (self.current_x - self.start_x).abs();
        let height = (self.current_y - self.start_y).abs();
        (x, y, width, height)
    }

    /// Get selection count
    pub fn selection_count(&self) -> usize {
        self.selection.len()
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }

    /// Check if object is selected
    pub fn is_selected(&self, id: ObjectId) -> bool {
        self.selection.contains(&id)
    }
}

impl ToolState {
    /// Add to selection
    pub fn add_to_selection(&mut self, id: ObjectId) {
        if !self.selection.contains(&id) {
            self.selection.push(id);
        }
    }

    /// Remove from selection
    pub fn remove_from_selection(&mut self, id: ObjectId) {
        self.selection.retain(|&s| s != id);
    }

    /// Set selection to single object
    pub fn set_selection(&mut self, id: ObjectId) {
        self.selection.clear();
        self.selection.push(id);
    }

    /// Get selection
    pub fn selection(&self) -> &[ObjectId] {
        &self.selection
    }
}

impl Default for ToolState {
    fn default() -> Self {
        Self::new()
    }
}

/// Hit test result
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum HitType {
    None,
    Object,
    Handle,
    Edge,
}

/// Hit test result with details
#[wasm_bindgen]
pub struct HitResult {
    hit_type: HitType,
    object_id: Option<ObjectId>,
    handle_index: Option<u32>,
}

#[wasm_bindgen]
impl HitResult {
    pub fn none() -> Self {
        Self {
            hit_type: HitType::None,
            object_id: None,
            handle_index: None,
        }
    }

    pub fn object(id: ObjectId) -> Self {
        Self {
            hit_type: HitType::Object,
            object_id: Some(id),
            handle_index: None,
        }
    }

    pub fn handle(id: ObjectId, index: u32) -> Self {
        Self {
            hit_type: HitType::Handle,
            object_id: Some(id),
            handle_index: Some(index),
        }
    }

    pub fn hit_type(&self) -> HitType {
        self.hit_type
    }

    pub fn object_id(&self) -> Option<ObjectId> {
        self.object_id
    }

    pub fn handle_index(&self) -> Option<u32> {
        self.handle_index
    }
}
