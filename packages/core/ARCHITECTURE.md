# Anatsui Core - Rust/WebAssembly Rendering Engine

## 📚 Study Guide for Understanding the Rust Architecture

This document explains how the Rust rendering engine works, aimed at helping you learn and contribute to the codebase.

## 🏗️ High-Level Architecture

```text
┌─────────────────────────────────────────────────────────────┐
│                    BROWSER (JavaScript/React)                │
│                                                               │
│  User Interactions → React Components → Zustand Store        │
└────────────────────────────┬────────────────────────────────┘
                             │ WASM Boundary
                             ↓
┌─────────────────────────────────────────────────────────────┐
│               ANATSUI CORE (Rust → WebAssembly)              │
│                                                               │
│  ┌──────────────┐   ┌───────────────┐   ┌────────────────┐ │
│  │  lib.rs      │   │  document/    │   │  geometry/     │ │
│  │  Entry Point │ → │  Scene Graph  │ → │  Vector Math   │ │
│  └──────────────┘   └───────────────┘   └────────────────┘ │
│                             ↓                                 │
│  ┌──────────────┐   ┌───────────────┐   ┌────────────────┐ │
│  │  renderer/   │ ← │  math/        │   │  tools/        │ │
│  │  WebGL2      │   │  Transforms   │   │  Drawing       │ │
│  └──────────────┘   └───────────────┘   └────────────────┘ │
│         ↓                                                     │
└─────────┼─────────────────────────────────────────────────────┘
          │ WebGL2 Calls
          ↓
┌─────────────────────────────────────────────────────────────┐
│                      GPU (Graphics Card)                      │
└─────────────────────────────────────────────────────────────┘
```

## 📁 Module Breakdown

### `/src/lib.rs` - Entry Point

**What it does:** Initializes the WASM module and exports functions to JavaScript.

**Key concepts:**
- `#[wasm_bindgen]` macro: Marks functions that can be called from JavaScript
- `init()`: Sets up panic hooks and logging
- `version()`: Returns the crate version

**How to read it:**
1. Start here to see what's exposed to JavaScript
2. Look for `#[wasm_bindgen]` - these are your public API
3. Module declarations (`pub mod document;`) show the code organization

---

### `/src/document/` - Scene Graph

**What it does:** Manages the tree structure of design objects (pages, frames, shapes).

**Files:**
- `mod.rs`: Public API and type definitions
- `tree.rs`: DocumentTree implementation with parent-child relationships
- `node.rs`: Individual node types (Rectangle, Ellipse, Frame, etc.)

**Key concepts:**

#### Parent-Child Relationships
```rust
// Each node knows its parent
parent_map: HashMap<ObjectId, ObjectId>
// "shape-a" → "frame-1"

// Each parent knows its children (ordered)
children_map: HashMap<ObjectId, Vec<ObjectId>>
// "frame-1" → ["shape-a", "shape-b", "shape-c"]
```

#### Fractional Indexing (Z-Order)
Instead of array indices, nodes have fractional order values:
```text
Node A: "0.5"
Node B: "0.75"
Node C: "0.875"

To insert between A and B:
new_order = (0.5 + 0.75) / 2 = "0.625"
```

**Why?** You can always insert between any two nodes without reordering everything.

#### How to explore:
```bash
# Read in this order:
1. src/document/mod.rs      # See the public types
2. src/document/tree.rs     # Understand the tree structure
3. src/document/node.rs     # Learn about different node types
```

---

### `/src/geometry/` - Vector Graphics

**What it does:** Handles bezier curves, vector paths, and hit testing.

**Files:**
- `mod.rs`: Public API
- `path.rs`: Bezier curve representation
- `network.rs`: Vector network (like Figma's pen tool)
- `hit_test.rs`: Check if a point is inside a shape

**Key concepts:**

#### Bezier Curves
```rust
struct CubicBezier {
    start: Point,      // Where the curve begins
    cp1: Point,        // First control point (handle)
    cp2: Point,        // Second control point (handle)
    end: Point,        // Where the curve ends
}
```

Visual explanation:
```text
      cp1 •
         /  \
start • ─────  curve  ───── • end
                     /  \
                    • cp2
```

The control points "pull" the curve toward them without actually touching it.

#### Hit Testing
To check if a mouse click hits a shape, we use:
1. **Bounding box test** (fast): Is point in rectangle?
2. **Precise test** (slower): Is point inside the actual shape?

**How to explore:**
```bash
1. src/geometry/path.rs      # Bezier curves
2. src/geometry/network.rs   # Pen tool data structure
3. src/geometry/hit_test.rs  # Click detection
```

---

### `/src/renderer/` - WebGL2 Rendering

**What it does:** Draws everything to the screen using the GPU.

**Files:**
- `mod.rs`: Renderer entry point
- `context.rs`: WebGL2 setup and state
- `shaders.rs`: GPU programs (vertex + fragment shaders)
- `shapes.rs`: Shape tessellation (converting curves to triangles)

**Key concepts:**

#### WebGL2 Pipeline
```text
1. CPU (Rust)                  2. GPU (WebGL2)
   ↓                              ↓
[Shapes] → [Triangles] → [Vertices] → [Vertex Shader] → [Fragment Shader] → [Screen]
                                          ↓                       ↓
                                    Transform to                Color each
                                    screen space                pixel
```

#### Why Triangles?
GPUs can only draw triangles. So we convert everything:
- **Rectangle** → 2 triangles
- **Ellipse** → 32 triangles arranged in a circle
- **Bezier curve** → Many small triangles along the path

This is called **tessellation**.

#### Vertex Shader (runs per-vertex)
```glsl
// Input: vertex position in world space
// Output: vertex position in screen space
void main() {
    // Apply camera transform (pan, zoom)
    vec2 pos = vertex_position * zoom + pan;
    
    // Convert to clip space (-1 to 1)
    gl_Position = vec4(pos, 0.0, 1.0);
}
```

#### Fragment Shader (runs per-pixel)
```glsl
// Input: interpolated color
// Output: pixel color
void main() {
    fragColor = vertex_color;
}
```

**How to explore:**
```bash
1. src/renderer/context.rs   # WebGL2 initialization
2. src/renderer/shaders.rs   # GPU programs
3. src/renderer/shapes.rs    # Triangle generation
```

---

### `/src/math/` - Mathematical Utilities

**What it does:** Vector math, transformations, bounding boxes.

**Files:**
- `vec2.rs`: 2D vectors (x, y)
- `transform.rs`: Translation, rotation, scale
- `rect.rs`: Axis-aligned bounding boxes
- `matrix.rs`: 2D transformation matrices

**Key concepts:**

#### Vector Math
```rust
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    // Add two vectors
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    
    // Length of vector (distance from origin)
    fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
```

#### Transformation Matrix
```text
[a  c  tx]   [x]   [a*x + c*y + tx]
[b  d  ty] × [y] = [b*x + d*y + ty]
[0  0  1 ]   [1]   [1             ]

Where:
- (a, d) = scale
- (b, c) = rotation/skew
- (tx, ty) = translation
```

**How to explore:**
```bash
1. src/math/vec2.rs        # Start here
2. src/math/transform.rs   # 2D transforms
3. src/math/matrix.rs      # Matrix operations
```

---

### `/src/tools/` - Drawing Tools

**What it does:** Implements tool logic (pen, rectangle, selection, etc.).

**Files:**
- `mod.rs`: Tool enum and common interfaces
- `pen.rs`: Bezier pen tool
- `shape_tool.rs`: Rectangle, ellipse, line tools
- `select.rs`: Selection and transformation

**Key concepts:**

#### Tool State Machine
```text
PenTool States:
    Idle → Clicking → DraggingHandle → PathComplete
     ↓         ↓            ↓              ↓
  [Start]  [Add Point] [Adjust Curve] [Create Node]
```

#### Event Flow
```rust
// Mouse down: Start drawing
tool.on_mouse_down(x, y);

// Mouse move: Update preview
tool.on_mouse_move(x, y);

// Mouse up: Finalize
tool.on_mouse_up(x, y);
```

**How to explore:**
```bash
1. src/tools/mod.rs        # Tool trait
2. src/tools/pen.rs        # Pen tool implementation
3. src/tools/select.rs     # Selection tool
```

---

### `/src/multiplayer/` - Collaborative Editing

**What it does:** Enables real-time collaboration using CRDTs.

**Files:**
- `mod.rs`: Multiplayer coordinator
- `crdt.rs`: Conflict-free replicated data types
- `message.rs`: Network message format
- `sync.rs`: State synchronization

**Key concepts:**

#### CRDT (Conflict-Free Replicated Data Type)
```text
User A: Moves rectangle to (100, 50)
User B: Changes rectangle color to red

Without CRDT:
  ❌ Conflict! Which change wins?

With CRDT:
  ✓ Both changes apply independently
  ✓ Eventual consistency guaranteed
  ✓ No central authority needed
```

#### How It Works
1. Each operation has a unique ID and timestamp
2. Operations are **commutative** (order doesn't matter)
3. All clients eventually converge to the same state

**How to explore:**
```bash
1. src/multiplayer/message.rs  # See message types
2. src/multiplayer/crdt.rs     # CRDT implementation
3. src/multiplayer/sync.rs     # State synchronization
```

---

## 🔄 Data Flow Example: Drawing a Rectangle

Let's trace what happens when you draw a rectangle:

### 1. **User Input** (Frontend - React)
```typescript
// Canvas.tsx
const handleMouseDown = (e) => {
  setDragStart({ x: e.clientX, y: e.clientY });
  setIsDragging(true);
};
```

### 2. **State Update** (Frontend - Zustand)
```typescript
// editorStore.ts
addNode: (node) => {
  const nodes = new Map(state.document.nodes);
  nodes.set(node.id, node);
  return { document: { ...state.document, nodes } };
}
```

### 3. **WASM Rendering** (Backend - Rust)
```rust
// renderer/shapes.rs
fn render_rectangle(rect: &Rectangle) -> Vec<Triangle> {
    // Convert rectangle to 2 triangles
    vec![
        Triangle { /* top-left, top-right, bottom-left */ },
        Triangle { /* bottom-left, top-right, bottom-right */ },
    ]
}
```

### 4. **GPU Drawing** (WebGL2)
```rust
// renderer/context.rs
gl.draw_arrays(GL::TRIANGLES, 0, vertex_count);
```

---

## 🎓 Learning Path

### Beginner (Start Here)
1. Read `/src/lib.rs` - See the public API
2. Explore `/src/document/tree.rs` - Understand the scene graph
3. Look at `/src/math/vec2.rs` - Basic vector math

### Intermediate
4. Study `/src/geometry/path.rs` - Bezier curves
5. Read `/src/renderer/context.rs` - WebGL2 setup
6. Examine `/src/tools/pen.rs` - Tool implementation

### Advanced
7. Deep dive into `/src/renderer/shaders.rs` - GPU programming
8. Explore `/src/multiplayer/crdt.rs` - Distributed systems
9. Optimize rendering performance - Batching, culling, etc.

---

## 🛠️ Common Development Tasks

### Adding a New Shape Type

1. **Add enum variant** in `src/document/node.rs`:
```rust
pub enum NodeType {
    Rectangle,
    Ellipse,
    // Add your shape here
    Triangle,
}
```

2. **Add rendering logic** in `src/renderer/shapes.rs`:
```rust
fn tessellate_triangle(tri: &Triangle) -> Vec<Vertex> {
    // Convert to GPU triangles
}
```

3. **Add tool** in `src/tools/`:
```rust
// src/tools/triangle_tool.rs
pub struct TriangleTool {
    // Tool state
}
```

### Adding a New Property

1. **Define in TypeScript**:
```typescript
// apps/web/src/types.ts
interface DesignNode {
    opacity: number;
    // Add your property
    borderWidth?: number;
}
```

2. **Add to Rust**:
```rust
// packages/core/src/document/node.rs
pub struct Node {
    pub opacity: f32,
    // Add your property
    pub border_width: Option<f32>,
}
```

3. **Handle in renderer**:
```rust
// packages/core/src/renderer/shapes.rs
if let Some(width) = node.border_width {
    // Render border
}
```

---

## 🐛 Debugging Tips

### Browser DevTools
```javascript
// Check if WASM loaded
console.log(await import('@anatsui/wasm'));

// Inspect memory
performance.memory
```

### Rust Logging
```rust
// Add to Cargo.toml
console_log = "0.2"

// In code
use console_log::log;
log!("Rectangle drawn at {:?}", position);
```

### WebGL Debugging
Use [Spector.js](https://spector.babylonjs.com/) to inspect WebGL calls:
1. Install Chrome extension
2. Capture a frame
3. See all GPU operations

---

## 📖 Further Reading

- **Rust Book**: https://doc.rust-lang.org/book/
- **WASM Book**: https://rustwasm.github.io/docs/book/
- **WebGL2 Tutorial**: https://webgl2fundamentals.org/
- **Figma Engineering**: https://www.figma.com/blog/section/engineering/
- **CRDTs Explained**: https://crdt.tech/

---

## 🤝 Contributing

When making changes:
1. Add detailed comments explaining **why**, not just what
2. Update this README if you add new modules
3. Write examples for complex algorithms
4. Test in different browsers

---

Happy coding! 🎨✨
