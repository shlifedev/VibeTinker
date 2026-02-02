# Productivity Tools Web Application Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a modular productivity tools web application with SvelteKit frontend and FastAPI backend, featuring an extensible router architecture for easy tool addition/removal.

**Architecture:** Monorepo with `/client` (SvelteKit + TypeScript) and `/server` (Python FastAPI). Tools are implemented as independent route modules that can be added/removed without touching core code. Client-side local storage for user preferences.

**Tech Stack:**
- Frontend: SvelteKit, TypeScript, Tailwind CSS (optional)
- Backend: Python 3.11+, FastAPI, uvicorn
- No database, no authentication

---

## Task 1: Initialize Project Structure

**Files:**
- Create: `client/` directory structure
- Create: `server/` directory structure
- Create: `README.md` at root

**Step 1: Create root README**

Create `README.md`:

```markdown
# LLM Dev Tools

A collection of productivity tools for developers.

## Structure

- `/client` - SvelteKit frontend (TypeScript)
- `/server` - FastAPI backend (Python)

## Quick Start

### Frontend
```bash
cd client
npm install
npm run dev
```

### Backend
```bash
cd server
pip install -r requirements.txt
uvicorn main:app --reload
```
```

**Step 2: Verify file created**

Run: `cat README.md`
Expected: Contents of the README displayed

**Step 3: Commit**

```bash
git add README.md
git commit -m "docs: add project README"
```

---

## Task 2: Setup SvelteKit Frontend

**Files:**
- Create: `client/package.json`
- Create: `client/svelte.config.js`
- Create: `client/tsconfig.json`
- Create: `client/vite.config.ts`
- Create: `client/src/app.html`
- Create: `client/src/app.d.ts`

**Step 1: Initialize SvelteKit project**

Run:
```bash
cd client
npm create svelte@latest . -- --template skeleton --types typescript --no-add-ons --no-install
```

Expected: SvelteKit project files created

**Step 2: Install dependencies**

Run:
```bash
cd client
npm install
```

Expected: `node_modules/` created, no errors

**Step 3: Verify dev server starts**

Run:
```bash
cd client
npm run dev -- --port 5173 &
sleep 3
curl -s http://localhost:5173 | head -20
pkill -f "vite"
```

Expected: HTML response from dev server

**Step 4: Commit**

```bash
git add client/
git commit -m "feat(client): initialize SvelteKit project with TypeScript"
```

---

## Task 3: Setup FastAPI Backend

**Files:**
- Create: `server/main.py`
- Create: `server/requirements.txt`
- Create: `server/routers/__init__.py`

**Step 1: Create requirements.txt**

Create `server/requirements.txt`:

```text
fastapi>=0.115.0
uvicorn[standard]>=0.30.0
pydantic>=2.0.0
```

**Step 2: Create main.py with CORS**

Create `server/main.py`:

```python
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI(
    title="LLM Dev Tools API",
    description="Backend API for productivity tools",
    version="0.1.0",
)

# CORS configuration for local development
origins = [
    "http://localhost:5173",
    "http://localhost:4173",
    "http://127.0.0.1:5173",
]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/health")
async def health_check():
    """Health check endpoint."""
    return {"status": "ok"}
```

**Step 3: Create routers package**

Create `server/routers/__init__.py`:

```python
"""
Tool routers package.

To add a new tool:
1. Create a new file in this directory (e.g., my_tool.py)
2. Define a router with: router = APIRouter(prefix="/my-tool", tags=["my-tool"])
3. Import and include in main.py: app.include_router(my_tool.router)
"""
```

**Step 4: Install dependencies and verify**

Run:
```bash
cd server
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

Expected: Dependencies installed successfully

**Step 5: Test server starts**

Run:
```bash
cd server
source .venv/bin/activate
uvicorn main:app --port 8000 &
sleep 2
curl -s http://localhost:8000/health
pkill -f "uvicorn"
```

Expected: `{"status":"ok"}`

**Step 6: Add .gitignore for server**

Create `server/.gitignore`:

```text
__pycache__/
*.py[cod]
.venv/
.env
*.egg-info/
dist/
build/
```

**Step 7: Commit**

```bash
git add server/
git commit -m "feat(server): initialize FastAPI backend with CORS"
```

---

## Task 4: Create Base64 Tool - Backend Router

**Files:**
- Create: `server/routers/base64_tool.py`
- Modify: `server/main.py` (add router import)

**Step 1: Create base64 router**

Create `server/routers/base64_tool.py`:

```python
"""Base64 encoding/decoding tool router."""

import base64
from fastapi import APIRouter, HTTPException
from pydantic import BaseModel


router = APIRouter(prefix="/tools/base64", tags=["base64"])


class EncodeRequest(BaseModel):
    """Request body for encoding."""
    text: str


class DecodeRequest(BaseModel):
    """Request body for decoding."""
    encoded: str


class Base64Response(BaseModel):
    """Response for base64 operations."""
    result: str


@router.post("/encode", response_model=Base64Response)
async def encode_base64(request: EncodeRequest) -> Base64Response:
    """Encode text to Base64."""
    try:
        encoded = base64.b64encode(request.text.encode("utf-8")).decode("utf-8")
        return Base64Response(result=encoded)
    except Exception as e:
        raise HTTPException(status_code=400, detail=f"Encoding failed: {str(e)}")


@router.post("/decode", response_model=Base64Response)
async def decode_base64(request: DecodeRequest) -> Base64Response:
    """Decode Base64 to text."""
    try:
        decoded = base64.b64decode(request.encoded).decode("utf-8")
        return Base64Response(result=decoded)
    except Exception as e:
        raise HTTPException(status_code=400, detail=f"Decoding failed: {str(e)}")
```

**Step 2: Register router in main.py**

Modify `server/main.py` - add after CORS middleware:

```python
from routers import base64_tool

# Register tool routers
app.include_router(base64_tool.router)
```

**Step 3: Test endpoints**

Run:
```bash
cd server
source .venv/bin/activate
uvicorn main:app --port 8000 &
sleep 2

# Test encode
curl -s -X POST http://localhost:8000/tools/base64/encode \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello World"}'

# Test decode
curl -s -X POST http://localhost:8000/tools/base64/decode \
  -H "Content-Type: application/json" \
  -d '{"encoded": "SGVsbG8gV29ybGQ="}'

pkill -f "uvicorn"
```

Expected:
- Encode: `{"result":"SGVsbG8gV29ybGQ="}`
- Decode: `{"result":"Hello World"}`

**Step 4: Commit**

```bash
git add server/routers/base64_tool.py server/main.py
git commit -m "feat(server): add Base64 encode/decode tool router"
```

---

## Task 5: Create Frontend Layout and Navigation

**Files:**
- Create: `client/src/routes/+layout.svelte`
- Create: `client/src/lib/config/tools.ts`
- Create: `client/src/lib/components/Sidebar.svelte`

**Step 1: Create tools configuration**

Create `client/src/lib/config/tools.ts`:

```typescript
/**
 * Tool registry - add/remove tools here
 * 
 * To add a new tool:
 * 1. Add entry to this array
 * 2. Create route at /src/routes/tools/[slug]/+page.svelte
 */

export interface Tool {
  slug: string;
  name: string;
  description: string;
  icon: string; // emoji or icon class
}

export const tools: Tool[] = [
  {
    slug: 'base64',
    name: 'Base64',
    description: 'Encode and decode Base64 strings',
    icon: '🔐',
  },
  // Add more tools here
];

export function getToolBySlug(slug: string): Tool | undefined {
  return tools.find((t) => t.slug === slug);
}
```

**Step 2: Create Sidebar component**

Create `client/src/lib/components/Sidebar.svelte`:

```svelte
<script lang="ts">
  import { page } from '$app/stores';
  import { tools } from '$lib/config/tools';
</script>

<aside class="sidebar">
  <div class="logo">
    <a href="/">Dev Tools</a>
  </div>
  
  <nav>
    <ul>
      {#each tools as tool}
        <li>
          <a 
            href="/tools/{tool.slug}" 
            class:active={$page.url.pathname === `/tools/${tool.slug}`}
          >
            <span class="icon">{tool.icon}</span>
            <span class="name">{tool.name}</span>
          </a>
        </li>
      {/each}
    </ul>
  </nav>
</aside>

<style>
  .sidebar {
    width: 240px;
    height: 100vh;
    background: #1a1a2e;
    color: #eee;
    display: flex;
    flex-direction: column;
    position: fixed;
    left: 0;
    top: 0;
  }

  .logo {
    padding: 1.5rem;
    font-size: 1.25rem;
    font-weight: bold;
    border-bottom: 1px solid #333;
  }

  .logo a {
    color: inherit;
    text-decoration: none;
  }

  nav ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  nav li a {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1.5rem;
    color: #aaa;
    text-decoration: none;
    transition: background 0.2s, color 0.2s;
  }

  nav li a:hover {
    background: #16213e;
    color: #fff;
  }

  nav li a.active {
    background: #0f3460;
    color: #fff;
  }

  .icon {
    font-size: 1.25rem;
  }
</style>
```

**Step 3: Create root layout**

Create `client/src/routes/+layout.svelte`:

```svelte
<script lang="ts">
  import Sidebar from '$lib/components/Sidebar.svelte';
  
  let { children } = $props();
</script>

<div class="app">
  <Sidebar />
  <main class="content">
    {@render children()}
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: #0a0a0f;
    color: #eee;
  }

  :global(*) {
    box-sizing: border-box;
  }

  .app {
    display: flex;
    min-height: 100vh;
  }

  .content {
    margin-left: 240px;
    flex: 1;
    padding: 2rem;
  }
</style>
```

**Step 4: Create home page**

Create `client/src/routes/+page.svelte`:

```svelte
<script lang="ts">
  import { tools } from '$lib/config/tools';
</script>

<svelte:head>
  <title>Dev Tools</title>
</svelte:head>

<h1>Developer Productivity Tools</h1>
<p>Select a tool from the sidebar to get started.</p>

<div class="tools-grid">
  {#each tools as tool}
    <a href="/tools/{tool.slug}" class="tool-card">
      <span class="icon">{tool.icon}</span>
      <h2>{tool.name}</h2>
      <p>{tool.description}</p>
    </a>
  {/each}
</div>

<style>
  h1 {
    margin-bottom: 0.5rem;
  }

  .tools-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1.5rem;
    margin-top: 2rem;
  }

  .tool-card {
    background: #16213e;
    border-radius: 8px;
    padding: 1.5rem;
    text-decoration: none;
    color: inherit;
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .tool-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .tool-card .icon {
    font-size: 2rem;
  }

  .tool-card h2 {
    margin: 0.5rem 0;
    font-size: 1.25rem;
  }

  .tool-card p {
    margin: 0;
    color: #aaa;
    font-size: 0.9rem;
  }
</style>
```

**Step 5: Verify layout renders**

Run:
```bash
cd client
npm run dev -- --port 5173 &
sleep 3
curl -s http://localhost:5173 | grep -o "Dev Tools"
pkill -f "vite"
```

Expected: "Dev Tools" found in response

**Step 6: Commit**

```bash
git add client/src/
git commit -m "feat(client): add layout, sidebar, and tools registry"
```

---

## Task 6: Create Base64 Tool - Frontend Page

**Files:**
- Create: `client/src/routes/tools/base64/+page.svelte`
- Create: `client/src/lib/api.ts`

**Step 1: Create API client**

Create `client/src/lib/api.ts`:

```typescript
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
```

**Step 2: Create Base64 tool page**

Create `client/src/routes/tools/base64/+page.svelte`:

```svelte
<script lang="ts">
  import { encodeBase64, decodeBase64 } from '$lib/api';
  
  let input = $state('');
  let output = $state('');
  let error = $state('');
  let mode = $state<'encode' | 'decode'>('encode');
  let loading = $state(false);
  
  async function handleConvert() {
    if (!input.trim()) {
      error = 'Please enter some text';
      return;
    }
    
    error = '';
    loading = true;
    
    try {
      if (mode === 'encode') {
        output = await encodeBase64(input);
      } else {
        output = await decodeBase64(input);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'An error occurred';
      output = '';
    } finally {
      loading = false;
    }
  }
  
  function handleSwap() {
    const temp = input;
    input = output;
    output = temp;
    mode = mode === 'encode' ? 'decode' : 'encode';
  }
  
  function handleCopy() {
    if (output) {
      navigator.clipboard.writeText(output);
    }
  }
  
  function handleClear() {
    input = '';
    output = '';
    error = '';
  }
</script>

<svelte:head>
  <title>Base64 Encoder/Decoder - Dev Tools</title>
</svelte:head>

<div class="tool-page">
  <h1>Base64 Encoder/Decoder</h1>
  
  <div class="mode-selector">
    <button 
      class:active={mode === 'encode'} 
      onclick={() => mode = 'encode'}
    >
      Encode
    </button>
    <button 
      class:active={mode === 'decode'} 
      onclick={() => mode = 'decode'}
    >
      Decode
    </button>
  </div>
  
  <div class="converter">
    <div class="input-section">
      <label for="input">
        {mode === 'encode' ? 'Plain Text' : 'Base64 String'}
      </label>
      <textarea 
        id="input" 
        bind:value={input} 
        placeholder={mode === 'encode' ? 'Enter text to encode...' : 'Enter Base64 to decode...'}
        rows="8"
      ></textarea>
    </div>
    
    <div class="actions">
      <button class="primary" onclick={handleConvert} disabled={loading}>
        {loading ? 'Converting...' : (mode === 'encode' ? 'Encode' : 'Decode')}
      </button>
      <button onclick={handleSwap} disabled={!output}>Swap</button>
      <button onclick={handleClear}>Clear</button>
    </div>
    
    {#if error}
      <div class="error">{error}</div>
    {/if}
    
    <div class="output-section">
      <label for="output">
        {mode === 'encode' ? 'Base64 Result' : 'Decoded Text'}
      </label>
      <textarea 
        id="output" 
        value={output} 
        readonly 
        placeholder="Result will appear here..."
        rows="8"
      ></textarea>
      {#if output}
        <button class="copy-btn" onclick={handleCopy}>Copy</button>
      {/if}
    </div>
  </div>
</div>

<style>
  .tool-page {
    max-width: 800px;
  }

  h1 {
    margin-bottom: 1.5rem;
  }

  .mode-selector {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
  }

  .mode-selector button {
    padding: 0.5rem 1.5rem;
    border: 1px solid #333;
    background: transparent;
    color: #aaa;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .mode-selector button.active {
    background: #0f3460;
    border-color: #0f3460;
    color: #fff;
  }

  .converter {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .input-section,
  .output-section {
    position: relative;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    color: #aaa;
    font-size: 0.9rem;
  }

  textarea {
    width: 100%;
    padding: 1rem;
    background: #16213e;
    border: 1px solid #333;
    border-radius: 4px;
    color: #eee;
    font-family: 'SF Mono', Monaco, monospace;
    font-size: 0.9rem;
    resize: vertical;
  }

  textarea:focus {
    outline: none;
    border-color: #0f3460;
  }

  textarea[readonly] {
    background: #0d1b2a;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  button {
    padding: 0.75rem 1.5rem;
    border: 1px solid #333;
    background: #16213e;
    color: #eee;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
  }

  button:hover:not(:disabled) {
    background: #1a2744;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  button.primary {
    background: #0f3460;
    border-color: #0f3460;
  }

  button.primary:hover:not(:disabled) {
    background: #1a4a8a;
  }

  .copy-btn {
    position: absolute;
    top: 2.5rem;
    right: 0.5rem;
    padding: 0.25rem 0.75rem;
    font-size: 0.8rem;
  }

  .error {
    padding: 0.75rem 1rem;
    background: rgba(255, 0, 0, 0.1);
    border: 1px solid rgba(255, 0, 0, 0.3);
    border-radius: 4px;
    color: #ff6b6b;
  }
</style>
```

**Step 3: Commit**

```bash
git add client/src/lib/api.ts client/src/routes/tools/
git commit -m "feat(client): add Base64 tool page with API integration"
```

---

## Task 7: Add Local Storage for Preferences

**Files:**
- Create: `client/src/lib/stores/preferences.ts`
- Modify: `client/src/routes/tools/base64/+page.svelte`

**Step 1: Create preferences store**

Create `client/src/lib/stores/preferences.ts`:

```typescript
/**
 * Local storage backed preferences store
 */

import { browser } from '$app/environment';

const STORAGE_KEY = 'devtools_preferences';

interface Preferences {
  lastTool?: string;
  base64Mode?: 'encode' | 'decode';
}

function loadPreferences(): Preferences {
  if (!browser) return {};
  
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    return stored ? JSON.parse(stored) : {};
  } catch {
    return {};
  }
}

function savePreferences(prefs: Preferences): void {
  if (!browser) return;
  
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(prefs));
  } catch {
    // Ignore storage errors
  }
}

export function getPreference<K extends keyof Preferences>(key: K): Preferences[K] {
  return loadPreferences()[key];
}

export function setPreference<K extends keyof Preferences>(
  key: K,
  value: Preferences[K]
): void {
  const prefs = loadPreferences();
  prefs[key] = value;
  savePreferences(prefs);
}
```

**Step 2: Update Base64 page to use preferences**

Modify `client/src/routes/tools/base64/+page.svelte` - update script section:

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { encodeBase64, decodeBase64 } from '$lib/api';
  import { getPreference, setPreference } from '$lib/stores/preferences';
  
  let input = $state('');
  let output = $state('');
  let error = $state('');
  let mode = $state<'encode' | 'decode'>('encode');
  let loading = $state(false);
  
  onMount(() => {
    const savedMode = getPreference('base64Mode');
    if (savedMode) {
      mode = savedMode;
    }
  });
  
  $effect(() => {
    setPreference('base64Mode', mode);
  });
  
  // ... rest of the functions remain the same
```

**Step 3: Commit**

```bash
git add client/src/lib/stores/
git add client/src/routes/tools/base64/+page.svelte
git commit -m "feat(client): add local storage preferences"
```

---

## Task 8: Add Development Scripts

**Files:**
- Create: `scripts/dev.sh`
- Modify: `README.md`

**Step 1: Create development script**

Create `scripts/dev.sh`:

```bash
#!/bin/bash

# Start both frontend and backend for development

cleanup() {
    echo "Stopping services..."
    pkill -f "uvicorn main:app" 2>/dev/null
    pkill -f "vite" 2>/dev/null
    exit 0
}

trap cleanup SIGINT SIGTERM

echo "Starting FastAPI backend..."
cd server
source .venv/bin/activate 2>/dev/null || python -m venv .venv && source .venv/bin/activate
pip install -q -r requirements.txt
uvicorn main:app --reload --port 8000 &
BACKEND_PID=$!

echo "Starting SvelteKit frontend..."
cd ../client
npm install --silent
npm run dev -- --port 5173 &
FRONTEND_PID=$!

echo ""
echo "==================================="
echo "Development servers running:"
echo "  Frontend: http://localhost:5173"
echo "  Backend:  http://localhost:8000"
echo "  API Docs: http://localhost:8000/docs"
echo "==================================="
echo ""
echo "Press Ctrl+C to stop all services"

wait
```

**Step 2: Make script executable**

Run: `chmod +x scripts/dev.sh`

**Step 3: Update README**

Update `README.md` with full instructions:

```markdown
# LLM Dev Tools

A collection of productivity tools for developers.

## Structure

- `/client` - SvelteKit frontend (TypeScript)
- `/server` - FastAPI backend (Python)
- `/scripts` - Development utilities

## Quick Start

### Option 1: Run both services
```bash
./scripts/dev.sh
```

### Option 2: Run separately

**Frontend:**
```bash
cd client
npm install
npm run dev
```

**Backend:**
```bash
cd server
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
uvicorn main:app --reload
```

## Adding New Tools

### Backend (FastAPI)

1. Create `server/routers/my_tool.py`:
```python
from fastapi import APIRouter

router = APIRouter(prefix="/tools/my-tool", tags=["my-tool"])

@router.post("/action")
async def my_action():
    return {"result": "ok"}
```

2. Register in `server/main.py`:
```python
from routers import my_tool
app.include_router(my_tool.router)
```

### Frontend (SvelteKit)

1. Add tool to `client/src/lib/config/tools.ts`:
```typescript
{
  slug: 'my-tool',
  name: 'My Tool',
  description: 'Does something useful',
  icon: '🔧',
}
```

2. Create page at `client/src/routes/tools/my-tool/+page.svelte`

## API Documentation

When backend is running: http://localhost:8000/docs
```

**Step 4: Commit**

```bash
git add scripts/ README.md
git commit -m "docs: add dev script and tool addition guide"
```

---

## Task 9: Add .gitignore Files

**Files:**
- Create: `.gitignore` (root)
- Verify: `client/.gitignore` exists
- Verify: `server/.gitignore` exists

**Step 1: Create root .gitignore**

Create `.gitignore`:

```text
# OS
.DS_Store
Thumbs.db

# IDE
.idea/
.vscode/
*.swp
*.swo

# Logs
*.log
npm-debug.log*
```

**Step 2: Verify client .gitignore (created by SvelteKit)**

Run: `cat client/.gitignore`
Expected: Should contain node_modules, .svelte-kit, etc.

**Step 3: Commit**

```bash
git add .gitignore
git commit -m "chore: add root gitignore"
```

---

## Task 10: Final Integration Test

**Files:** None (verification only)

**Step 1: Start backend**

Run:
```bash
cd server
source .venv/bin/activate
uvicorn main:app --port 8000 &
sleep 2
```

**Step 2: Test API endpoints**

Run:
```bash
# Health check
curl -s http://localhost:8000/health

# Encode test
curl -s -X POST http://localhost:8000/tools/base64/encode \
  -H "Content-Type: application/json" \
  -d '{"text": "Integration Test"}'

# Decode test  
curl -s -X POST http://localhost:8000/tools/base64/decode \
  -H "Content-Type: application/json" \
  -d '{"encoded": "SW50ZWdyYXRpb24gVGVzdA=="}'
```

Expected:
- `{"status":"ok"}`
- `{"result":"SW50ZWdyYXRpb24gVGVzdA=="}`
- `{"result":"Integration Test"}`

**Step 3: Start frontend and verify**

Run:
```bash
cd client
npm run build
npm run preview -- --port 4173 &
sleep 3
curl -s http://localhost:4173 | grep -o "Dev Tools"
```

Expected: "Dev Tools" found

**Step 4: Cleanup and final commit**

Run:
```bash
pkill -f "uvicorn"
pkill -f "vite"
git status
```

If clean, done. Otherwise:
```bash
git add -A
git commit -m "chore: final cleanup"
```

---

## Summary

| Task | Component | Description |
|------|-----------|-------------|
| 1 | Root | Project README |
| 2 | Client | SvelteKit initialization |
| 3 | Server | FastAPI setup with CORS |
| 4 | Server | Base64 tool router |
| 5 | Client | Layout and navigation |
| 6 | Client | Base64 tool page |
| 7 | Client | Local storage preferences |
| 8 | Scripts | Development utilities |
| 9 | Root | Gitignore files |
| 10 | All | Integration verification |

**Total estimated time:** 45-60 minutes

**Next steps after completion:**
1. Add more tools by following the pattern in README
2. Consider adding Tailwind CSS for better styling
3. Deploy using SvelteKit adapter-node + any Python hosting
