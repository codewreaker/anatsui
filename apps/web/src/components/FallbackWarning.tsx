import { useState } from 'react';

interface FallbackWarningProps {
  onDismiss?: () => void;
}

function FallbackWarning({ onDismiss }: FallbackWarningProps) {
  const [expanded, setExpanded] = useState(false);
  const [dismissed, setDismissed] = useState(false);

  if (dismissed) return null;

  return (
    <div className="fixed top-14 left-1/2 -translate-x-1/2 z-50 max-w-lg">
      <div className="bg-yellow-900/90 border border-yellow-600 rounded-lg shadow-lg backdrop-blur-sm">
        <div className="flex items-center gap-3 px-4 py-2">
          <svg 
            className="w-5 h-5 text-yellow-400 flex-shrink-0" 
            fill="currentColor" 
            viewBox="0 0 20 20"
          >
            <path 
              fillRule="evenodd" 
              d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" 
              clipRule="evenodd" 
            />
          </svg>
          <span className="text-yellow-200 text-sm font-medium">
            Using Canvas2D Fallback
          </span>
          <span className="text-yellow-400/70 text-xs">
            (WebGL/WASM not loaded)
          </span>
          <button
            onClick={() => setExpanded(!expanded)}
            className="ml-2 text-yellow-400 hover:text-yellow-200 text-xs underline"
          >
            {expanded ? 'Hide' : 'How to fix'}
          </button>
          <button
            onClick={() => {
              setDismissed(true);
              onDismiss?.();
            }}
            className="ml-auto text-yellow-400 hover:text-yellow-200"
          >
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path 
                fillRule="evenodd" 
                d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" 
                clipRule="evenodd" 
              />
            </svg>
          </button>
        </div>

        {expanded && (
          <div className="px-4 pb-4 pt-2 border-t border-yellow-600/50">
            <p className="text-yellow-200/80 text-xs mb-3">
              For GPU-accelerated rendering, build the Rust/WASM core:
            </p>
            <div className="bg-black/30 rounded p-3 font-mono text-xs space-y-2">
              <p className="text-yellow-100/60"># 1. Install Rust (if not installed)</p>
              <p className="text-green-400">curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</p>
              
              <p className="text-yellow-100/60 mt-2"># 2. Install wasm-pack</p>
              <p className="text-green-400">cargo install wasm-pack</p>
              
              <p className="text-yellow-100/60 mt-2"># 3. Build the WASM core</p>
              <p className="text-green-400">bun build:wasm</p>
              
              <p className="text-yellow-100/60 mt-2"># 4. Restart the dev server</p>
              <p className="text-green-400">bun dev</p>
            </div>
            <p className="text-yellow-200/60 text-xs mt-3">
              Note: Requires Rust 1.88+ and wasm-pack installed globally.
            </p>
          </div>
        )}
      </div>
    </div>
  );
}

export default FallbackWarning;
