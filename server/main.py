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


from routers import base64_tool

app.include_router(base64_tool.router)


@app.get("/health")
async def health_check():
    """Health check endpoint."""
    return {"status": "ok"}
