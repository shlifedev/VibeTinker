#[tauri::command]
#[specta::specta]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
