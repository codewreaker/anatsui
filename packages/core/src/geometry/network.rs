//! Vector network - graph-based vector representation
//!
//! Unlike paths (which are sequences of points), vector networks allow
//! any point to connect to any other point, enabling more intuitive
//! vector editing.

use super::{VectorPoint, VectorSegment};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

/// A region that can be filled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorRegion {
    /// Indices of segments forming this region (in order)
    pub segments: Vec<u32>,
    /// Whether this region should be filled
    pub filled: bool,
}

/// A vector network is a graph of points connected by segments
#[wasm_bindgen]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VectorNetwork {
    points: Vec<VectorPoint>,
    segments: Vec<VectorSegment>,
    regions: Vec<VectorRegion>,
    /// Map from point index to connected segment indices
    point_connections: HashMap<u32, Vec<u32>>,
}

#[wasm_bindgen]
impl VectorNetwork {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a point to the network
    pub fn add_point(&mut self, x: f32, y: f32) -> u32 {
        let index = self.points.len() as u32;
        self.points.push(VectorPoint::new(x, y));
        self.point_connections.insert(index, Vec::new());
        index
    }

    /// Add a point with handles
    pub fn add_point_with_handles(&mut self, x: f32, y: f32, in_x: f32, in_y: f32, out_x: f32, out_y: f32) -> u32 {
        let index = self.points.len() as u32;
        self.points.push(VectorPoint::with_handles(x, y, in_x, in_y, out_x, out_y));
        self.point_connections.insert(index, Vec::new());
        index
    }

    /// Connect two points with a segment
    pub fn connect(&mut self, start: u32, end: u32) -> Option<u32> {
        if start >= self.points.len() as u32 || end >= self.points.len() as u32 {
            return None;
        }
        
        // Check if already connected
        let segment = VectorSegment::new(start, end);
        if self.segments.contains(&segment) {
            return None;
        }
        
        let segment_index = self.segments.len() as u32;
        self.segments.push(segment);
        
        // Update connections
        self.point_connections.entry(start).or_default().push(segment_index);
        self.point_connections.entry(end).or_default().push(segment_index);
        
        Some(segment_index)
    }

    /// Disconnect two points
    pub fn disconnect(&mut self, start: u32, end: u32) -> bool {
        let segment = VectorSegment::new(start, end);
        if let Some(index) = self.segments.iter().position(|s| s == &segment) {
            self.segments.remove(index);
            
            // Update connections (need to recompute indices after removal)
            self.rebuild_connections();
            
            // Remove regions that use this segment
            self.regions.retain(|r| !r.segments.contains(&(index as u32)));
            
            true
        } else {
            false
        }
    }

    /// Get a point by index
    pub fn get_point(&self, index: u32) -> Option<VectorPoint> {
        self.points.get(index as usize).cloned()
    }

    /// Move a point
    pub fn move_point(&mut self, index: u32, x: f32, y: f32) {
        if let Some(point) = self.points.get_mut(index as usize) {
            point.x = x;
            point.y = y;
        }
    }

    /// Translate a point by delta
    pub fn translate_point(&mut self, index: u32, dx: f32, dy: f32) {
        if let Some(point) = self.points.get_mut(index as usize) {
            point.translate(dx, dy);
        }
    }

    /// Delete a point and all its connections
    pub fn delete_point(&mut self, index: u32) {
        if index >= self.points.len() as u32 {
            return;
        }
        
        // Remove all segments connected to this point
        self.segments.retain(|s| !s.connects_to(index));
        
        // Remove the point (this invalidates indices!)
        self.points.remove(index as usize);
        
        // Update all segment indices that reference higher indices
        for segment in &mut self.segments {
            if segment.start > index {
                segment.start -= 1;
            }
            if segment.end > index {
                segment.end -= 1;
            }
        }
        
        // Rebuild connections
        self.rebuild_connections();
        
        // Clear regions (would need to be recalculated)
        self.regions.clear();
    }

    /// Get number of points
    pub fn point_count(&self) -> u32 {
        self.points.len() as u32
    }

    /// Get number of segments
    pub fn segment_count(&self) -> u32 {
        self.segments.len() as u32
    }

    /// Get segments connected to a point
    pub fn get_connected_segments(&self, point_index: u32) -> Vec<u32> {
        self.point_connections.get(&point_index).cloned().unwrap_or_default()
    }

    /// Get points connected to a point (neighbors)
    pub fn get_neighbors(&self, point_index: u32) -> Vec<u32> {
        let mut neighbors = Vec::new();
        if let Some(segment_indices) = self.point_connections.get(&point_index) {
            for &seg_idx in segment_indices {
                if let Some(segment) = self.segments.get(seg_idx as usize) {
                    if let Some(other) = segment.other_point(point_index) {
                        neighbors.push(other);
                    }
                }
            }
        }
        neighbors
    }

    /// Find point near a coordinate (for hit testing)
    pub fn find_point_near(&self, x: f32, y: f32, threshold: f32) -> Option<u32> {
        for (index, point) in self.points.iter().enumerate() {
            if point.distance_to_coord(x, y) <= threshold {
                return Some(index as u32);
            }
        }
        None
    }

    /// Toggle fill for a region containing a point
    pub fn toggle_fill_at(&mut self, x: f32, y: f32) {
        // Find which region contains this point
        // This is a simplified implementation
        // A proper implementation would trace the boundary
        
        // For now, auto-detect and toggle regions
        if self.regions.is_empty() {
            self.detect_regions();
        }
        
        // Toggle first region (simplified)
        if let Some(region) = self.regions.first_mut() {
            region.filled = !region.filled;
        }
    }

    /// Detect enclosed regions in the network
    fn detect_regions(&mut self) {
        // This is a simplified region detection
        // A proper implementation would use a cycle-finding algorithm
        
        self.regions.clear();
        
        // If we have at least 3 segments forming a cycle, create a region
        if self.segments.len() >= 3 {
            // Check if segments form a closed path
            let mut visited: HashSet<u32> = HashSet::new();
            let mut path: Vec<u32> = Vec::new();
            
            if let Some(first_segment) = self.segments.first() {
                let mut current = first_segment.start;
                let start = current;
                
                for i in 0..self.segments.len() {
                    path.push(i as u32);
                    visited.insert(i as u32);
                    
                    let segment = &self.segments[i];
                    current = segment.other_point(current).unwrap_or(current);
                    
                    if current == start && path.len() >= 3 {
                        // Found a cycle
                        self.regions.push(VectorRegion {
                            segments: path.clone(),
                            filled: true,
                        });
                        break;
                    }
                }
            }
        }
    }

    fn rebuild_connections(&mut self) {
        self.point_connections.clear();
        
        for (i, segment) in self.segments.iter().enumerate() {
            self.point_connections.entry(segment.start).or_default().push(i as u32);
            self.point_connections.entry(segment.end).or_default().push(i as u32);
        }
    }

    /// Create a rectangle network
    pub fn from_rectangle(x: f32, y: f32, width: f32, height: f32) -> Self {
        let mut network = Self::new();
        
        let p0 = network.add_point(x, y);
        let p1 = network.add_point(x + width, y);
        let p2 = network.add_point(x + width, y + height);
        let p3 = network.add_point(x, y + height);
        
        network.connect(p0, p1);
        network.connect(p1, p2);
        network.connect(p2, p3);
        network.connect(p3, p0);
        
        // Auto-fill the rectangle
        network.regions.push(VectorRegion {
            segments: vec![0, 1, 2, 3],
            filled: true,
        });
        
        network
    }

    /// Create an ellipse network (approximated with bezier curves)
    pub fn from_ellipse(cx: f32, cy: f32, rx: f32, ry: f32) -> Self {
        let mut network = Self::new();
        
        // 4 points with bezier handles to approximate ellipse
        let k = 0.5522847498; // Magic number for bezier approximation of circle
        let kx = rx * k;
        let ky = ry * k;
        
        let p0 = network.add_point_with_handles(cx + rx, cy, 0.0, -ky, 0.0, ky);
        let p1 = network.add_point_with_handles(cx, cy + ry, kx, 0.0, -kx, 0.0);
        let p2 = network.add_point_with_handles(cx - rx, cy, 0.0, ky, 0.0, -ky);
        let p3 = network.add_point_with_handles(cx, cy - ry, -kx, 0.0, kx, 0.0);
        
        network.connect(p0, p1);
        network.connect(p1, p2);
        network.connect(p2, p3);
        network.connect(p3, p0);
        
        network.regions.push(VectorRegion {
            segments: vec![0, 1, 2, 3],
            filled: true,
        });
        
        network
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<VectorNetwork, JsValue> {
        serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

impl VectorNetwork {
    /// Get all points (for internal use)
    pub fn points(&self) -> &[VectorPoint] {
        &self.points
    }

    /// Get all segments (for internal use)
    pub fn segments(&self) -> &[VectorSegment] {
        &self.segments
    }

    /// Get all regions (for internal use)
    pub fn regions(&self) -> &[VectorRegion] {
        &self.regions
    }
}
