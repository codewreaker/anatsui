//! Shape rendering utilities - bezier curves, paths, etc.

use crate::math::Vec2;
use lyon::geom::{CubicBezierSegment, QuadraticBezierSegment, point};
use lyon::path::Path;
use lyon::tessellation::{BuffersBuilder, FillOptions, FillTessellator, VertexBuffers, StrokeOptions, StrokeTessellator};

/// Vertex for tessellated geometry
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 2],
}

/// Tessellate a path for filling
pub fn tessellate_fill(path: &Path) -> VertexBuffers<Vertex, u16> {
    let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();
    
    tessellator.tessellate_path(
        path,
        &FillOptions::default(),
        &mut BuffersBuilder::new(&mut geometry, |vertex: lyon::tessellation::FillVertex| {
            Vertex {
                position: [vertex.position().x, vertex.position().y],
            }
        }),
    ).ok();
    
    geometry
}

/// Tessellate a path for stroking
pub fn tessellate_stroke(path: &Path, line_width: f32) -> VertexBuffers<Vertex, u16> {
    let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();
    let mut tessellator = StrokeTessellator::new();
    
    tessellator.tessellate_path(
        path,
        &StrokeOptions::default().with_line_width(line_width),
        &mut BuffersBuilder::new(&mut geometry, |vertex: lyon::tessellation::StrokeVertex| {
            Vertex {
                position: [vertex.position().x, vertex.position().y],
            }
        }),
    ).ok();
    
    geometry
}

/// Build a rounded rectangle path
pub fn rounded_rect_path(x: f32, y: f32, width: f32, height: f32, radius: f32) -> Path {
    use lyon::path::builder::*;
    
    let r = radius.min(width / 2.0).min(height / 2.0);
    let mut builder = Path::builder();
    
    if r > 0.0 {
        // Start at top-left after the corner
        builder.begin(point(x + r, y));
        
        // Top edge
        builder.line_to(point(x + width - r, y));
        
        // Top-right corner
        builder.quadratic_bezier_to(point(x + width, y), point(x + width, y + r));
        
        // Right edge
        builder.line_to(point(x + width, y + height - r));
        
        // Bottom-right corner
        builder.quadratic_bezier_to(point(x + width, y + height), point(x + width - r, y + height));
        
        // Bottom edge
        builder.line_to(point(x + r, y + height));
        
        // Bottom-left corner
        builder.quadratic_bezier_to(point(x, y + height), point(x, y + height - r));
        
        // Left edge
        builder.line_to(point(x, y + r));
        
        // Top-left corner
        builder.quadratic_bezier_to(point(x, y), point(x + r, y));
        
        builder.close();
    } else {
        // Simple rectangle
        builder.begin(point(x, y));
        builder.line_to(point(x + width, y));
        builder.line_to(point(x + width, y + height));
        builder.line_to(point(x, y + height));
        builder.close();
    }
    
    builder.build()
}

/// Build an ellipse path
pub fn ellipse_path(cx: f32, cy: f32, rx: f32, ry: f32) -> Path {
    use lyon::path::builder::*;
    
    let mut builder = Path::builder();
    
    // Approximate ellipse with cubic beziers
    // Magic number for bezier control point distance: 0.5522847498
    let k = 0.5522847498;
    let kx = rx * k;
    let ky = ry * k;
    
    builder.begin(point(cx + rx, cy));
    
    // Top-right quadrant
    builder.cubic_bezier_to(
        point(cx + rx, cy - ky),
        point(cx + kx, cy - ry),
        point(cx, cy - ry),
    );
    
    // Top-left quadrant
    builder.cubic_bezier_to(
        point(cx - kx, cy - ry),
        point(cx - rx, cy - ky),
        point(cx - rx, cy),
    );
    
    // Bottom-left quadrant
    builder.cubic_bezier_to(
        point(cx - rx, cy + ky),
        point(cx - kx, cy + ry),
        point(cx, cy + ry),
    );
    
    // Bottom-right quadrant
    builder.cubic_bezier_to(
        point(cx + kx, cy + ry),
        point(cx + rx, cy + ky),
        point(cx + rx, cy),
    );
    
    builder.close();
    builder.build()
}
