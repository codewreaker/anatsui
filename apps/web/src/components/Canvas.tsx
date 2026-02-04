import { useRef, useEffect, useCallback, useState } from 'react';
import { useEditorStore } from '../store/editorStore';
import ContextMenu from './ContextMenu';
import type { Point, DesignNode } from '../types';

interface PenPoint {
  x: number;
  y: number;
  handleIn?: Point;
  handleOut?: Point;
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
      // Pen tool - add point
      const newPoint: PenPoint = { x: canvasPoint.x, y: canvasPoint.y };
      setPenPoints(prev => [...prev, newPoint]);
      setIsPenActive(true);
      setDragMode(null);
      setIsDragging(false);
    } else {
      setDragMode('create');
    }
  }, [tool, screenToCanvas, isSpacePressed, hitTest, selection, addToSelection, selectNodes, clearSelection, document.nodes]);

  // Handle mouse move
  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    const rect = containerRef.current?.getBoundingClientRect();
    if (!rect) return;

    const screenX = e.clientX - rect.left;
    const screenY = e.clientY - rect.top;
    const canvasPoint = screenToCanvas(screenX, screenY);
    
    // Always update current mouse position for pen tool preview
    setCurrentMousePos(canvasPoint);

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
  }, [isDragging, dragMode, tool, dragStart, dragCurrent, document.nodes.size, addNode, selectNodes]);

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
      // Create vector path from pen points
      // For now, just create a simple line representation
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
    }
  }, [tool, penPoints, document.nodes.size, addNode, selectNodes]);

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

  // Render canvas
  useEffect(() => {
    const canvas = canvasRef.current;
    const ctx = canvas?.getContext('2d');
    if (!canvas || !ctx) return;

    // Resize canvas
    const container = containerRef.current;
    if (container) {
      canvas.width = container.clientWidth * window.devicePixelRatio;
      canvas.height = container.clientHeight * window.devicePixelRatio;
      canvas.style.width = `${container.clientWidth}px`;
      canvas.style.height = `${container.clientHeight}px`;
      ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    }

    // Clear
    ctx.fillStyle = '#2c2c2c';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Draw grid
    ctx.save();
    ctx.translate(panX, panY);
    ctx.scale(zoom, zoom);

    // Grid pattern
    const gridSize = 100;
    ctx.strokeStyle = '#383838';
    ctx.lineWidth = 1 / zoom;
    
    const startX = Math.floor(-panX / zoom / gridSize) * gridSize;
    const startY = Math.floor(-panY / zoom / gridSize) * gridSize;
    const endX = startX + (canvas.width / zoom) + gridSize * 2;
    const endY = startY + (canvas.height / zoom) + gridSize * 2;

    for (let x = startX; x < endX; x += gridSize) {
      ctx.beginPath();
      ctx.moveTo(x, startY);
      ctx.lineTo(x, endY);
      ctx.stroke();
    }

    for (let y = startY; y < endY; y += gridSize) {
      ctx.beginPath();
      ctx.moveTo(startX, y);
      ctx.lineTo(endX, y);
      ctx.stroke();
    }

    // Draw nodes
    document.nodes.forEach((node) => {
      if (node.type === 'page') return;
      if (!node.visible) return;

      ctx.save();
      ctx.translate(node.x, node.y);
      ctx.rotate((node.rotation * Math.PI) / 180);
      ctx.globalAlpha = node.opacity;

      // Draw fill
      if (node.fills.length > 0) {
        const fill = node.fills[0];
        if (fill.visible && fill.color) {
          ctx.fillStyle = `rgba(${fill.color.r}, ${fill.color.g}, ${fill.color.b}, ${fill.color.a * fill.opacity})`;
          
          if (node.type === 'rectangle' || node.type === 'frame') {
            if (node.cornerRadius) {
              roundRect(ctx, 0, 0, node.width, node.height, node.cornerRadius);
              ctx.fill();
            } else {
              ctx.fillRect(0, 0, node.width, node.height);
            }
          } else if (node.type === 'ellipse') {
            ctx.beginPath();
            ctx.ellipse(node.width / 2, node.height / 2, node.width / 2, node.height / 2, 0, 0, Math.PI * 2);
            ctx.fill();
          }
        }
      }

      // Draw stroke
      if (node.strokes.length > 0) {
        const stroke = node.strokes[0];
        if (stroke.visible) {
          ctx.strokeStyle = `rgba(${stroke.color.r}, ${stroke.color.g}, ${stroke.color.b}, ${stroke.color.a * stroke.opacity})`;
          ctx.lineWidth = stroke.width;

          if (node.type === 'rectangle' || node.type === 'frame') {
            if (node.cornerRadius) {
              roundRect(ctx, 0, 0, node.width, node.height, node.cornerRadius);
              ctx.stroke();
            } else {
              ctx.strokeRect(0, 0, node.width, node.height);
            }
          } else if (node.type === 'ellipse') {
            ctx.beginPath();
            ctx.ellipse(node.width / 2, node.height / 2, node.width / 2, node.height / 2, 0, 0, Math.PI * 2);
            ctx.stroke();
          } else if (node.type === 'line') {
            ctx.beginPath();
            ctx.moveTo(0, 0);
            ctx.lineTo(node.width, node.height);
            ctx.stroke();
          }
        }
      }

      ctx.restore();

      // Draw selection
      if (selection.includes(node.id)) {
        ctx.strokeStyle = '#0d99ff';
        ctx.lineWidth = 2 / zoom;
        ctx.strokeRect(node.x - 1, node.y - 1, node.width + 2, node.height + 2);

        // Selection handles
        const handleSize = 8 / zoom;
        ctx.fillStyle = '#ffffff';
        const handles = [
          { x: node.x - handleSize / 2, y: node.y - handleSize / 2 },
          { x: node.x + node.width - handleSize / 2, y: node.y - handleSize / 2 },
          { x: node.x + node.width - handleSize / 2, y: node.y + node.height - handleSize / 2 },
          { x: node.x - handleSize / 2, y: node.y + node.height - handleSize / 2 },
        ];
        handles.forEach((h) => {
          ctx.fillRect(h.x, h.y, handleSize, handleSize);
          ctx.strokeRect(h.x, h.y, handleSize, handleSize);
        });
      }
    });

    // Draw drag preview
    if (isDragging && dragMode === 'create' && ['rectangle', 'ellipse', 'frame'].includes(tool)) {
      const x = Math.min(dragStart.x, dragCurrent.x);
      const y = Math.min(dragStart.y, dragCurrent.y);
      const width = Math.abs(dragCurrent.x - dragStart.x);
      const height = Math.abs(dragCurrent.y - dragStart.y);

      ctx.strokeStyle = '#0d99ff';
      ctx.lineWidth = 1 / zoom;
      ctx.setLineDash([5 / zoom, 5 / zoom]);

      if (tool === 'ellipse') {
        ctx.beginPath();
        ctx.ellipse(x + width / 2, y + height / 2, width / 2, height / 2, 0, 0, Math.PI * 2);
        ctx.stroke();
      } else {
        ctx.strokeRect(x, y, width, height);
      }

      ctx.setLineDash([]);
    }

    // Draw pen tool path preview
    if (tool === 'pen' && penPoints.length > 0) {
      ctx.strokeStyle = '#0d99ff';
      ctx.lineWidth = 2 / zoom;
      ctx.fillStyle = '#ffffff';
      
      // Draw lines between points
      ctx.beginPath();
      ctx.moveTo(penPoints[0].x, penPoints[0].y);
      for (let i = 1; i < penPoints.length; i++) {
        ctx.lineTo(penPoints[i].x, penPoints[i].y);
      }
      
      // Draw preview line to current mouse position
      if (penPoints.length >= 1) {
        ctx.setLineDash([5 / zoom, 5 / zoom]);
        ctx.lineTo(currentMousePos.x, currentMousePos.y);
      }
      
      ctx.stroke();
      ctx.setLineDash([]);

      // Draw points
      const pointRadius = 4 / zoom;
      penPoints.forEach((point, index) => {
        ctx.beginPath();
        ctx.arc(point.x, point.y, pointRadius, 0, Math.PI * 2);
        ctx.fill();
        ctx.strokeStyle = '#0d99ff';
        ctx.stroke();
        
        // Highlight first point (for closing path)
        if (index === 0 && penPoints.length > 2) {
          ctx.beginPath();
          ctx.arc(point.x, point.y, pointRadius * 1.5, 0, Math.PI * 2);
          ctx.strokeStyle = '#ff6b00';
          ctx.stroke();
          ctx.strokeStyle = '#0d99ff';
        }
      });
    }

    ctx.restore();
  }, [document.nodes, selection, zoom, panX, panY, isDragging, dragMode, dragStart, dragCurrent, tool, penPoints, currentMousePos]);

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
    </div>
  );
}

// Helper function for rounded rectangles
function roundRect(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  width: number,
  height: number,
  radius: number
) {
  ctx.beginPath();
  ctx.moveTo(x + radius, y);
  ctx.lineTo(x + width - radius, y);
  ctx.quadraticCurveTo(x + width, y, x + width, y + radius);
  ctx.lineTo(x + width, y + height - radius);
  ctx.quadraticCurveTo(x + width, y + height, x + width - radius, y + height);
  ctx.lineTo(x + radius, y + height);
  ctx.quadraticCurveTo(x, y + height, x, y + height - radius);
  ctx.lineTo(x, y + radius);
  ctx.quadraticCurveTo(x, y, x + radius, y);
  ctx.closePath();
}

export default Canvas;
