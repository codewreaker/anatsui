interface WasmErrorScreenProps {
  error?: Error | string;
}

export default function WasmErrorScreen({ error }: WasmErrorScreenProps) {
  const errorMessage = error instanceof Error ? error.message : error || 'Unknown error';
  
  const handleReload = () => {
    window.location.reload();
  };

  return (
    <div className="fixed inset-0 bg-figma-bg flex items-center justify-center z-50">
      <div className="max-w-2xl mx-auto px-6">
        <div className="bg-figma-panel border border-red-500/30 rounded-xl shadow-2xl p-8">
          {/* Icon and Title */}
          <div className="flex items-center gap-4 mb-6">
            <div className="w-16 h-16 rounded-full bg-red-500/10 flex items-center justify-center flex-shrink-0">
              {/* Alert Triangle Icon */}
              <svg className="w-8 h-8 text-red-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
                <line x1="12" y1="9" x2="12" y2="13" />
                <line x1="12" y1="17" x2="12.01" y2="17" />
              </svg>
            </div>
            <div>
              <h1 className="text-2xl font-bold text-figma-text mb-1">
                Failed to Load Rendering Engine
              </h1>
              <p className="text-figma-text-secondary text-sm">
                Anatsui requires WebAssembly to run
              </p>
            </div>
          </div>

          {/* Error Details */}
          <div className="mb-6">
            <div className="bg-figma-bg border border-figma-border rounded-lg p-4">
              <p className="text-sm text-red-400 font-mono break-all">
                {errorMessage}
              </p>
            </div>
          </div>

          {/* Possible Causes */}
          <div className="mb-6">
            <h2 className="text-lg font-semibold text-figma-text mb-3">
              Possible Causes:
            </h2>
            <ul className="space-y-2 text-figma-text-secondary">
              <li className="flex items-start gap-2">
                <span className="text-red-500 mt-1">•</span>
                <span>WebAssembly is disabled in your browser settings</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-red-500 mt-1">•</span>
                <span>Your browser does not support WebAssembly (very unlikely in 2026)</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-red-500 mt-1">•</span>
                <span>The WASM module failed to compile or download</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-red-500 mt-1">•</span>
                <span>Network error prevented loading the rendering engine</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-red-500 mt-1">•</span>
                <span>Development build is incomplete (run <code className="text-figma-accent bg-figma-bg px-1 py-0.5 rounded">bun build:wasm</code>)</span>
              </li>
            </ul>
          </div>

          {/* Actions */}
          <div className="flex gap-3">
            <button
              onClick={handleReload}
              className="flex items-center gap-2 px-4 py-2 bg-figma-accent hover:bg-figma-accent-hover text-white rounded-lg transition-colors font-medium"
            >
              {/* Refresh Icon */}
              <svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <polyline points="23 4 23 10 17 10" />
                <polyline points="1 20 1 14 7 14" />
                <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
              </svg>
              Reload Page
            </button>
            <a
              href="https://developer.mozilla.org/en-US/docs/WebAssembly"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-2 px-4 py-2 bg-figma-panel border border-figma-border hover:bg-figma-hover text-figma-text rounded-lg transition-colors"
            >
              {/* External Link Icon */}
              <svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
                <polyline points="15 3 21 3 21 9" />
                <line x1="10" y1="14" x2="21" y2="3" />
              </svg>
              Learn About WebAssembly
            </a>
          </div>

          {/* Browser Check */}
          <div className="mt-6 pt-6 border-t border-figma-border">
            <p className="text-xs text-figma-text-secondary">
              Browser: <span className="text-figma-text">{navigator.userAgent}</span>
            </p>
            <p className="text-xs text-figma-text-secondary mt-1">
              WebAssembly Support: <span className={typeof WebAssembly !== 'undefined' ? 'text-green-400' : 'text-red-400'}>
                {typeof WebAssembly !== 'undefined' ? '✓ Available' : '✗ Not Available'}
              </span>
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
