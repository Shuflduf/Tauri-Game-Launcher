<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import NewGame from "./NewGame.svelte";
  import type { Game } from "../lib";

  let name = $state("");
  let greetMsg = $state("");
  let games: Game[] = $state([]);

  onMount(async () => {
    // games = await invoke("current_games");
    await refresh();
  });

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  function tmpGame(): Game {
    let newGame: Game = {
      name: "Risk of Rain 2",
      launch_command: "flatpak run com.valvesoftware.Steam steam://run/632360",
    };
    return newGame;
  }

  async function refresh() {
    games = await invoke("current_games");
  }
</script>

{#each games as game}
  <p>{game.name}</p>
{/each}
<NewGame onAdded={refresh} />
