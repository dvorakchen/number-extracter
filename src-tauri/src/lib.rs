mod extract;
mod excel;

#[tauri::command]
fn hash_file(bytes: Vec<u8>) -> String {
    let hash = blake3::hash(&bytes);
    hash.to_string()
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![hash_file, extract::extract])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
