import { create } from 'zustand';
import type { ToolType, DesignNode, DocumentState } from '../types';

interface EditorState {
  // Core
  coreLoaded: boolean;
  wasmEnabled: boolean;

  // Tool
  tool: ToolType;
  previousTool: ToolType | null;
  setTool: (tool: ToolType) => void;
  setPreviousTool: (tool: ToolType | null) => void;

  // Space bar panning
  isSpacePressed: boolean;
  setSpacePressed: (pressed: boolean) => void;

  // Viewport
  zoom: number;
  panX: number;
  panY: number;
  setZoom: (zoom: number) => void;
  setPan: (x: number, y: number) => void;

  // Document
  document: DocumentState;
  selection: string[];
  selectNodes: (ids: string[]) => void;
  addToSelection: (id: string) => void;
  clearSelection: () => void;

  // Nodes
  getNode: (id: string) => DesignNode | undefined;
  addNode: (node: DesignNode) => void;
  updateNode: (id: string, updates: Partial<DesignNode>) => void;
  deleteNode: (id: string) => void;

  // Actions
  initCore: () => Promise<void>;
  undo: () => void;
  redo: () => void;
}

export const useEditorStore = create<EditorState>((set, get) => ({
  // Core
  coreLoaded: false,
  wasmEnabled: false,

  // Tool
  tool: 'select',
  previousTool: null,
  setTool: (tool) => set({ tool }),
  setPreviousTool: (tool) => set({ previousTool: tool }),

  // Space bar panning
  isSpacePressed: false,
  setSpacePressed: (pressed) => set({ isSpacePressed: pressed }),

  // Viewport
  zoom: 1,
  panX: 0,
  panY: 0,
  setZoom: (zoom) => set({ zoom: Math.max(0.1, Math.min(10, zoom)) }),
  setPan: (panX, panY) => set({ panX, panY }),

  // Document
  document: {
    nodes: new Map(),
    pages: ['page-1'],
    currentPageId: 'page-1',
    selection: [],
  },
  selection: [],
  selectNodes: (ids) => set({ selection: ids }),
  addToSelection: (id) => set((state) => ({
    selection: state.selection.includes(id) ? state.selection : [...state.selection, id],
  })),
  clearSelection: () => set({ selection: [] }),

  // Nodes
  getNode: (id) => get().document.nodes.get(id),
  addNode: (node) => set((state) => {
    const nodes = new Map(state.document.nodes);
    nodes.set(node.id, node);
    return {
      document: { ...state.document, nodes },
    };
  }),
  updateNode: (id, updates) => set((state) => {
    const nodes = new Map(state.document.nodes);
    const existing = nodes.get(id);
    if (existing) {
      nodes.set(id, { ...existing, ...updates });
    }
    return {
      document: { ...state.document, nodes },
    };
  }),
  deleteNode: (id) => set((state) => {
    const nodes = new Map(state.document.nodes);
    nodes.delete(id);
    const selection = state.selection.filter((s) => s !== id);
    return {
      document: { ...state.document, nodes },
      selection,
    };
  }),

  // Actions
  initCore: async () => {
    let wasmLoaded = false;
    
    try {
      // Try to load WASM module
      const core = await import('@anatsui/wasm');
      if (core && core.default) {
        await core.default();
        wasmLoaded = true;
        console.log('✅ WASM rendering engine loaded successfully');
      }
    } catch (error) {
      console.warn('⚠️ WASM core not available, using Canvas2D fallback:', error);
      wasmLoaded = false;
    }
      
    // Create initial page
    const pageNode: DesignNode = {
      id: 'page-1',
      type: 'page',
      name: 'Page 1',
      visible: true,
      locked: false,
      x: 0,
      y: 0,
      width: 0,
      height: 0,
      rotation: 0,
      opacity: 1,
      fills: [],
      strokes: [],
      children: [],
    };

    set((state) => {
      const nodes = new Map(state.document.nodes);
      nodes.set(pageNode.id, pageNode);
      return {
        coreLoaded: true,
        wasmEnabled: wasmLoaded,
        document: { ...state.document, nodes },
      };
    });
  },

  undo: () => {
    // TODO: Implement undo
  },
  redo: () => {
    // TODO: Implement redo
  },
}));
