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
