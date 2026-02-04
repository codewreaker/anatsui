# Anatsui

> A professional-grade collaborative design tool inspired by Figma, built with Rust/WebAssembly for maximum performance.

![Anatsui](https://img.shields.io/badge/version-0.1.0-blue)
![Rust](https://img.shields.io/badge/Rust-WASM-orange)
![Bun](https://img.shields.io/badge/Bun-Runtime-black)
![License](https://img.shields.io/badge/license-MIT-green)

## 🎨 Overview

Anatsui is a modern collaborative design tool that pushes the boundaries of what's possible in a web browser. Unlike traditional web-based design tools, Anatsui uses:

- **Rust + WebAssembly** for the rendering engine (instead of C++)
- **WebGL2** for GPU-accelerated 2D graphics
- **Vector Networks** (Figma's innovation) instead of traditional paths
- **CRDT-inspired sync** for seamless real-time collaboration
- **Bun** for lightning-fast development and runtime

Named after [El Anatsui](https://en.wikipedia.org/wiki/El_Anatsui), the renowned Ghanaian sculptor known for transforming simple materials into extraordinary art.

## ✨ Features

- **High-Performance Rendering**: Rust/WASM core with WebGL2 for smooth 60fps
- **Real-time Collaboration**: Multiple users can edit simultaneously
- **Vector Networks**: Advanced path editing with any-to-any connections
- **Modern UI**: React + TypeScript with Tailwind CSS
- **Infinite Canvas**: Smooth panning and zooming
- **Design Tools**: Rectangle, ellipse, line, frame, text, and pen tools
- **Layer System**: Full layer management with visibility and locking
- **Properties Panel**: Edit position, size, fill, stroke, and more

## 🏗️ Architecture

```
anatsui/
├── packages/
│   └── core/               # Rust/WASM rendering engine
│       ├── src/
│       │   ├── document/   # Document model with fractional indexing
│       │   ├── geometry/   # Vector networks implementation
│       │   ├── renderer/   # WebGL2 renderer with shaders
│       │   ├── multiplayer/# Sync engine and messaging
│       │   └── tools.rs    # Design tools (select, draw, etc.)
│       └── Cargo.toml
├── apps/
│   ├── web/               # React frontend
│   │   ├── src/
│   │   │   ├── components/# UI components
│   │   │   ├── store/     # Zustand state management
│   │   │   └── types/     # TypeScript types
│   │   └── package.json
│   └── server/            # Bun WebSocket server
│       └── src/
│           └── index.ts   # Multiplayer server
├── docs/
│   ├── ARCHITECTURE.md    # Technical architecture
│   └── TECHNICAL_INSIGHTS.md # Figma research
└── package.json           # Monorepo root
```

## 🚀 Quick Start

### Prerequisites

- [Bun](https://bun.sh) v1.0+ (for package management and server)
- [Rust](https://rustup.rs) v1.70+ (for WASM core)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (for building WASM)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/anatsui.git
cd anatsui

# Install dependencies
bun install
```

### Building the WASM Core

```bash
# Navigate to the core package
cd packages/core

# Build for web (release mode)
wasm-pack build --target web --release

# Or for development (faster builds)
wasm-pack build --target web --dev
```

### Running the Development Server

```bash
# Start the web app (from project root)
cd apps/web
bun dev

# In a separate terminal, start the multiplayer server
cd apps/server
bun dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

## 📖 Documentation

### Core Concepts

#### Document Model
The document is stored as a property-level map: `Map<ObjectID, Map<Property, Value>>`. This enables:
- Fine-grained conflict resolution
- Efficient syncing (only changed properties)
- Property-level undo/redo

#### Vector Networks
Unlike traditional SVG paths (sequences of points), vector networks are graphs where any point can connect to any other. This allows:
- Complex shapes from simple primitives
- Non-destructive boolean operations
- Intuitive direct manipulation

#### Rendering Pipeline
1. **Document changes** trigger re-renders
2. **Nodes are sorted** by z-index (fractional indexing)
3. **Geometry is tessellated** using Lyon
4. **WebGL draws** with custom shaders

#### Multiplayer Sync
- Property-level last-writer-wins (LWW)
- Fractional indexing for ordering
- WebSocket for real-time updates
- Optimistic updates with acknowledgements

### Keyboard Shortcuts

| Key | Tool |
|-----|------|
| V | Select |
| F | Frame |
| R | Rectangle |
| O | Ellipse |
| L | Line |
| P | Pen |
| T | Text |
| H | Hand (pan) |
| ⌘/Ctrl + = | Zoom in |
| ⌘/Ctrl + - | Zoom out |
| ⌘/Ctrl + 0 | Reset zoom |

## 🛠️ Development

### Project Structure

| Package | Description |
|---------|-------------|
| `@anatsui/core` | Rust/WASM rendering engine |
| `@anatsui/web` | React frontend application |
| `@anatsui/server` | Bun WebSocket server |

### Available Scripts

```bash
# Root level
bun dev          # Start web dev server
bun build        # Build all packages
bun test         # Run tests

# WASM Core (packages/core)
wasm-pack build  # Build WASM
cargo test       # Run Rust tests
cargo clippy     # Lint Rust code
cargo doc        # Generate docs

# Web App (apps/web)
bun dev          # Start dev server
bun build        # Production build
bun preview      # Preview production build

# Server (apps/server)
bun dev          # Start with hot reload
bun start        # Production start
```

### Tech Stack

- **Rendering**: Rust, wasm-bindgen, WebGL2
- **Math**: glam (linear algebra)
- **Tessellation**: Lyon (path tessellation)
- **Text**: fontdue (font rendering)
- **Serialization**: serde + serde_json
- **Frontend**: React 18, TypeScript, Tailwind CSS
- **State**: Zustand
- **Build**: Vite, wasm-pack
- **Runtime**: Bun

## 🧪 Testing

```bash
# Rust unit tests
cd packages/core
cargo test

# TypeScript tests (when implemented)
bun test
```

## 📦 Building for Production

```bash
# Build WASM core (optimized)
cd packages/core
wasm-pack build --target web --release

# Build frontend
cd apps/web
bun build

# Build server
cd apps/server
bun build
```

## 🤝 Contributing

Contributions are welcome! Please read our contributing guidelines before submitting a PR.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Figma](https://figma.com) for pioneering browser-based design tools
- [Evan Wallace](https://madebyevan.com) for the technical blog posts
- [El Anatsui](https://en.wikipedia.org/wiki/El_Anatsui) for inspiring the project name
- The Rust/WASM community for excellent tooling

---

<p align="center">
  Built with ❤️ and Rust
</p>
