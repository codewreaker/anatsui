import { useEffect, useRef } from 'react';
import { useEditorStore } from '../store/editorStore';

interface ContextMenuProps {
  x: number;
  y: number;
  nodeId: string;
  onClose: () => void;
}

function ContextMenu({ x, y, nodeId, onClose }: ContextMenuProps) {
  const menuRef = useRef<HTMLDivElement>(null);
  const { getNode, updateNode, deleteNode, selectNodes, addNode, document: editorDocument } = useEditorStore();
  
  const node = getNode(nodeId);

  // Close on click outside
  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        onClose();
      }
    };

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };

    window.document.addEventListener('mousedown', handleClickOutside);
    window.document.addEventListener('keydown', handleKeyDown);
    return () => {
      window.document.removeEventListener('mousedown', handleClickOutside);
      window.document.removeEventListener('keydown', handleKeyDown);
    };
  }, [onClose]);

  if (!node) return null;

  const menuItems = [
    {
      label: 'Cut',
      shortcut: '⌘X',
      onClick: () => {
        // TODO: Implement cut
        onClose();
      },
    },
    {
      label: 'Copy',
      shortcut: '⌘C',
      onClick: () => {
        // TODO: Implement copy
        onClose();
      },
    },
    {
      label: 'Paste',
      shortcut: '⌘V',
      onClick: () => {
        // TODO: Implement paste
        onClose();
      },
    },
    { type: 'divider' as const },
    {
      label: 'Duplicate',
      shortcut: '⌘D',
      onClick: () => {
        const newNode = {
          ...node,
          id: `node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
          name: `${node.name} copy`,
          x: node.x + 20,
          y: node.y + 20,
        };
        addNode(newNode);
        selectNodes([newNode.id]);
        onClose();
      },
    },
    {
      label: 'Delete',
      shortcut: '⌫',
      onClick: () => {
        deleteNode(nodeId);
        onClose();
      },
      danger: true,
    },
    { type: 'divider' as const },
    {
      label: node.visible ? 'Hide' : 'Show',
      shortcut: '⌘⇧H',
      onClick: () => {
        updateNode(nodeId, { visible: !node.visible });
        onClose();
      },
    },
    {
      label: node.locked ? 'Unlock' : 'Lock',
      shortcut: '⌘⇧L',
      onClick: () => {
        updateNode(nodeId, { locked: !node.locked });
        onClose();
      },
    },
    { type: 'divider' as const },
    {
      label: 'Bring to Front',
      shortcut: '⌘]',
      onClick: () => {
        // TODO: Implement z-index
        onClose();
      },
    },
    {
      label: 'Send to Back',
      shortcut: '⌘[',
      onClick: () => {
        // TODO: Implement z-index
        onClose();
      },
    },
    { type: 'divider' as const },
    {
      label: 'Rename',
      onClick: () => {
        const newName = prompt('Rename', node.name);
        if (newName) {
          updateNode(nodeId, { name: newName });
        }
        onClose();
      },
    },
  ];

  return (
    <div
      ref={menuRef}
      className="fixed z-50 bg-figma-panel border border-figma-border rounded-lg shadow-xl py-1 min-w-[180px]"
      style={{ left: x, top: y }}
    >
      {menuItems.map((item, index) => {
        if ('type' in item && item.type === 'divider') {
          return <div key={index} className="h-px bg-figma-border my-1" />;
        }

        const menuItem = item as {
          label: string;
          shortcut?: string;
          onClick: () => void;
          danger?: boolean;
        };

        return (
          <button
            key={index}
            onClick={menuItem.onClick}
            className={`
              w-full px-3 py-1.5 text-left text-sm flex items-center justify-between
              ${menuItem.danger 
                ? 'text-red-400 hover:bg-red-500/20' 
                : 'text-figma-text hover:bg-figma-border/50'}
            `}
          >
            <span>{menuItem.label}</span>
            {menuItem.shortcut && (
              <span className="text-figma-text-secondary text-xs ml-4">
                {menuItem.shortcut}
              </span>
            )}
          </button>
        );
      })}
    </div>
  );
}

export default ContextMenu;
