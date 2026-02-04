/**
 * Anatsui Multiplayer Server
 * 
 * WebSocket server for real-time collaboration.
 * Built with Bun for maximum performance.
 */

import { Server, ServerWebSocket } from 'bun';

// Types
interface ClientData {
  id: number;
  name: string;
  documentId: string;
  x: number;
  y: number;
}

interface Message {
  type: string;
  [key: string]: unknown;
}

// State
const clients = new Map<ServerWebSocket<ClientData>, ClientData>();
const documents = new Map<string, Set<ServerWebSocket<ClientData>>>();
let nextClientId = 1;

// Server
const server = Bun.serve<ClientData>({
  port: process.env.PORT || 8080,
  
  fetch(req, server) {
    const url = new URL(req.url);
    
    // Health check
    if (url.pathname === '/health') {
      return new Response('OK');
    }
    
    // WebSocket upgrade
    if (url.pathname === '/ws') {
      const documentId = url.searchParams.get('document') || 'default';
      const clientName = url.searchParams.get('name') || 'Anonymous';
      
      const upgraded = server.upgrade(req, {
        data: {
          id: nextClientId++,
          name: clientName,
          documentId,
          x: 0,
          y: 0,
        },
      });
      
      if (upgraded) {
        return undefined;
      }
      
      return new Response('WebSocket upgrade failed', { status: 400 });
    }
    
    // CORS preflight
    if (req.method === 'OPTIONS') {
      return new Response(null, {
        headers: {
          'Access-Control-Allow-Origin': '*',
          'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
          'Access-Control-Allow-Headers': 'Content-Type',
        },
      });
    }
    
    return new Response('Anatsui Multiplayer Server', {
      headers: {
        'Content-Type': 'text/plain',
      },
    });
  },
  
  websocket: {
    open(ws) {
      const data = ws.data;
      console.log(`Client ${data.id} (${data.name}) connected to document ${data.documentId}`);
      
      // Store client
      clients.set(ws, data);
      
      // Add to document room
      if (!documents.has(data.documentId)) {
        documents.set(data.documentId, new Set());
      }
      documents.get(data.documentId)!.add(ws);
      
      // Send join acknowledgement
      ws.send(JSON.stringify({
        type: 'JoinAck',
        client_id: data.id,
        document_state: '{}', // Empty for now, would contain serialized document
      }));
      
      // Notify others in the document
      broadcastToDocument(data.documentId, {
        type: 'UserJoined',
        client_id: data.id,
        name: data.name,
        color: getUserColor(data.id),
      }, ws);
      
      // Send existing users
      const room = documents.get(data.documentId);
      if (room) {
        const users = Array.from(room)
          .filter(c => c !== ws)
          .map(c => ({
            client_id: c.data.id,
            name: c.data.name,
            color: getUserColor(c.data.id),
            x: c.data.x,
            y: c.data.y,
          }));
        
        if (users.length > 0) {
          ws.send(JSON.stringify({
            type: 'ExistingUsers',
            users,
          }));
        }
      }
    },
    
    message(ws, message) {
      const data = ws.data;
      
      try {
        const msg: Message = JSON.parse(message.toString());
        
        switch (msg.type) {
          case 'CursorMove':
            data.x = msg.x as number;
            data.y = msg.y as number;
            broadcastToDocument(data.documentId, {
              type: 'CursorMove',
              client_id: data.id,
              x: data.x,
              y: data.y,
            }, ws);
            break;
            
          case 'PropertyChange':
          case 'CreateObject':
          case 'DeleteObject':
          case 'MoveObject':
            // Broadcast to all other clients in the document
            broadcastToDocument(data.documentId, {
              ...msg,
              client_id: data.id,
            }, ws);
            
            // Acknowledge the change
            ws.send(JSON.stringify({
              type: 'Ack',
              sequence: msg.sequence,
            }));
            break;
            
          case 'SelectionChange':
            broadcastToDocument(data.documentId, {
              type: 'SelectionChange',
              client_id: data.id,
              selected_ids: msg.selected_ids,
            }, ws);
            break;
            
          case 'Ping':
            ws.send(JSON.stringify({ type: 'Pong' }));
            break;
            
          default:
            console.log(`Unknown message type: ${msg.type}`);
        }
      } catch (error) {
        console.error('Failed to parse message:', error);
      }
    },
    
    close(ws) {
      const data = ws.data;
      console.log(`Client ${data.id} (${data.name}) disconnected`);
      
      // Remove from clients
      clients.delete(ws);
      
      // Remove from document room
      const room = documents.get(data.documentId);
      if (room) {
        room.delete(ws);
        
        // Clean up empty rooms
        if (room.size === 0) {
          documents.delete(data.documentId);
        } else {
          // Notify others
          broadcastToDocument(data.documentId, {
            type: 'UserLeft',
            client_id: data.id,
          });
        }
      }
    },
    
    error(ws, error) {
      console.error(`WebSocket error for client ${ws.data.id}:`, error);
    },
  },
});

function broadcastToDocument(
  documentId: string, 
  message: object, 
  exclude?: ServerWebSocket<ClientData>
) {
  const room = documents.get(documentId);
  if (!room) return;
  
  const json = JSON.stringify(message);
  for (const client of room) {
    if (client !== exclude) {
      client.send(json);
    }
  }
}

const USER_COLORS = [
  '#F24E1E', // Red
  '#A259FF', // Purple
  '#1ABCFE', // Blue
  '#0ACF83', // Green
  '#FF7262', // Orange
  '#FFC700', // Yellow
  '#00C2FF', // Cyan
  '#C7B9FF', // Lavender
];

function getUserColor(clientId: number): string {
  return USER_COLORS[clientId % USER_COLORS.length];
}

console.log(`🎨 Anatsui Multiplayer Server running on http://localhost:${server.port}`);
console.log(`   WebSocket endpoint: ws://localhost:${server.port}/ws`);
