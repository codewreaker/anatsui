import { useState } from 'react';
import { useEditorStore } from '../store/editorStore';
import ContextMenu from './ContextMenu';

function LayersPanel() {
  const { document, selection, selectNodes, updateNode } = useEditorStore();
  const [contextMenu, setContextMenu] = useState<{ x: number; y: number; nodeId: string } | null>(null);

  // Get nodes sorted by order (excluding page)
  const nodes = Array.from(document.nodes.values())
    .filter((n) => n.type !== 'page')
    .reverse();

  return (
    <div className="w-60 bg-figma-panel border-r border-figma-border flex flex-col">
      {/* Header */}
      <div className="h-10 border-b border-figma-border flex items-center px-3">
        <span className="text-xs font-medium text-figma-text-secondary">Layers</span>
      </div>

      {/* Page */}
      <div className="h-8 border-b border-figma-border flex items-center px-3 gap-2">
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor" className="text-figma-text-secondary">
          <path d="M2 3a1 1 0 011-1h10a1 1 0 011 1v10a1 1 0 01-1 1H3a1 1 0 01-1-1V3z" />
        </svg>
        <span className="text-xs">Page 1</span>
      </div>

      {/* Layers list */}
      <div className="flex-1 overflow-y-auto">
        {nodes.length === 0 ? (
          <div className="p-4 text-center text-xs text-figma-text-secondary">
            No layers yet
          </div>
        ) : (
          <ul className="py-1">
            {nodes.map((node) => (
              <li
                key={node.id}
                onClick={() => selectNodes([node.id])}
                onContextMenu={(e) => {
                  e.preventDefault();
                  selectNodes([node.id]);
                  setContextMenu({ x: e.clientX, y: e.clientY, nodeId: node.id });
                }}
                className={`
                  h-8 flex items-center px-3 gap-2 cursor-pointer
                  ${selection.includes(node.id) 
                    ? 'bg-figma-accent/20' 
                    : 'hover:bg-figma-border/30'}
                `}
              >
                {/* Visibility toggle */}
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    updateNode(node.id, { visible: !node.visible });
                  }}
                  className={`w-4 h-4 flex items-center justify-center ${
                    node.visible ? 'text-figma-text' : 'text-figma-text-secondary'
                  }`}
                >
                  {node.visible ? (
                    <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
                      <path d="M8 3C4.5 3 1.5 5.5 0 8c1.5 2.5 4.5 5 8 5s6.5-2.5 8-5c-1.5-2.5-4.5-5-8-5zm0 8a3 3 0 110-6 3 3 0 010 6z" />
                    </svg>
                  ) : (
                    <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
                      <path d="M14.5 1.5l-13 13m3-5.5a3 3 0 015-2.5m2 4c1-.5 2-1.5 2.5-2.5-1.5-2.5-4.5-5-8-5-.5 0-1 0-1.5.1" stroke="currentColor" strokeWidth="1.5" fill="none" />
                    </svg>
                  )}
                </button>

                {/* Icon */}
                <NodeIcon type={node.type} />

                {/* Name */}
                <span className="text-xs truncate flex-1">{node.name}</span>

                {/* Lock toggle */}
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    updateNode(node.id, { locked: !node.locked });
                  }}
                  className={`w-4 h-4 flex items-center justify-center ${
                    node.locked ? 'text-figma-text' : 'text-transparent hover:text-figma-text-secondary'
                  }`}
                >
                  <svg width="10" height="10" viewBox="0 0 16 16" fill="currentColor">
                    <path d="M12 7V5a4 4 0 00-8 0v2H3v7h10V7h-1zM6 5a2 2 0 014 0v2H6V5z" />
                  </svg>
                </button>
              </li>
            ))}
          </ul>
        )}
      </div>
      
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

function NodeIcon({ type }: { type: string }) {
  switch (type) {
    case 'frame':
      return (
        <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.5">
          <rect x="3" y="3" width="10" height="10" rx="1" />
        </svg>
      );
    case 'rectangle':
      return (
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
          <rect x="2" y="4" width="12" height="8" rx="1" />
        </svg>
      );
    case 'ellipse':
      return (
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
          <ellipse cx="8" cy="8" rx="6" ry="5" />
        </svg>
      );
    case 'line':
      return (
        <svg width="12" height="12" viewBox="0 0 16 16" stroke="currentColor" strokeWidth="2">
          <line x1="2" y1="14" x2="14" y2="2" />
        </svg>
      );
    case 'text':
      return (
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
          <path d="M2 3h12v2H9v9H7V5H2V3z" />
        </svg>
      );
    default:
      return (
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
          <rect x="2" y="2" width="12" height="12" rx="2" />
        </svg>
      );
  }
}

export default LayersPanel;
