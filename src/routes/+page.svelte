<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  type Game = {
    name: string;
    launchCommand: string;
  };

  let name = $state("");
  let greetMsg = $state("");
  let games: Game[] = $state([]);

  onMount(async () => {
    // games.push(tmpGame());
    games = await invoke("current_games");
  });

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  async function addGame(event: Event) {
    event.preventDefault();
    console.log({ game: tmpGame() });
    await invoke("add_game", { game: tmpGame() });
  }

  function tmpGame(): Game {
    let newGame: Game = {
      name: "Risk of Rain 2",
      launchCommand: "flatpak run com.valvesoftware.Steam steam://run/632360",
    };
    return newGame;
  }
</script>

{#each games as game}
  <p>{game.name}</p>
{/each}
<button onclick={addGame}>Add Risky Rain</button>
