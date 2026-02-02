use base64::{engine::general_purpose::STANDARD, Engine};

#[derive(Debug, thiserror::Error, specta::Type, serde::Serialize)]
pub enum Base64Error {
    #[error("Invalid Base64 input: {0}")]
    DecodeError(String),
    #[error("Invalid UTF-8 in decoded data")]
    Utf8Error,
}

#[tauri::command]
#[specta::specta]
pub fn base64_encode(input: String) -> String {
    STANDARD.encode(input.as_bytes())
}

#[tauri::command]
#[specta::specta]
pub fn base64_decode(input: String) -> Result<String, Base64Error> {
    let decoded = STANDARD
        .decode(input.trim())
        .map_err(|e| Base64Error::DecodeError(e.to_string()))?;

    String::from_utf8(decoded).map_err(|_| Base64Error::Utf8Error)
}
