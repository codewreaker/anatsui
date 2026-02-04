import { useEditorStore } from '../store/editorStore';
import type { Color } from '../types';

function PropertiesPanel() {
  const { selection, document, updateNode } = useEditorStore();

  // Get selected node
  const selectedNode = selection.length === 1 
    ? document.nodes.get(selection[0]) 
    : null;

  if (!selectedNode) {
    return (
      <div className="w-60 bg-figma-panel border-l border-figma-border flex flex-col">
        <div className="h-10 border-b border-figma-border flex items-center px-3">
          <span className="text-xs font-medium text-figma-text-secondary">Properties</span>
        </div>
        <div className="flex-1 flex items-center justify-center text-xs text-figma-text-secondary">
          Select an object
        </div>
      </div>
    );
  }

  const updateFillColor = (color: Color) => {
    const fills = [...selectedNode.fills];
    if (fills.length > 0) {
      fills[0] = { ...fills[0], color };
    }
    updateNode(selectedNode.id, { fills });
  };

  const updateStrokeColor = (color: Color) => {
    const strokes = [...selectedNode.strokes];
    if (strokes.length > 0) {
      strokes[0] = { ...strokes[0], color };
    }
    updateNode(selectedNode.id, { strokes });
  };

  const updateStrokeWidth = (width: number) => {
    const strokes = [...selectedNode.strokes];
    if (strokes.length > 0) {
      strokes[0] = { ...strokes[0], width };
    }
    updateNode(selectedNode.id, { strokes });
  };

  return (
    <div className="w-60 bg-figma-panel border-l border-figma-border flex flex-col">
      {/* Header */}
      <div className="h-10 border-b border-figma-border flex items-center px-3">
        <span className="text-xs font-medium text-figma-text-secondary">Properties</span>
      </div>

      <div className="flex-1 overflow-y-auto">
        {/* Position & Size */}
        <Section title="Transform">
          <div className="grid grid-cols-2 gap-2">
            <PropertyInput
              label="X"
              value={selectedNode.x}
              onChange={(v) => updateNode(selectedNode.id, { x: v })}
            />
            <PropertyInput
              label="Y"
              value={selectedNode.y}
              onChange={(v) => updateNode(selectedNode.id, { y: v })}
            />
            <PropertyInput
              label="W"
              value={selectedNode.width}
              onChange={(v) => updateNode(selectedNode.id, { width: v })}
            />
            <PropertyInput
              label="H"
              value={selectedNode.height}
              onChange={(v) => updateNode(selectedNode.id, { height: v })}
            />
          </div>
          <div className="grid grid-cols-2 gap-2 mt-2">
            <PropertyInput
              label="Rotation"
              value={selectedNode.rotation}
              onChange={(v) => updateNode(selectedNode.id, { rotation: v })}
              suffix="°"
            />
            {(selectedNode.type === 'rectangle' || selectedNode.type === 'frame') && (
              <PropertyInput
                label="Radius"
                value={selectedNode.cornerRadius || 0}
                onChange={(v) => updateNode(selectedNode.id, { cornerRadius: v })}
              />
            )}
          </div>
        </Section>

        {/* Fill */}
        {selectedNode.type !== 'line' && (
          <Section title="Fill">
            {selectedNode.fills.length > 0 && selectedNode.fills[0].color && (
              <div className="flex items-center gap-2">
                <ColorPicker
                  color={selectedNode.fills[0].color}
                  onChange={updateFillColor}
                />
                <div className="flex-1 text-xs">
                  {colorToHex(selectedNode.fills[0].color)}
                </div>
                <PropertyInput
                  label=""
                  value={Math.round(selectedNode.fills[0].opacity * 100)}
                  onChange={(v) => {
                    const fills = [...selectedNode.fills];
                    fills[0] = { ...fills[0], opacity: v / 100 };
                    updateNode(selectedNode.id, { fills });
                  }}
                  suffix="%"
                  className="w-16"
                />
              </div>
            )}
          </Section>
        )}

        {/* Stroke */}
        <Section title="Stroke">
          {selectedNode.strokes.length > 0 && (
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <ColorPicker
                  color={selectedNode.strokes[0].color}
                  onChange={updateStrokeColor}
                />
                <div className="flex-1 text-xs">
                  {colorToHex(selectedNode.strokes[0].color)}
                </div>
                <PropertyInput
                  label=""
                  value={selectedNode.strokes[0].width}
                  onChange={updateStrokeWidth}
                  className="w-16"
                />
              </div>
            </div>
          )}
        </Section>

        {/* Opacity */}
        <Section title="Layer">
          <div className="flex items-center gap-2">
            <span className="text-xs text-figma-text-secondary w-16">Opacity</span>
            <input
              type="range"
              min="0"
              max="100"
              value={selectedNode.opacity * 100}
              onChange={(e) => updateNode(selectedNode.id, { opacity: parseInt(e.target.value) / 100 })}
              className="flex-1 h-1 bg-figma-border rounded-lg appearance-none cursor-pointer"
            />
            <span className="text-xs w-10 text-right">{Math.round(selectedNode.opacity * 100)}%</span>
          </div>
        </Section>
      </div>
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="border-b border-figma-border">
      <div className="px-3 py-2">
        <h3 className="text-xs font-medium text-figma-text-secondary mb-2">{title}</h3>
        {children}
      </div>
    </div>
  );
}

function PropertyInput({
  label,
  value,
  onChange,
  suffix,
  className = '',
}: {
  label: string;
  value: number;
  onChange: (value: number) => void;
  suffix?: string;
  className?: string;
}) {
  return (
    <div className={`flex items-center gap-1 ${className}`}>
      {label && <span className="text-xs text-figma-text-secondary w-4">{label}</span>}
      <input
        type="number"
        value={Math.round(value * 100) / 100}
        onChange={(e) => onChange(parseFloat(e.target.value) || 0)}
        className="flex-1 bg-figma-bg border border-figma-border rounded px-2 py-1 text-xs text-figma-text focus:border-figma-accent focus:outline-none"
      />
      {suffix && <span className="text-xs text-figma-text-secondary">{suffix}</span>}
    </div>
  );
}

function ColorPicker({ color, onChange }: { color: Color; onChange: (color: Color) => void }) {
  const hex = colorToHex(color);

  return (
    <input
      type="color"
      value={hex}
      onChange={(e) => {
        const c = hexToColor(e.target.value);
        onChange({ ...c, a: color.a });
      }}
      className="w-6 h-6 rounded border border-figma-border cursor-pointer"
      style={{ backgroundColor: hex }}
    />
  );
}

function colorToHex(color: Color): string {
  const r = Math.round(color.r).toString(16).padStart(2, '0');
  const g = Math.round(color.g).toString(16).padStart(2, '0');
  const b = Math.round(color.b).toString(16).padStart(2, '0');
  return `#${r}${g}${b}`.toUpperCase();
}

function hexToColor(hex: string): Color {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  return { r, g, b, a: 1 };
}

export default PropertiesPanel;
