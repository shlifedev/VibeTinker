/**
 * API client for backend communication
 */

const API_BASE = 'http://localhost:8000';

interface Base64Response {
  result: string;
}

export async function encodeBase64(text: string): Promise<string> {
  const res = await fetch(`${API_BASE}/tools/base64/encode`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ text }),
  });
  
  if (!res.ok) {
    const error = await res.json();
    throw new Error(error.detail || 'Encoding failed');
  }
  
  const data: Base64Response = await res.json();
  return data.result;
}

export async function decodeBase64(encoded: string): Promise<string> {
  const res = await fetch(`${API_BASE}/tools/base64/decode`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ encoded }),
  });
  
  if (!res.ok) {
    const error = await res.json();
    throw new Error(error.detail || 'Decoding failed');
  }
  
  const data: Base64Response = await res.json();
  return data.result;
}
