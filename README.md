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
