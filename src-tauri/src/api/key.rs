use keyring::Entry;

#[tauri::command]
pub fn save_api_key(key: String) -> Result<(), String> {
    let entry = Entry::new("nex-analytics", "api_key").map_err(|e| e.to_string())?;
    entry.set_password(&key).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_api_key() -> Result<String, String> {
    let entry = Entry::new("nex-analytics", "api_key").map_err(|e| e.to_string())?;
    entry.get_password().map_err(|e| e.to_string())
}