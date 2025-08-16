#[derive(serde::Deserialize, serde::Serialize, Debug)]
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

#[tauri::command]
fn current_games() -> Vec<Game> {
    vec![Game {
        name: String::from("ror2"),
        launch_command: String::from("AAAA"),
    }]
    // println!("{game:?}");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, add_game, current_games])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
