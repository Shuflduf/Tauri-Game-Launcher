use std::fs;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Game {
    pub name: String,
    pub launch_command: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct SavedGames {
    games: Vec<Game>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tauri::command]
fn add_game(game: Game) {
    println!("{game:?}");
    let toml_data = fs::read_to_string("save.toml").unwrap();
    let mut saved_games: SavedGames = toml::from_str(&toml_data).unwrap();
    saved_games.games.push(game);
    let toml_data = toml::to_string(&saved_games).unwrap();
    let _ = fs::write("save.toml", toml_data).unwrap();
}

#[tauri::command]
fn current_games() -> Vec<Game> {
    let toml_data = fs::read_to_string("save.toml").unwrap();
    let saved_games: SavedGames = toml::from_str(&toml_data).unwrap();
    saved_games.games
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, add_game, current_games])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
