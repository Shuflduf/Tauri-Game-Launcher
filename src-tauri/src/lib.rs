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
    games: Option<Vec<Game>>,
}

impl SavedGames {
    pub fn games_mut(&mut self) -> &mut Vec<Game> {
        self.games.get_or_insert_with(Vec::new)
    }
}


fn read_save_data() -> Result<SavedGames, String> {
    let path = "save.toml";
    if !std::path::Path::new(path).exists() {
        fs::File::create(path).map_err(|err| err.to_string())?;
    }
    let toml_data = fs::read_to_string(path).map_err(|err| err.to_string())?;
    toml::from_str::<SavedGames>(&toml_data).map_err(|err| err.to_string())
}

fn write_save_data(data: SavedGames) -> Result<(), String> {
    let toml_data = toml::to_string(&data).map_err(|err| err.to_string())?;
    fs::write("save.toml", toml_data).map_err(|err| err.to_string())
}

#[tauri::command]
fn select_app() -> Result<String, String> {
    use rfd::FileDialog;

    #[cfg(target_os = "windows")]
    {
        if let Some(path) = FileDialog::new().add_filter("Executable", &["exe"]).pick_file() {
            Ok(path.display().to_string())
        } else {
            Err("No file selected".to_string())
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(path) = FileDialog::new().add_filter("Executable", &["AppImage", "sh"]).pick_file() {
            Ok(path.display().to_string())
        } else {
            Err("No file selected".to_string())
        }
    }

    // imma be fr this might not work, i dont have a mac mini to test this on... YET!
    #[cfg(target_os = "macos")]
    {
        if let Some(path) = FileDialog::new().add_filter("Application", &["app"]).pick_folder() {
            Ok(format!("open \"{}\"", path.display()))
        } else {
            Err("No application selected".to_string())
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        Err("This command is not supported on this OS.".to_string())
    }
}


#[tauri::command]
fn add_game(game: Game) -> Result<(), String> {
    let mut saved_games = read_save_data()?;
    saved_games.games_mut().push(game);
    write_save_data(saved_games)?;

    Ok(())
}


#[tauri::command]
fn edit_game(id: String, game: Game) -> Result<(), String> {
    let mut saved_games = read_save_data()?;
    for g in saved_games.games_mut().iter_mut() {
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
    Ok(read_save_data()?.games.unwrap_or_default())
}


#[tauri::command]
fn launch_game(command: String) -> Result<(), String> {
    println!("Running {command}");
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

    saved_games.games_mut().retain(|g| g != &game);
    write_save_data(saved_games)?;

    Ok(())
}


#[tauri::command(rename_all = "snake_case")]
fn move_game(game_index: i32, new_index: i32) -> Result<(), String> {
    let mut saved_games = read_save_data()?;
    let games = saved_games.games_mut();

    let game = games.remove(game_index as usize);
    games.insert(new_index as usize, game);

    write_save_data(saved_games)?;
    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            select_app,
            add_game,
            current_games,
            edit_game,
            launch_game,
            delete_game,
            move_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
