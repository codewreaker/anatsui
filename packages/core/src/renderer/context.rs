//! WebGL rendering context wrapper

use crate::document::Color;
use crate::math::Rect;
use crate::renderer::Viewport;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram, WebGlBuffer, WebGlVertexArrayObject};

/// WebGL rendering context
pub struct RenderContext {
    gl: GL,
    canvas: HtmlCanvasElement,
    width: u32,
    height: u32,
    // Shaders and programs
    rect_program: WebGlProgram,
    ellipse_program: WebGlProgram,
    line_program: WebGlProgram,
    // Buffers
    quad_vao: WebGlVertexArrayObject,
    quad_buffer: WebGlBuffer,
}

impl RenderContext {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {
        let gl = canvas
            .get_context("webgl2")?
            .ok_or("Failed to get WebGL2 context")?
            .dyn_into::<GL>()?;

        // Enable blending for transparency
        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        
        // Create shader programs
        let rect_program = create_rect_program(&gl)?;
        let ellipse_program = create_ellipse_program(&gl)?;
        let line_program = create_line_program(&gl)?;
        
        // Create quad geometry
        let (quad_vao, quad_buffer) = create_quad_geometry(&gl)?;
        
        let width = canvas.width();
        let height = canvas.height();
        
        Ok(Self {
            gl,
            canvas,
            width,
            height,
            rect_program,
            ellipse_program,
            line_program,
            quad_vao,
            quad_buffer,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.canvas.set_width(width);
        self.canvas.set_height(height);
        self.width = width;
        self.height = height;
        self.gl.viewport(0, 0, width as i32, height as i32);
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn clear(&self, color: Color) {
        self.gl.clear_color(color.r, color.g, color.b, color.a);
        self.gl.clear(GL::COLOR_BUFFER_BIT);
    }

    pub fn flush(&self) {
        self.gl.flush();
    }

    pub fn draw_rect(&self, rect: Rect, color: Color, viewport: &Viewport, corner_radius: f32) {
        self.gl.use_program(Some(&self.rect_program));
        self.gl.bind_vertex_array(Some(&self.quad_vao));
        
        // Set uniforms
        let resolution_loc = self.gl.get_uniform_location(&self.rect_program, "u_resolution");
        let rect_loc = self.gl.get_uniform_location(&self.rect_program, "u_rect");
        let color_loc = self.gl.get_uniform_location(&self.rect_program, "u_color");
        let viewport_loc = self.gl.get_uniform_location(&self.rect_program, "u_viewport");
        let radius_loc = self.gl.get_uniform_location(&self.rect_program, "u_cornerRadius");
        
        self.gl.uniform2f(resolution_loc.as_ref(), self.width as f32, self.height as f32);
        self.gl.uniform4f(rect_loc.as_ref(), rect.x, rect.y, rect.width, rect.height);
        self.gl.uniform4f(color_loc.as_ref(), color.r, color.g, color.b, color.a);
        self.gl.uniform3f(viewport_loc.as_ref(), viewport.x, viewport.y, viewport.zoom);
        self.gl.uniform1f(radius_loc.as_ref(), corner_radius);
        
        self.gl.draw_arrays(GL::TRIANGLES, 0, 6);
    }

    pub fn draw_rect_stroke(&self, rect: Rect, color: Color, viewport: &Viewport, stroke_width: f32) {
        // Draw four thin rectangles for the stroke
        let sw = stroke_width / viewport.zoom;
        
        // Top
        self.draw_rect(Rect::new(rect.x - sw, rect.y - sw, rect.width + sw * 2.0, sw), color, viewport, 0.0);
        // Bottom
        self.draw_rect(Rect::new(rect.x - sw, rect.y + rect.height, rect.width + sw * 2.0, sw), color, viewport, 0.0);
        // Left
        self.draw_rect(Rect::new(rect.x - sw, rect.y, sw, rect.height), color, viewport, 0.0);
        // Right
        self.draw_rect(Rect::new(rect.x + rect.width, rect.y, sw, rect.height), color, viewport, 0.0);
    }

    pub fn draw_ellipse(&self, x: f32, y: f32, width: f32, height: f32, color: Color, viewport: &Viewport) {
        self.gl.use_program(Some(&self.ellipse_program));
        self.gl.bind_vertex_array(Some(&self.quad_vao));
        
        let resolution_loc = self.gl.get_uniform_location(&self.ellipse_program, "u_resolution");
        let rect_loc = self.gl.get_uniform_location(&self.ellipse_program, "u_rect");
        let color_loc = self.gl.get_uniform_location(&self.ellipse_program, "u_color");
        let viewport_loc = self.gl.get_uniform_location(&self.ellipse_program, "u_viewport");
        
        self.gl.uniform2f(resolution_loc.as_ref(), self.width as f32, self.height as f32);
        self.gl.uniform4f(rect_loc.as_ref(), x, y, width, height);
        self.gl.uniform4f(color_loc.as_ref(), color.r, color.g, color.b, color.a);
        self.gl.uniform3f(viewport_loc.as_ref(), viewport.x, viewport.y, viewport.zoom);
        
        self.gl.draw_arrays(GL::TRIANGLES, 0, 6);
    }

    pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color, viewport: &Viewport, width: f32) {
        self.gl.use_program(Some(&self.line_program));
        self.gl.bind_vertex_array(Some(&self.quad_vao));
        
        let resolution_loc = self.gl.get_uniform_location(&self.line_program, "u_resolution");
        let start_loc = self.gl.get_uniform_location(&self.line_program, "u_start");
        let end_loc = self.gl.get_uniform_location(&self.line_program, "u_end");
        let color_loc = self.gl.get_uniform_location(&self.line_program, "u_color");
        let viewport_loc = self.gl.get_uniform_location(&self.line_program, "u_viewport");
        let width_loc = self.gl.get_uniform_location(&self.line_program, "u_width");
        
        self.gl.uniform2f(resolution_loc.as_ref(), self.width as f32, self.height as f32);
        self.gl.uniform2f(start_loc.as_ref(), x1, y1);
        self.gl.uniform2f(end_loc.as_ref(), x2, y2);
        self.gl.uniform4f(color_loc.as_ref(), color.r, color.g, color.b, color.a);
        self.gl.uniform3f(viewport_loc.as_ref(), viewport.x, viewport.y, viewport.zoom);
        self.gl.uniform1f(width_loc.as_ref(), width);
        
        self.gl.draw_arrays(GL::TRIANGLES, 0, 6);
    }
}

fn create_rect_program(gl: &GL) -> Result<WebGlProgram, JsValue> {
    let vertex_shader = compile_shader(gl, GL::VERTEX_SHADER, RECT_VERTEX_SHADER)?;
    let fragment_shader = compile_shader(gl, GL::FRAGMENT_SHADER, RECT_FRAGMENT_SHADER)?;
    link_program(gl, &vertex_shader, &fragment_shader)
}

fn create_ellipse_program(gl: &GL) -> Result<WebGlProgram, JsValue> {
    let vertex_shader = compile_shader(gl, GL::VERTEX_SHADER, ELLIPSE_VERTEX_SHADER)?;
    let fragment_shader = compile_shader(gl, GL::FRAGMENT_SHADER, ELLIPSE_FRAGMENT_SHADER)?;
    link_program(gl, &vertex_shader, &fragment_shader)
}

fn create_line_program(gl: &GL) -> Result<WebGlProgram, JsValue> {
    let vertex_shader = compile_shader(gl, GL::VERTEX_SHADER, LINE_VERTEX_SHADER)?;
    let fragment_shader = compile_shader(gl, GL::FRAGMENT_SHADER, LINE_FRAGMENT_SHADER)?;
    link_program(gl, &vertex_shader, &fragment_shader)
}

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<web_sys::WebGlShader, JsValue> {
    let shader = gl.create_shader(shader_type).ok_or("Failed to create shader")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        let log = gl.get_shader_info_log(&shader).unwrap_or_default();
        gl.delete_shader(Some(&shader));
        Err(JsValue::from_str(&format!("Shader compilation failed: {}", log)))
    }
}

fn link_program(gl: &GL, vertex_shader: &web_sys::WebGlShader, fragment_shader: &web_sys::WebGlShader) -> Result<WebGlProgram, JsValue> {
    let program = gl.create_program().ok_or("Failed to create program")?;
    gl.attach_shader(&program, vertex_shader);
    gl.attach_shader(&program, fragment_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        let log = gl.get_program_info_log(&program).unwrap_or_default();
        gl.delete_program(Some(&program));
        Err(JsValue::from_str(&format!("Program linking failed: {}", log)))
    }
}

fn create_quad_geometry(gl: &GL) -> Result<(WebGlVertexArrayObject, WebGlBuffer), JsValue> {
    let vao = gl.create_vertex_array().ok_or("Failed to create VAO")?;
    gl.bind_vertex_array(Some(&vao));

    let buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

    // Full-screen quad (two triangles)
    let vertices: [f32; 12] = [
        -1.0, -1.0,
         1.0, -1.0,
        -1.0,  1.0,
        -1.0,  1.0,
         1.0, -1.0,
         1.0,  1.0,
    ];

    unsafe {
        let array = js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    }

    gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    Ok((vao, buffer))
}

// Shader sources
const RECT_VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;
layout(location = 0) in vec2 a_position;
uniform vec2 u_resolution;
uniform vec4 u_rect;
uniform vec3 u_viewport;
out vec2 v_uv;
out vec2 v_size;

void main() {
    // Convert rect to screen space
    vec2 pos = u_rect.xy * u_viewport.z + u_viewport.xy;
    vec2 size = u_rect.zw * u_viewport.z;
    
    // Map -1..1 to rect bounds
    vec2 p = pos + (a_position * 0.5 + 0.5) * size;
    
    // Convert to clip space
    vec2 clipSpace = (p / u_resolution) * 2.0 - 1.0;
    clipSpace.y = -clipSpace.y;
    
    gl_Position = vec4(clipSpace, 0.0, 1.0);
    v_uv = a_position * 0.5 + 0.5;
    v_size = size;
}
"#;

const RECT_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;
uniform vec4 u_color;
uniform float u_cornerRadius;
in vec2 v_uv;
in vec2 v_size;
out vec4 fragColor;

float roundedBoxSDF(vec2 p, vec2 b, float r) {
    vec2 q = abs(p) - b + r;
    return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - r;
}

void main() {
    if (u_cornerRadius > 0.0) {
        vec2 p = (v_uv - 0.5) * v_size;
        vec2 b = v_size * 0.5;
        float d = roundedBoxSDF(p, b, u_cornerRadius);
        float aa = 1.0 / min(v_size.x, v_size.y);
        float alpha = 1.0 - smoothstep(-aa, aa, d);
        fragColor = vec4(u_color.rgb, u_color.a * alpha);
    } else {
        fragColor = u_color;
    }
}
"#;

const ELLIPSE_VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;
layout(location = 0) in vec2 a_position;
uniform vec2 u_resolution;
uniform vec4 u_rect;
uniform vec3 u_viewport;
out vec2 v_uv;

void main() {
    vec2 pos = u_rect.xy * u_viewport.z + u_viewport.xy;
    vec2 size = u_rect.zw * u_viewport.z;
    vec2 p = pos + (a_position * 0.5 + 0.5) * size;
    vec2 clipSpace = (p / u_resolution) * 2.0 - 1.0;
    clipSpace.y = -clipSpace.y;
    gl_Position = vec4(clipSpace, 0.0, 1.0);
    v_uv = a_position;
}
"#;

const ELLIPSE_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;
uniform vec4 u_color;
in vec2 v_uv;
out vec4 fragColor;

void main() {
    float d = length(v_uv);
    float aa = fwidth(d);
    float alpha = 1.0 - smoothstep(1.0 - aa, 1.0 + aa, d);
    fragColor = vec4(u_color.rgb, u_color.a * alpha);
}
"#;

const LINE_VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;
layout(location = 0) in vec2 a_position;
uniform vec2 u_resolution;
uniform vec2 u_start;
uniform vec2 u_end;
uniform vec3 u_viewport;
uniform float u_width;
out vec2 v_uv;

void main() {
    vec2 start = u_start * u_viewport.z + u_viewport.xy;
    vec2 end = u_end * u_viewport.z + u_viewport.xy;
    
    vec2 dir = normalize(end - start);
    vec2 perp = vec2(-dir.y, dir.x);
    
    vec2 p;
    if (a_position.x < 0.0) {
        p = start;
    } else {
        p = end;
    }
    p += perp * a_position.y * u_width * 0.5;
    
    vec2 clipSpace = (p / u_resolution) * 2.0 - 1.0;
    clipSpace.y = -clipSpace.y;
    gl_Position = vec4(clipSpace, 0.0, 1.0);
    v_uv = a_position;
}
"#;

const LINE_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;
uniform vec4 u_color;
in vec2 v_uv;
out vec4 fragColor;

void main() {
    fragColor = u_color;
}
"#;
