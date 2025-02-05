mod excel;
mod extract;

#[tauri::command]
fn hash_file(bytes: Vec<u8>) -> String {
    let hash = blake3::hash(&bytes);
    hash.to_string()
}

#[tauri::command]
fn core_nums() -> usize {
    num_cpus::get()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            hash_file,
            extract::extract,
            excel::create_write_excel,
            core_nums
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
