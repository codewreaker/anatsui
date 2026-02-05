import { useRef, useEffect, useCallback, useState } from 'react';
import { useEditorStore } from '../store/editorStore';
import ContextMenu from './ContextMenu';
import type { Point, DesignNode, ToolType } from '../types';

interface PenPoint {
  x: number;
  y: number;
  handleIn: Point | null;
  handleOut: Point | null;
}

function Canvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const { 
    tool, 
    zoom, 
    panX, 
    panY, 
    setPan,
    setZoom,
    document,
    selection,
    addNode,
    selectNodes,
    addToSelection,
    clearSelection,
    updateNode,
    isSpacePressed,
  } = useEditorStore();

  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState<Point>({ x: 0, y: 0 });
  const [dragCurrent, setDragCurrent] = useState<Point>({ x: 0, y: 0 });
  const [dragMode, setDragMode] = useState<'create' | 'move' | 'pan' | null>(null);
  const [movingNodeId, setMovingNodeId] = useState<string | null>(null);
  const [moveStartPos, setMoveStartPos] = useState<Point | null>(null);
  const [currentMousePos, setCurrentMousePos] = useState<Point>({ x: 0, y: 0 });

  // Context menu state
  const [contextMenu, setContextMenu] = useState<{ x: number; y: number; nodeId: string } | null>(null);

  // Pen tool state
  const [penPoints, setPenPoints] = useState<PenPoint[]>([]);
  const [, setIsPenActive] = useState(false);
  const [isDraggingHandle, setIsDraggingHandle] = useState(false);
  const [tempHandlePos, setTempHandlePos] = useState<Point | null>(null);

  // Convert screen coordinates to canvas coordinates
  const screenToCanvas = useCallback((screenX: number, screenY: number): Point => {
    return {
      x: (screenX - panX) / zoom,
      y: (screenY - panY) / zoom,
    };
  }, [panX, panY, zoom]);

  // Generate unique ID
  const generateId = () => `node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;

  // Hit test - find node at position
  const hitTest = useCallback((canvasX: number, canvasY: number): string | null => {
    // Check nodes in reverse order (top to bottom)
    const nodes = Array.from(document.nodes.values()).filter(n => n.type !== 'page' && n.visible && !n.locked);
    
    for (let i = nodes.length - 1; i >= 0; i--) {
      const node = nodes[i];
      if (
        canvasX >= node.x &&
        canvasX <= node.x + node.width &&
        canvasY >= node.y &&
        canvasY <= node.y + node.height
      ) {
        return node.id;
      }
    }
    return null;
  }, [document.nodes]);

  // Handle mouse down
  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    const rect = containerRef.current?.getBoundingClientRect();
    if (!rect) return;

    const screenX = e.clientX - rect.left;
    const screenY = e.clientY - rect.top;
    const canvasPoint = screenToCanvas(screenX, screenY);

    setIsDragging(true);
    setDragStart(canvasPoint);
    setDragCurrent(canvasPoint);

    // Space bar panning
    if (isSpacePressed || tool === 'hand') {
      setDragMode('pan');
      return;
    }

    if (tool === 'select') {
      // Hit test for selection
      const hitNodeId = hitTest(canvasPoint.x, canvasPoint.y);
      
      if (hitNodeId) {
        if (e.shiftKey) {
          // Add to selection
          addToSelection(hitNodeId);
        } else if (!selection.includes(hitNodeId)) {
          // Select this node
          selectNodes([hitNodeId]);
        }
        // Start moving
        setDragMode('move');
        setMovingNodeId(hitNodeId);
        const node = document.nodes.get(hitNodeId);
        if (node) {
          setMoveStartPos({ x: node.x, y: node.y });
        }
      } else {
        // Clear selection if clicking empty space
        if (!e.shiftKey) {
          clearSelection();
        }
        setDragMode('create'); // Marquee selection would go here
      }
    } else if (tool === 'pen') {
      // Check if clicking near first point to close path
      if (penPoints.length >= 3) {
        const firstPoint = penPoints[0];
        const distToFirst = Math.sqrt(
          Math.pow(canvasPoint.x - firstPoint.x, 2) + 
          Math.pow(canvasPoint.y - firstPoint.y, 2)
        );
        
        // Close path if within 10 pixels of first point
        if (distToFirst < 10 / zoom) {
          finalizePenPath();
          return;
        }
      }
      
      // Add point and prepare to drag handle
      const newPoint: PenPoint = {
        x: canvasPoint.x,
        y: canvasPoint.y,
        handleIn: null,
        handleOut: null,
      };
      setPenPoints(prev => [...prev, newPoint]);
      setIsPenActive(true);
      setIsDraggingHandle(true);
      setTempHandlePos(canvasPoint);
      setDragMode(null);
      return;
    } else {
      setDragMode('create');
    }
  }, [tool, screenToCanvas, isSpacePressed, hitTest, selection, addToSelection, selectNodes, clearSelection, document.nodes, penPoints, zoom]);

  // Handle mouse move
  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    const rect = containerRef.current?.getBoundingClientRect();
    if (!rect) return;

    const screenX = e.clientX - rect.left;
    const screenY = e.clientY - rect.top;
    const canvasPoint = screenToCanvas(screenX, screenY);
    
    // Always update current mouse position for pen tool preview
    setCurrentMousePos(canvasPoint);
    
    // Handle pen tool handle dragging
    if (tool === 'pen' && isDraggingHandle) {
      setTempHandlePos(canvasPoint);
      return;
    }

    if (isDragging) {
      if (dragMode === 'pan' || tool === 'hand' || isSpacePressed) {
        // Pan the canvas
        setPan(panX + e.movementX, panY + e.movementY);
      } else if (dragMode === 'move' && movingNodeId) {
        // Move selected nodes using movement delta
        selection.forEach(nodeId => {
          const node = document.nodes.get(nodeId);
          if (node && !node.locked) {
            updateNode(nodeId, {
              x: node.x + e.movementX / zoom,
              y: node.y + e.movementY / zoom,
            });
          }
        });
      } else {
        const canvasPoint = screenToCanvas(screenX, screenY);
        setDragCurrent(canvasPoint);
      }
    }
  }, [isDragging, dragMode, tool, isSpacePressed, panX, panY, setPan, screenToCanvas, movingNodeId, selection, document.nodes, updateNode, zoom]);

  // Handle mouse up
  const handleMouseUp = useCallback(() => {
    // Finalize pen tool handle
    if (tool === 'pen' && isDraggingHandle && tempHandlePos) {
      setPenPoints(prev => {
        if (prev.length === 0) return prev;
        const newPoints = [...prev];
        const lastPoint = newPoints[newPoints.length - 1];
        
        // Calculate handle offset from point
        const handleOut = {
          x: tempHandlePos.x - lastPoint.x,
          y: tempHandlePos.y - lastPoint.y,
        };
        
        // Only add handle if dragged significantly
        const handleLength = Math.sqrt(handleOut.x * handleOut.x + handleOut.y * handleOut.y);
        if (handleLength > 5 / zoom) {
          newPoints[newPoints.length - 1] = {
            ...lastPoint,
            handleOut,
            handleIn: { x: -handleOut.x, y: -handleOut.y },
          };
        }
        
        return newPoints;
      });
      
      setIsDraggingHandle(false);
      setTempHandlePos(null);
      return;
    }
    
    if (!isDragging) return;

    // Create shape based on tool
    if (dragMode === 'create' && ['rectangle', 'ellipse', 'frame', 'line'].includes(tool)) {
      const x = Math.min(dragStart.x, dragCurrent.x);
      const y = Math.min(dragStart.y, dragCurrent.y);
      const width = Math.abs(dragCurrent.x - dragStart.x);
      const height = Math.abs(dragCurrent.y - dragStart.y);

      if (width > 2 && height > 2) {
        const node: DesignNode = {
          id: generateId(),
          type: tool === 'frame' ? 'frame' : tool as 'rectangle' | 'ellipse' | 'line',
          name: `${tool.charAt(0).toUpperCase() + tool.slice(1)} ${document.nodes.size + 1}`,
          visible: true,
          locked: false,
          x,
          y,
          width,
          height,
          rotation: 0,
          opacity: 1,
          fills: tool !== 'line' ? [{
            type: 'solid',
            color: { r: 200, g: 200, b: 200, a: 1 },
            visible: true,
            opacity: 1,
          }] : [],
          strokes: [{
            color: { r: 0, g: 0, b: 0, a: 1 },
            width: 1,
            visible: true,
            opacity: 1,
          }],
          cornerRadius: 0,
          parentId: 'page-1',
        };

        addNode(node);
        selectNodes([node.id]);
      }
    }

    setIsDragging(false);
    setDragMode(null);
    setMovingNodeId(null);
  }, [isDragging, dragMode, tool, dragStart, dragCurrent, document.nodes.size, addNode, selectNodes, zoom, tempHandlePos, isDraggingHandle, penPoints]);

  // Finalize pen path and create node
  const finalizePenPath = useCallback(() => {
    if (penPoints.length < 2) return;
    
    const minX = Math.min(...penPoints.map(p => p.x));
    const minY = Math.min(...penPoints.map(p => p.y));
    const maxX = Math.max(...penPoints.map(p => p.x));
    const maxY = Math.max(...penPoints.map(p => p.y));

    const node: DesignNode = {
      id: generateId(),
      type: 'vector',
      name: `Vector ${document.nodes.size + 1}`,
      visible: true,
      locked: false,
      x: minX,
      y: minY,
      width: maxX - minX || 1,
      height: maxY - minY || 1,
      rotation: 0,
      opacity: 1,
      fills: [],
      strokes: [{
        color: { r: 0, g: 0, b: 0, a: 1 },
        width: 2,
        visible: true,
        opacity: 1,
      }],
      cornerRadius: 0,
      parentId: 'page-1',
    };

    addNode(node);
    selectNodes([node.id]);
    setPenPoints([]);
    setIsPenActive(false);
    setIsDraggingHandle(false);
    setTempHandlePos(null);
  }, [penPoints, document.nodes.size, addNode, selectNodes]);

  // Handle right click (context menu)
  const handleContextMenu = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    
    const rect = containerRef.current?.getBoundingClientRect();
    if (!rect) return;

    const screenX = e.clientX - rect.left;
    const screenY = e.clientY - rect.top;
    const canvasPoint = screenToCanvas(screenX, screenY);

    const hitNodeId = hitTest(canvasPoint.x, canvasPoint.y);
    
    if (hitNodeId) {
      if (!selection.includes(hitNodeId)) {
        selectNodes([hitNodeId]);
      }
      setContextMenu({ x: e.clientX, y: e.clientY, nodeId: hitNodeId });
    } else {
      setContextMenu(null);
    }
  }, [screenToCanvas, hitTest, selection, selectNodes]);

  // Handle double click to finish pen path
  const handleDoubleClick = useCallback(() => {
    if (tool === 'pen' && penPoints.length >= 2) {
      finalizePenPath();
    }
  }, [tool, penPoints.length, finalizePenPath]);

  // Handle escape key to cancel pen path
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && tool === 'pen' && penPoints.length > 0) {
        setPenPoints([]);
        setIsPenActive(false);
        setIsDraggingHandle(false);
        setTempHandlePos(null);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [tool, penPoints.length]);

  // Handle wheel (zoom/pan)
  const handleWheel = useCallback((e: WheelEvent) => {
    e.preventDefault();

    if (e.ctrlKey || e.metaKey) {
      // Zoom
      const delta = e.deltaY > 0 ? 0.9 : 1.1;
      setZoom(zoom * delta);
    } else {
      // Pan
      setPan(panX - e.deltaX, panY - e.deltaY);
    }
  }, [zoom, panX, panY, setZoom, setPan]);

  // Add wheel listener
  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    container.addEventListener('wheel', handleWheel, { passive: false });
    return () => container.removeEventListener('wheel', handleWheel);
  }, [handleWheel]);

  // TODO: Integrate WASM WebGL2 renderer here
  // This is a placeholder - the actual rendering will be done by the Rust WASM engine
  // For now, just setup the canvas
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    // Resize canvas to container
    const container = containerRef.current;
    if (container) {
      canvas.width = container.clientWidth * window.devicePixelRatio;
      canvas.height = container.clientHeight * window.devicePixelRatio;
      canvas.style.width = `${container.clientWidth}px`;
      canvas.style.height = `${container.clientHeight}px`;
    }

    // TODO: Initialize WebGL2 context and pass to Rust renderer
    // const gl = canvas.getContext('webgl2');
    // if (gl) {
    //   // Pass GL context to WASM renderer
    //   // wasmRenderer.init(gl);
    //   // wasmRenderer.render(document, zoom, panX, panY);
    // }
  }, [document.nodes, selection, zoom, panX, panY, isDragging, dragMode, dragStart, dragCurrent, tool, penPoints, currentMousePos, isDraggingHandle, tempHandlePos]);

  // Get cursor style based on tool and state
  const getCursor = (): string => {
    // Space bar pan mode
    if (isSpacePressed) {
      return isDragging ? 'grabbing' : 'grab';
    }
    
    switch (tool) {
      case 'select':
        return isDragging && dragMode === 'move' ? 'move' : 'default';
      case 'hand':
        return isDragging ? 'grabbing' : 'grab';
      case 'text':
        return 'text';
      case 'pen':
        return 'crosshair';
      default:
        return 'crosshair';
    }
  };

  // Handle touch events for two-finger pan
  const handleTouchStart = useCallback((e: React.TouchEvent) => {
    if (e.touches.length === 2) {
      // Two-finger pan - store initial touch positions
      const touch1 = e.touches[0];
      const touch2 = e.touches[1];
      const centerX = (touch1.clientX + touch2.clientX) / 2;
      const centerY = (touch1.clientY + touch2.clientY) / 2;
      setMoveStartPos({ x: centerX, y: centerY });
      setDragMode('pan');
      setIsDragging(true);
    }
  }, []);

  const handleTouchMove = useCallback((e: React.TouchEvent) => {
    if (e.touches.length === 2 && isDragging && dragMode === 'pan' && moveStartPos) {
      const touch1 = e.touches[0];
      const touch2 = e.touches[1];
      const centerX = (touch1.clientX + touch2.clientX) / 2;
      const centerY = (touch1.clientY + touch2.clientY) / 2;
      
      const deltaX = centerX - moveStartPos.x;
      const deltaY = centerY - moveStartPos.y;
      
      setPan(panX + deltaX, panY + deltaY);
      setMoveStartPos({ x: centerX, y: centerY });
    }
  }, [isDragging, dragMode, moveStartPos, panX, panY, setPan]);

  const handleTouchEnd = useCallback(() => {
    if (dragMode === 'pan') {
      setIsDragging(false);
      setDragMode(null);
      setMoveStartPos(null);
    }
  }, [dragMode]);

  return (
    <div
      ref={containerRef}
      className="flex-1 bg-figma-bg overflow-hidden relative"
      style={{ cursor: getCursor() }}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      onMouseLeave={handleMouseUp}
      onContextMenu={handleContextMenu}
      onDoubleClick={handleDoubleClick}
      onTouchStart={handleTouchStart}
      onTouchMove={handleTouchMove}
      onTouchEnd={handleTouchEnd}
    >
      <canvas ref={canvasRef} className="absolute inset-0" />
      
      {/* Context Menu */}
      {contextMenu && (
        <ContextMenu
          x={contextMenu.x}
          y={contextMenu.y}
          nodeId={contextMenu.nodeId}
          onClose={() => setContextMenu(null)}
        />
      )}
      
      {/* Toolbar positioned at bottom center of canvas */}
      <Toolbar />
    </div>
  );
}

function Toolbar() {
  const { tool, setTool } = useEditorStore();
  
  const tools = [
    { tool: 'select', label: 'Select', shortcut: 'V', icon: <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M3 1l10 7-4.5 1L7 13.5 3 1z" /></svg> },
    { tool: 'frame', label: 'Frame', shortcut: 'F', icon: <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.5"><rect x="3" y="3" width="10" height="10" rx="1" /></svg> },
    { tool: 'rectangle', label: 'Rectangle', shortcut: 'R', icon: <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="2" y="4" width="12" height="8" rx="1" /></svg> },
    { tool: 'ellipse', label: 'Ellipse', shortcut: 'O', icon: <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><ellipse cx="8" cy="8" rx="6" ry="5" /></svg> },
    { tool: 'line', label: 'Line', shortcut: 'L', icon: <svg width="16" height="16" viewBox="0 0 16 16" stroke="currentColor" strokeWidth="2"><line x1="2" y1="14" x2="14" y2="2" /></svg> },
    { tool: 'pen', label: 'Pen', shortcut: 'P', icon: <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M12.9 1.1a1.5 1.5 0 012.1 2.1l-8.5 8.5-2.8.7.7-2.8 8.5-8.5z" /></svg> },
    { tool: 'text', label: 'Text', shortcut: 'T', icon: <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M4 2h8v2H9v10H7V4H4V2z" /></svg> },
    { tool: 'hand', label: 'Hand', shortcut: 'H', icon: <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M8 1a1 1 0 011 1v5h1a1 1 0 011 1v3a4 4 0 01-8 0V4a1 1 0 011-1h1V2a1 1 0 011-1z" /></svg> },
  ] as const;
  
  return (
    <div className="absolute bottom-4 left-1/2 -translate-x-1/2 bg-figma-panel border border-figma-border rounded-lg shadow-lg flex items-center px-2 py-2 gap-0.5 z-10">
      {tools.map((t) => (
        <button
          key={t.tool}
          onClick={() => setTool(t.tool as any)}
          className={`w-8 h-8 rounded flex items-center justify-center transition-colors group relative ${
            tool === t.tool ? 'bg-figma-accent text-white' : 'text-figma-text-secondary hover:bg-figma-hover hover:text-figma-text'
          }`}
          title={`${t.label} (${t.shortcut})`}
        >
          {t.icon}
          <div className="absolute bottom-full mb-2 px-2 py-1 bg-figma-tooltip text-white text-xs rounded whitespace-nowrap opacity-0 group-hover:opacity-100 pointer-events-none transition-opacity z-50">
            {t.label} <span className="ml-2 text-figma-text-secondary">{t.shortcut}</span>
          </div>
        </button>
      ))}
    </div>
  );
}

export default Canvas;
