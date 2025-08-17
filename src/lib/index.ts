// Seperate file so that every Svelte component has access to it easily
// This has to have the same structure as the Game struct in Rust,
// and unless you want to do rename_all stuff, it's the simplest way.
// https://v2.tauri.app/develop/calling-rust/#passing-arguments
export type Game = {
  name: string;
  launch_command: string;
  description: string;
  bg_color: string;
  text_color: string;
};

