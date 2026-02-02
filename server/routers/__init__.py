"""
Tool routers package.

To add a new tool:
1. Create a new file in this directory (e.g., my_tool.py)
2. Define a router with: router = APIRouter(prefix="/my-tool", tags=["my-tool"])
3. Import and include in main.py: app.include_router(my_tool.router)
"""
