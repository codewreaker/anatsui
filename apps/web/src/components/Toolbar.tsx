import type { ToolType } from '../types';

interface ToolbarProps {
  currentTool: ToolType;
  onToolChange: (tool: ToolType) => void;
}

interface ToolButton {
  tool: ToolType;
  icon: JSX.Element;
  label: string;
  shortcut: string;
}

function Toolbar({ currentTool, onToolChange }: ToolbarProps) {
  const tools: ToolButton[] = [
    {
      tool: 'select',
      label: 'Select',
      shortcut: 'V',
      icon: (
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M3 1l10 7-4.5 1L7 13.5 3 1z" />
        </svg>
      ),
    },
    {
      tool: 'frame',
      label: 'Frame',
      shortcut: 'F',
      icon: (
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.5">
          <rect x="3" y="3" width="10" height="10" rx="1" />
        </svg>
      ),
    },
    {
      tool: 'rectangle',
      label: 'Rectangle',
      shortcut: 'R',
      icon: (
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <rect x="2" y="4" width="12" height="8" rx="1" />
        </svg>
      ),
    },
    {
      tool: 'ellipse',
      label: 'Ellipse',
      shortcut: 'O',
      icon: (
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <ellipse cx="8" cy="8" rx="6" ry="5" />
        </svg>
      ),
    },
    {
      tool: 'line',
      label: 'Line',
      shortcut: 'L',
      icon: (
        <svg width="16" height="16" viewBox="0 0 16 16" stroke="currentColor" strokeWidth="2">
          <line x1="2" y1="14" x2="14" y2="2" />
        </svg>
      ),
    },
    {
      tool: 'pen',
      label: 'Pen',
      shortcut: 'P',
      icon: (
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M12.9 1.1a1.5 1.5 0 012.1 2.1l-8.5 8.5-2.8.7.7-2.8 8.5-8.5z" />
        </svg>
      ),
    },
    {
      tool: 'text',
      label: 'Text',
      shortcut: 'T',
      icon: (
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M2 3h12v2H9v9H7V5H2V3z" />
        </svg>
      ),
    },
    {
      tool: 'hand',
      label: 'Hand',
      shortcut: 'H',
      icon: (
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M8 2a1 1 0 011 1v5h2a1 1 0 011 1v4a3 3 0 01-3 3H6a3 3 0 01-3-3V7a1 1 0 011-1h1V3a1 1 0 011-1h2z" />
        </svg>
      ),
    },
  ];

  return (
    <div className="w-12 bg-figma-panel border-r border-figma-border flex flex-col items-center py-2 gap-1">
      {tools.map(({ tool, icon, label, shortcut }) => (
        <button
          key={tool}
          onClick={() => onToolChange(tool)}
          title={`${label} (${shortcut})`}
          className={`
            w-9 h-9 rounded flex items-center justify-center transition-colors
            ${currentTool === tool 
              ? 'bg-figma-accent text-white' 
              : 'text-figma-text-secondary hover:text-figma-text hover:bg-figma-border/50'}
          `}
        >
          {icon}
        </button>
      ))}
    </div>
  );
}

export default Toolbar;
