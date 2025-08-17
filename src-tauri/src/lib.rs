use std::{fs, process::Command};

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct Game {
    pub name: String,
    pub launch_command: String,
    pub description: String,
    pub bg_color: String,
    pub text_color: String,
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
fn add_game(game: Game) -> Result<(), String> {
    println!("{game:?}");
    let toml_data = fs::read_to_string("save.toml").unwrap();
    let mut saved_games: SavedGames = toml::from_str(&toml_data).unwrap();
    saved_games.games.push(game);
    let toml_data = toml::to_string(&saved_games).unwrap();
    fs::write("save.toml", toml_data).map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
fn edit_game(id: String, game: Game) {
    println!("{id} -> {game:?}");
    let toml_data = fs::read_to_string("save.toml").unwrap();
    let mut saved_games: SavedGames = toml::from_str(&toml_data).unwrap();
    for g in saved_games.games.iter_mut() {
        if g.name == id {
            *g = game;
            break;
        }
    }
    let toml_data = toml::to_string(&saved_games).unwrap();
    let _res = fs::write("save.toml", toml_data);
}

#[tauri::command]
fn current_games() -> Vec<Game> {
    let toml_data = fs::read_to_string("save.toml").unwrap();
    let saved_games: SavedGames = toml::from_str(&toml_data).unwrap();
    saved_games.games
}

#[tauri::command]
fn launch_game(command: String) -> Result<(), String> {
    println!("Executing {command}");

    let mut parts = command.split_whitespace();
    let program = parts.next().ok_or("Empty command")?;
    let args: Vec<&str> = parts.collect();
    Command::new(program)
        .args(args)
        .spawn()
        .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
fn delete_game(game: Game) {
    let toml_data = fs::read_to_string("save.toml").unwrap();
    let mut saved_games: SavedGames = toml::from_str(&toml_data).unwrap();
    saved_games.games.retain(|g| g != &game);
    let toml_data = toml::to_string(&saved_games).unwrap();
    let _res = fs::write("save.toml", toml_data);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            add_game,
            current_games,
            edit_game,
            launch_game,
            delete_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
