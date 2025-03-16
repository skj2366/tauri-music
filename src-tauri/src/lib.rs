// use lofty::tag::Accessor;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn read_metadata(file_path: String) -> Result<String, String> {
//     use lofty::file::TaggedFileExt;
//     let tagged_file = lofty::read_from_path(&file_path).map_err(|e| e.to_string())?;
//     let tags = tagged_file.primary_tag().ok_or("No tags found")?;
//     Ok(format!("Title: {}, Artist: {}", tags.title().unwrap_or(""), tags.artist().unwrap_or("")))
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
