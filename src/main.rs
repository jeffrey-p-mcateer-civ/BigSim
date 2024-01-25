
// May not use this, I like doing CLI-first and then dynamically throwing away the console for UI mode
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


}

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}



