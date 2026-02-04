export type ToolType = 
  | 'select'
  | 'frame'
  | 'rectangle'
  | 'ellipse'
  | 'line'
  | 'pen'
  | 'text'
  | 'hand'
  | 'zoom';

export interface Point {
  x: number;
  y: number;
}

export interface Size {
  width: number;
  height: number;
}

export interface Bounds {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface Color {
  r: number;
  g: number;
  b: number;
  a: number;
}

export interface Fill {
  type: 'solid' | 'gradient' | 'image';
  color?: Color;
  visible: boolean;
  opacity: number;
}

export interface Stroke {
  color: Color;
  width: number;
  visible: boolean;
  opacity: number;
}

export interface DesignNode {
  id: string;
  type: NodeType;
  name: string;
  visible: boolean;
  locked: boolean;
  x: number;
  y: number;
  width: number;
  height: number;
  rotation: number;
  opacity: number;
  fills: Fill[];
  strokes: Stroke[];
  cornerRadius?: number;
  children?: string[];
  parentId?: string;
}

export type NodeType =
  | 'document'
  | 'page'
  | 'frame'
  | 'group'
  | 'rectangle'
  | 'ellipse'
  | 'line'
  | 'polygon'
  | 'star'
  | 'vector'
  | 'text'
  | 'image';

export interface Cursor {
  id: string;
  name: string;
  color: string;
  x: number;
  y: number;
}

export interface DocumentState {
  nodes: Map<string, DesignNode>;
  pages: string[];
  currentPageId: string;
  selection: string[];
}
