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

fn read_save_data() -> Result<SavedGames, String> {
    let toml_data = fs::read_to_string("save.toml").map_err(|err| err.to_string())?;
    toml::from_str::<SavedGames>(&toml_data).map_err(|err| err.to_string())
}

fn write_save_data(data: SavedGames) -> Result<(), String> {
    let toml_data = toml::to_string(&data).map_err(|err| err.to_string())?;
    fs::write("save.toml", toml_data).map_err(|err| err.to_string())
}

#[tauri::command]
fn add_game(game: Game) -> Result<(), String> {
    println!("{game:?}");
    let mut saved_games = read_save_data()?;
    saved_games.games.push(game);
    write_save_data(saved_games)?;

    Ok(())
}

#[tauri::command]
fn edit_game(id: String, game: Game) -> Result<(), String> {
    println!("{id} -> {game:?}");
    let mut saved_games = read_save_data()?;
    for g in saved_games.games.iter_mut() {
        if g.name == id {
            *g = game;
            break;
        }
    }
    write_save_data(saved_games)?;

    Ok(())
}

#[tauri::command]
fn current_games() -> Result<Vec<Game>, String> {
    Ok(read_save_data()?.games)
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
fn delete_game(game: Game) -> Result<(), String> {
    let mut saved_games = read_save_data()?;

    saved_games.games.retain(|g| g != &game);
    write_save_data(saved_games)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            add_game,
            current_games,
            edit_game,
            launch_game,
            delete_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
