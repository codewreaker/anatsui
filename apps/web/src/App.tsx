import { useState, useEffect } from 'react';
import Canvas from './components/Canvas';
import LayersPanel from './components/LayersPanel';
import PropertiesPanel from './components/PropertiesPanel';
import TopBar from './components/TopBar';
import FallbackWarning from './components/FallbackWarning';
import { useEditorStore } from './store/editorStore';
import type { ToolType } from './types';

function App() {
  const { initCore, wasmEnabled, setSpacePressed, setPreviousTool } = useEditorStore();
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Initialize WASM core
    initCore().then(() => {
      setLoading(false);
    });
  }, [initCore]);

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ignore if typing in an input
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
        return;
      }

      // Space bar for temporary pan mode
      if (e.code === 'Space' && !e.repeat) {
        e.preventDefault();
        setSpacePressed(true);
        const currentTool = useEditorStore.getState().tool;
        setPreviousTool(currentTool);
        return;
      }

      const toolMap: Record<string, ToolType> = {
        'v': 'select',
        'f': 'frame',
        'r': 'rectangle',
        'o': 'ellipse',
        'l': 'line',
        'p': 'pen',
        't': 'text',
        'h': 'hand',
      };

      const newTool = toolMap[e.key.toLowerCase()];
      if (newTool) {
        e.preventDefault();
        useEditorStore.getState().setTool(newTool);
      }
      
      // Escape key - could be used to cancel operations
      if (e.key === 'Escape') {
        e.preventDefault();
        useEditorStore.getState().setTool('select');
      }

      // Zoom shortcuts
      if ((e.metaKey || e.ctrlKey) && e.key === '=') {
        e.preventDefault();
        // Zoom in
      } else if ((e.metaKey || e.ctrlKey) && e.key === '-') {
        e.preventDefault();
        // Zoom out
      } else if ((e.metaKey || e.ctrlKey) && e.key === '0') {
        e.preventDefault();
        // Reset zoom
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      // Release space bar pan mode
      if (e.code === 'Space') {
        e.preventDefault();
        setSpacePressed(false);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
    };
  }, [setSpacePressed, setPreviousTool]);

  if (loading) {
    return (
      <div className="w-full h-full flex items-center justify-center bg-figma-panel">
        <div className="text-center">
          <div className="w-12 h-12 border-4 border-figma-accent border-t-transparent rounded-full animate-spin mx-auto mb-4"></div>
          <p className="text-figma-text-secondary">Loading Anatsui...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="w-full h-full flex flex-col">
      <TopBar />
      {!wasmEnabled && <FallbackWarning />}
      <div className="flex-1 flex overflow-hidden">
        <LayersPanel />
        <Canvas />
        <PropertiesPanel />
      </div>
    </div>
  );
}

export default App;
