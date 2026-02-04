import { useEditorStore } from '../store/editorStore';

function TopBar() {
  const { zoom, setZoom } = useEditorStore();

  const zoomOptions = [0.25, 0.5, 0.75, 1, 1.25, 1.5, 2, 4];

  return (
    <header className="h-12 bg-figma-panel border-b border-figma-border flex items-center justify-between px-4">
      {/* Left: Logo and file name */}
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2">
          <svg width="24" height="24" viewBox="0 0 32 32" fill="none">
            <rect width="32" height="32" rx="6" fill="#1E1E1E"/>
            <path d="M8 8h16v16H8z" fill="url(#topbar-grad)"/>
            <defs>
              <linearGradient id="topbar-grad" x1="8" y1="8" x2="24" y2="24">
                <stop offset="0%" stopColor="#0D99FF"/>
                <stop offset="100%" stopColor="#A259FF"/>
              </linearGradient>
            </defs>
          </svg>
          <span className="text-sm font-medium">Anatsui</span>
        </div>
        <div className="h-4 w-px bg-figma-border" />
        <span className="text-sm text-figma-text-secondary">Untitled</span>
      </div>

      {/* Center: Nothing for now */}
      <div className="flex-1" />

      {/* Right: Zoom and user */}
      <div className="flex items-center gap-4">
        {/* Zoom dropdown */}
        <select
          value={zoom}
          onChange={(e) => setZoom(parseFloat(e.target.value))}
          className="bg-transparent text-sm text-figma-text border border-figma-border rounded px-2 py-1 focus:outline-none focus:border-figma-accent"
        >
          {zoomOptions.map((z) => (
            <option key={z} value={z} className="bg-figma-panel">
              {Math.round(z * 100)}%
            </option>
          ))}
        </select>

        {/* Share button */}
        <button className="bg-figma-accent text-white text-sm font-medium px-4 py-1.5 rounded hover:bg-blue-500 transition-colors">
          Share
        </button>

        {/* User avatar */}
        <div className="w-8 h-8 rounded-full bg-gradient-to-br from-purple-500 to-pink-500 flex items-center justify-center text-sm font-medium">
          U
        </div>
      </div>
    </header>
  );
}

export default TopBar;
