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
