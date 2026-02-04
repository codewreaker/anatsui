# Technical Deep Dive: Figma Insights Applied to Anatsui

## Summary of Key Figma Technical Insights

This document summarizes the research from Figma's engineering blog and how these insights inform Anatsui's architecture.

---

## 1. Rendering Engine Architecture

### Figma's Approach
- Custom WebGL-based rendering engine (not HTML/SVG/Canvas 2D)
- Tile-based engine with GPU acceleration
- Supports masking, blurring, dithered gradients, blend modes, nested opacity
- Looks like "a browser inside a browser" - has own DOM, compositor, text layout

### Why Browser Rendering Doesn't Work
1. **HTML/SVG baggage**: Slower than Canvas 2D, optimized for scrolling not zooming
2. **No GPU guarantee**: Many things still CPU-rendered
3. **Inconsistent masking/blurring**: Varies between browsers
4. **Canvas 2D is immediate mode**: Re-uploads geometry every frame
5. **Inconsistent text layout**: Different between browsers and platforms
6. **Missing features**: No angular gradients, etc.

### Anatsui Implementation
- Rust-based WebGL renderer compiled to WASM
- Lyon library for path tessellation
- Custom text shaping with fontdue/HarfBuzz
- Retained-mode rendering with geometry caching

---

## 2. WebAssembly Performance

### Figma's Results
- **3x faster load time** after switching from asm.js to WASM
- **10x faster serialization** after rewriting in Rust
- Near-instant subsequent loads due to WASM caching

### WASM Benefits
1. **Compact format**: Faster network transfer
2. **Fast parsing**: 20x faster than asm.js
3. **Pre-optimized by LLVM**: No browser optimization needed
4. **Native code caching**: Browser caches compiled code
5. **64-bit integer support**: No emulation overhead

### Anatsui Implementation
- Rust compiled to WASM via wasm-pack
- Use wasm-bindgen for JS interop
- Memory managed in Rust (no GC pauses)
- Binary format for document serialization

---

## 3. Multiplayer Technology

### Figma's Approach (Not OT, CRDT-Inspired)
- **Client-server architecture**: Server is authority
- **Property-level sync**: Changes atomic at property boundary
- **Last-writer-wins**: Server determines final order
- **No true CRDT overhead**: Simplified since centralized

### Document Structure
```
Map<ObjectID, Map<Property, Value>>
```
- Tree of objects (like DOM)
- Each object has ID + properties
- Conflicts only when same property on same object

### Object Creation
- Client-generated unique IDs (client ID + sequence)
- Server doesn't assign IDs (works offline)
- Deletion removes from server (undo buffer on client)

### Tree Synchronization
- Parent link stored as property on child
- Server rejects cycles
- **Fractional indexing** for ordering:
  - Position as fraction 0-1 exclusive
  - Insert = average of neighbors
  - Arbitrary precision strings

### Undo/Redo in Multiplayer
- Key principle: undo a lot, copy, redo should not change document
- Undo modifies redo history at time of undo
- Redo modifies undo history at time of redo

### Anatsui Implementation
- WebSocket-based sync layer
- Rust multiplayer service (can be separate process)
- Fractional indexing for child ordering
- Property-level conflict resolution

---

## 4. Vector Networks

### Traditional Paths Problem
- Sequence of points from start to end
- Can't have 3+ lines meeting at a point
- Connecting/disconnecting is awkward
- Winding rules for fills are confusing

### Vector Networks Solution
- Any point can connect to any other point
- Lines/curves between any two points
- Stroke caps/joins work at 3+ way junctions
- Fills: auto-fill enclosed regions, toggle with bucket tool
- Direct manipulation: bend curves by dragging them

### Anatsui Implementation
- Graph-based vector representation
- Nodes (points) and edges (curves)
- Region detection for fills
- Bend tool for direct curve manipulation

---

## 5. Text Rendering Complexity

### Challenges
1. **Script variation**: Different scripts have different rules
2. **Ligatures**: Shape depends on neighbors (HarfBuzz required)
3. **Font fallback**: Character-level, not font-level
4. **Emoji**: Multi-color, composite characters
5. **Anti-aliasing**: Subpixel AA not composable with transparency

### Font Fallback
- Per-character fallback through font cascade
- Platform adds hidden fallback fonts
- Last resort fonts (.LastResort, Noto, etc.)

### Text Pipeline
1. Styling (query fonts for scalars)
2. Layout (break into lines)
3. Shaping (compute glyphs + positions) - USE HARFBUZZ
4. Rasterization (glyph atlas cache)
5. Composition (render from atlas)

### Anatsui Implementation
- fontdue for rasterization
- HarfBuzz bindings for shaping
- Glyph atlas caching
- Consistent cross-platform rendering (don't use native text)

---

## 6. Performance Measurement

### Figma's Metrics
1. **Average frame time**: Smoothness measure
2. **Maximum frame time**: "Hitching" measure
3. Both matter for "direct manipulation" feel

### Benchmarking Approach
- Run in Electron with simulated user interactions
- Measure real design files, not synthetic tests
- Track on every commit
- Alert on regressions

### Anatsui Implementation
- Frame time measurement
- Benchmark suite with real-world documents
- Performance regression testing

---

## 7. Rust at Figma (Server-Side)

### Why Rust
- Low memory usage (can spawn process per document)
- 10x faster serialization
- No GC pauses
- LLVM optimizations

### Challenges Encountered
1. **Lifetimes confusing**: Simplified with single event loop
2. **Errors hard to debug**: Used string errors with line/column
3. **Early libraries**: Some had bugs, used C libraries instead
4. **Async complexity**: Kept networking in Node.js

### Anatsui Application
- Rust for WASM core (rendering, document)
- Rust for multiplayer server (optional)
- Simple ownership patterns
- Use mature dependencies

---

## 8. Data Corruption Debugging

### Emscripten/WASM Gotchas
- Unaligned memory access in WASM is undefined
- Pointer shifts round down, causing corruption
- SAFE_HEAP flag helps detect

### Lessons
- Use SAFE_HEAP in development
- Be careful with memory layout
- Binary format bugs are hard to track

---

## Summary: Key Takeaways for Anatsui

1. **Use WebGL directly** - Don't rely on browser rendering
2. **Hybrid architecture** - Rust/WASM core + TypeScript UI
3. **Property-level sync** - Not OT, simpler CRDT-inspired
4. **Fractional indexing** - Simple ordered sequence sync
5. **Vector networks** - Better than paths
6. **Custom text rendering** - Cross-platform consistency
7. **Frame time metrics** - Both average and max matter
8. **Simple patterns** - Avoid complexity, iterate quickly
