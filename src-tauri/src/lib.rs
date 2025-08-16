#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub name: String,
    pub launch_command: String,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tauri::command]
fn add_game(game: Game) {
    println!("{game:?}");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, add_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
