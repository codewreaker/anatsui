import { useRef, useEffect, useCallback, useState } from 'react';
import { useEditorStore } from '../store/editorStore';
import type { ToolType, Point, DesignNode } from '../types';

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
    clearSelection,
  } = useEditorStore();

  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState<Point>({ x: 0, y: 0 });
  const [dragCurrent, setDragCurrent] = useState<Point>({ x: 0, y: 0 });

  // Convert screen coordinates to canvas coordinates
  const screenToCanvas = useCallback((screenX: number, screenY: number): Point => {
    return {
      x: (screenX - panX) / zoom,
      y: (screenY - panY) / zoom,
    };
  }, [panX, panY, zoom]);

  // Generate unique ID
  const generateId = () => `node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;

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

    if (tool === 'select') {
      // Hit testing would go here
      clearSelection();
    }
  }, [tool, screenToCanvas, clearSelection]);

  // Handle mouse move
  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    const rect = containerRef.current?.getBoundingClientRect();
    if (!rect) return;

    const screenX = e.clientX - rect.left;
    const screenY = e.clientY - rect.top;

    if (isDragging) {
      if (tool === 'hand') {
        // Pan the canvas
        setPan(panX + e.movementX, panY + e.movementY);
      } else {
        const canvasPoint = screenToCanvas(screenX, screenY);
        setDragCurrent(canvasPoint);
      }
    }
  }, [isDragging, tool, panX, panY, setPan, screenToCanvas]);

  // Handle mouse up
  const handleMouseUp = useCallback(() => {
    if (!isDragging) return;

    // Create shape based on tool
    if (['rectangle', 'ellipse', 'frame', 'line'].includes(tool)) {
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
  }, [isDragging, tool, dragStart, dragCurrent, document.nodes.size, addNode, selectNodes]);

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
    if (isDragging && ['rectangle', 'ellipse', 'frame'].includes(tool)) {
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

    ctx.restore();
  }, [document.nodes, selection, zoom, panX, panY, isDragging, dragStart, dragCurrent, tool]);

  // Get cursor style based on tool
  const getCursor = (): string => {
    switch (tool) {
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

  return (
    <div
      ref={containerRef}
      className="flex-1 bg-figma-bg overflow-hidden relative"
      style={{ cursor: getCursor() }}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      onMouseLeave={handleMouseUp}
    >
      <canvas ref={canvasRef} className="absolute inset-0" />
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
