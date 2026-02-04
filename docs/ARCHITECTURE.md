# Anatsui Architecture

> A Figma-inspired design tool built with Rust WebAssembly and modern web technologies

## Overview

Anatsui is a browser-based collaborative design tool that draws inspiration from Figma's revolutionary architecture. Like Figma, it uses a compiled language (Rust instead of C++) for its rendering engine, compiled to WebAssembly for the browser.

## Core Architecture Principles

### 1. Hybrid Architecture (Rust + TypeScript)

Following Figma's approach, Anatsui uses a hybrid architecture:

- **Rust/WebAssembly Core**: The document representation, rendering engine, and performance-critical operations
- **TypeScript/React UI**: The user interface layer surrounding the canvas

This separation allows us to:
- Heavily optimize the document representation for memory and speed
- Use modern UI technologies for fast iteration on the interface
- Control memory layout with compact representations
- Avoid garbage collection pauses in critical rendering paths

### 2. WebGL-Based Rendering Engine

Instead of relying on browser rendering (HTML/SVG/Canvas 2D), Anatsui implements its own WebGL rendering engine:

- **Tile-based rendering**: Efficient handling of large documents
- **GPU acceleration**: All rendering on the GPU
- **Anti-aliased vector graphics**: High-quality curve rendering
- **Custom text layout**: Consistent cross-platform rendering
- **Blend modes and effects**: Masking, blurring, opacity groups

### 3. Document Model

The document is represented as a tree structure similar to the DOM:

```
Document (root)
├── Page
│   ├── Frame
│   │   ├── Rectangle
│   │   ├── Text
│   │   └── Vector
│   └── Group
│       ├── Ellipse
│       └── Line
└── Page
    └── ...
```

Each object has:
- A unique ID (client ID + sequence number for conflict-free generation)
- A collection of properties with values
- Represented as `Map<ObjectID, Map<Property, Value>>`

### 4. Multiplayer Collaboration

Inspired by Figma's approach (not OT, but CRDT-inspired):

- **Last-Writer-Wins for properties**: Changes at property level, not object level
- **Fractional indexing**: For ordering children without conflicts
- **Client-server architecture**: Server is the source of truth
- **WebSocket-based sync**: Real-time updates over persistent connections

Key features:
- No locking - everyone can edit simultaneously
- Conflict resolution at the property level
- Offline support with reconnection sync

### 5. Vector Networks (vs Traditional Paths)

Unlike traditional vector tools that use paths (sequences of points), Anatsui implements vector networks:

- **Any point can connect to any other point**: Not limited to chains
- **Strokes work naturally**: Even at 3+ way intersections
- **Fills auto-detect enclosed regions**: Toggle any region on/off
- **Direct manipulation**: Bend curves by dragging them directly

## Technology Stack

### Frontend
- **Bun**: JavaScript runtime and package manager
- **React 18**: UI framework
- **TypeScript**: Type-safe JavaScript
- **WebGL 2**: GPU rendering
- **WebSocket**: Real-time communication

### Rust/WASM Core
- **Rust**: Systems programming language
- **wasm-bindgen**: JS-Rust interop
- **wasm-pack**: Build tool for Rust WASM
- **lyon**: Path tessellation
- **glam**: Linear algebra (vectors, matrices)
- **serde**: Serialization

### Backend (Multiplayer Server)
- **Bun**: HTTP/WebSocket server
- **Rust (optional)**: For performance-critical sync logic

## Performance Optimizations

### Memory Management
- Rust's ownership model prevents GC pauses
- Compact data representations (32-bit floats, byte-packed structs)
- Arena allocators for temporary objects
- Retained mode rendering (geometry cached on GPU)

### Rendering
- Tile-based rendering for large canvases
- Dirty rectangle tracking (only re-render what changed)
- Level-of-detail (LOD) for zoom levels
- Texture atlasing for repeated elements

### Loading
- Lazy loading of off-screen content
- Progressive document loading
- Binary serialization format (like Figma's Kiwi)
- WASM caching (near-instant subsequent loads)

## File Format

Anatsui uses a binary format inspired by Figma's approach:

1. **Schema-driven**: Property types defined in schema
2. **Compact encoding**: No redundant type information
3. **Efficient updates**: Can update individual properties
4. **Snapshot + deltas**: Full state + incremental changes

## Security Model

- WASM runs in browser sandbox
- No file system access (browser APIs only)
- All I/O through JavaScript interop
- Content Security Policy enforcement

## Future Considerations

- **Components and Variants**: Reusable design elements
- **Plugins API**: Extend functionality
- **Version History**: Time-travel through document states
- **Comments and Annotations**: Collaborative feedback
- **Prototyping**: Interactive prototypes
- **Dev Mode**: Developer handoff tools
