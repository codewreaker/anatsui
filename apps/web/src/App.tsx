import { useState, useEffect } from 'react';
import Toolbar from './components/Toolbar';
import Canvas from './components/Canvas';
import LayersPanel from './components/LayersPanel';
import PropertiesPanel from './components/PropertiesPanel';
import TopBar from './components/TopBar';
import { useEditorStore } from './store/editorStore';
import type { ToolType } from './types';

function App() {
  const { tool, setTool, initCore } = useEditorStore();
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
        setTool(newTool);
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

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [setTool]);

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
      <div className="flex-1 flex overflow-hidden">
        <Toolbar currentTool={tool} onToolChange={setTool} />
        <LayersPanel />
        <Canvas />
        <PropertiesPanel />
      </div>
    </div>
  );
}

export default App;
